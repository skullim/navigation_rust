use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

use crate::communication_msgs::ImuMsg;
use crate::communication_msgs::LocalizationMsg;

///Once the Storage is created the object might still not be created and therefore the type is wrapped into Option
#[derive(Clone)]
pub struct Storage<T>(Option<T>);

impl<T> Storage<T> {
    pub fn get(&self) -> Option<&T> {
        self.0.as_ref()
    }

    pub fn set(&mut self, new: T) {
        self.0 = Some(new);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputMsgType {
    Localization,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProtocolType {
    MQTT,
    gRPC,
    ROS,
    TCP,
}

#[derive(Debug)]
pub struct MsgWithMeta {
    pub protocol: ProtocolType,
    pub msg_type: InputMsgType,
    pub payload: String, //todo change to &str if feasible
}

///@todo error handling -> Result<T>
pub trait AdaptToLocMsg {
    fn adapt(&self, from: MsgWithMeta) -> LocalizationMsg;
}

pub trait AdaptToImuMsg {
    fn adapt(&self, from: MsgWithMeta) -> ImuMsg;
}

pub struct MqttAdapter {}

pub struct TcpAdapter {}

pub trait Publish<T> {
    fn publish(&mut self, input: T);

    fn get_input_msg_type(&self) -> InputMsgType;
}

pub trait Subscribe<T: Send> {
    fn subscribe(&mut self, message: T);
}

pub struct SubscriberRegistry<'a, T> {
    subscribers: Vec<&'a mut dyn Subscribe<T>>,
}

impl<'a, T: Clone + Send> SubscriberRegistry<'a, T> {
    pub fn new() -> Self {
        Self {
            subscribers: Vec::new(),
        }
    }

    pub fn register(&mut self, subscriber: &'a mut dyn Subscribe<T>) {
        self.subscribers.push(subscriber);
    }

    pub fn notify(&mut self, msg: T) {
        for subscriber in &mut self.subscribers {
            subscriber.subscribe(msg.clone());
        }
    }
}

pub struct LocMsgPublisher<'a> {
    adapter: Box<dyn AdaptToLocMsg + Send>,
    subscriber_registry: SubscriberRegistry<'a, LocalizationMsg>,
}

impl<'a> Publish<MsgWithMeta> for LocMsgPublisher<'a> {
    fn publish(&mut self, input: MsgWithMeta) {
        let adapted_msg = self.adapter.adapt(input);
        self.subscriber_registry.notify(adapted_msg);
    }

    fn get_input_msg_type(&self) -> InputMsgType {
        InputMsgType::Localization
    }
}

impl<'a> LocMsgPublisher<'a> {
    pub fn new(adapter: Box<dyn AdaptToLocMsg + Send>) -> Self {
        Self {
            adapter,
            subscriber_registry: SubscriberRegistry::<'a, LocalizationMsg>::new(),
        }
    }

    pub fn register(&mut self, subscriber: &'a mut dyn Subscribe<LocalizationMsg>) {
        self.subscriber_registry.register(subscriber);
    }
}
///Constrained to one publisher per one data type for now
pub struct CommunicationHandler {
    sender: Sender<MsgWithMeta>,
}

impl CommunicationHandler {
    pub fn new(publishers: Vec<Box<dyn Publish<MsgWithMeta> + Send>>) -> Self {
        let (sender, receiver): (Sender<MsgWithMeta>, Receiver<MsgWithMeta>) = channel();
        let mut publishers_map: HashMap<InputMsgType, Box<dyn Publish<MsgWithMeta> + Send>> =
            HashMap::new();
        for publisher in publishers {
            publishers_map.insert(publisher.as_ref().get_input_msg_type(), publisher);
        }

        thread::spawn(move || {
            for new_msg in receiver {
                println!("Received {:?}", new_msg);
                let publisher = publishers_map
                    .get_mut(&new_msg.msg_type)
                    .expect(&format!("No publisher/adapter that handles {:?}", new_msg));
                publisher.publish(new_msg);
            }
        });
        Self { sender }
    }

    pub fn send_new_msg(&self, msg_with_meta: MsgWithMeta) {
        self.sender.send(msg_with_meta).unwrap();
    }
}

// Define integration tests
#[cfg(test)]
mod tests {

    use crate::geometry::{Angle, Pose};

    use super::*;

    pub struct MyStorage {
        pub imu_msg: Arc<Mutex<Storage<ImuMsg>>>,
        pub localization_msg: Arc<Mutex<Storage<LocalizationMsg>>>,
    }

    impl Subscribe<ImuMsg> for MyStorage {
        fn subscribe(&mut self, message: ImuMsg) {
            println!("Received new Imu msg");
            let mut imu_msg = self.imu_msg.lock().unwrap();
            imu_msg.set(message);
        }
    }

    impl Subscribe<LocalizationMsg> for MyStorage {
        fn subscribe(&mut self, message: LocalizationMsg) {
            println!("Received new Localization msg");
            let mut localization_msg = self.localization_msg.lock().unwrap();
            localization_msg.set(message);
        }
    }

    pub struct FakeAdapter {}

    impl AdaptToLocMsg for FakeAdapter {
        fn adapt(&self, from: MsgWithMeta) -> LocalizationMsg {
            LocalizationMsg {
                pose: Pose {
                    x: 2.0,
                    y: 5.0,
                    theta: Angle::new(1.0, crate::geometry::AngleWrapping::PlusMinusPi),
                },
            }
        }
    }

    pub struct NavigationComponent {}

    impl NavigationComponent {
        pub fn tick(&self, storage_view: MyStorage) {
            let loc_msg = storage_view.localization_msg;
            let imu_msg = storage_view.imu_msg;
        }
    }

    #[test]
    fn test_communication_handler() {
        let my_storage = Arc::new(Mutex::new(MyStorage {
            imu_msg: Arc::new(Mutex::new(Storage(None))),
            localization_msg: Arc::new(Mutex::new(Storage(None))),
        }));
        let fake_adapter = Box::new(FakeAdapter {});
        let mut loc_msg_pub = Box::new(LocMsgPublisher::new(fake_adapter));
        {
            let mut my_storage_val = my_storage.lock().unwrap();
            loc_msg_pub.register(&mut *my_storage_val);
        }

        let communication_handler = CommunicationHandler::new(vec![loc_msg_pub]);

        communication_handler.send_new_msg(MsgWithMeta {
            protocol: ProtocolType::MQTT,
            msg_type: InputMsgType::Localization,
            payload: "Hello".to_string(),
        });
    }
}
