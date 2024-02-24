use std::sync::mpsc::{channel, Sender};
use std::thread;

use crate::communication_msgs::LocalizationMsg;
use crate::communication_msgs::ImuMsg;

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
pub enum InputMsgType
{
    Localization
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProtocolType
{
    MQTT,
    gRPC,
    ROS,
    TCP
}

#[derive(Debug)]
pub struct MsgWithMeta
{
    pub protocol: ProtocolType,
    pub msg_type: InputMsgType,
    pub payload: String //todo change to &str if feasible
}

///@todo Adrian error handling -> Result<T>
pub trait AdaptToLocMsg
{
    fn adapt(&self, from: MsgWithMeta) -> LocalizationMsg;
}

pub trait AdaptToImuMsg
{
    fn adapt(&self, from: MsgWithMeta) -> ImuMsg;
}

pub struct MqttAdapter
{

}

pub struct TcpAdapter
{

}

pub trait Publish<T>
{
    fn publish(&mut self, input: T);
}

pub trait Subscribe<T>
{
    fn subscribe(&mut self, message: &T);
}

pub struct SubscriberRegistry<'a, T>
{
    subscribers :  Vec<&'a mut dyn Subscribe<T>>,
}

impl <'a, T> SubscriberRegistry<'a, T>
{

    pub fn new() -> Self
    {
        Self{subscribers: Vec::new()}
    }

    pub fn register(&mut self, subscriber: &'a mut dyn Subscribe<T>)
    {
        self.subscribers.push(subscriber);
    }

    pub fn notify(&mut self, msg: T)
    {
        for subscriber in &mut self.subscribers
        {
            subscriber.subscribe(&msg);
        }
    }
}

pub struct LocMsgPublisher<'a>
{
    adapter : Box<dyn AdaptToLocMsg>,
    subscriber_registry : SubscriberRegistry<'a, LocalizationMsg>
}

impl<'a> Publish<MsgWithMeta> for LocMsgPublisher<'a>
{
    fn publish(&mut self, input: MsgWithMeta) {
        let adapted_msg = self.adapter.adapt(input);
        self.subscriber_registry.notify(adapted_msg);
    }
}

impl<'a> LocMsgPublisher<'a>
{
    pub fn new(adapter: Box<dyn AdaptToLocMsg>) -> Self
    {
        Self {adapter, subscriber_registry: SubscriberRegistry::<'a, LocalizationMsg>::new()}
    }

    pub fn register(&mut self, subscriber: &'a mut dyn Subscribe<LocalizationMsg>)
    {
        self.subscriber_registry.register(subscriber);
    }
}

pub struct CommunicationHandler
{
    sender: Sender<MsgWithMeta>,
    publishers: Vec<Box<dyn Publish<MsgWithMeta>>>,
}

impl CommunicationHandler
{
    pub fn new() -> Self
    {
        let (sender, receiver) = channel();
        thread::spawn(move || {for new_msg in receiver {println!("Received {:?}", new_msg)}});
        Self{sender, publishers: Vec::new()}
    }

    pub fn register_publisher(&mut self, publisher: Box<dyn Publish<MsgWithMeta>>)
    {
        self.publishers.push(publisher);
    }

    pub fn get_new_sender(&self) -> Sender<MsgWithMeta>
    {
        self.sender.clone()
    }
}


// Define integration tests
#[cfg(test)]
mod tests {

    use crate::geometry::{Angle, Pose};

    use super::*;

    #[derive(Clone)]
    pub struct MyStorage
    {
        pub imu_msg : Storage<ImuMsg>,
        pub localization_msg : Storage<LocalizationMsg>,

    }

    impl Subscribe<ImuMsg> for MyStorage
    {
        fn subscribe(&mut self, message : &ImuMsg)
        {
            println!("Received new Imu msg");
            self.imu_msg.set(message.clone())
        }
    }

    impl Subscribe<LocalizationMsg> for MyStorage
    {
        fn subscribe(&mut self, message : &LocalizationMsg)
        {
            println!("Received new Localization msg");
            self.localization_msg.set(message.clone());
        }
    }

    pub struct FakeAdapter
    {

    }

    impl AdaptToLocMsg for FakeAdapter
    {
        fn adapt(&self, from: MsgWithMeta) -> LocalizationMsg {
            LocalizationMsg{pose: Pose{ x: 2.0, y: 5.0, theta: Angle::new(1.0, crate::geometry::AngleWrapping::PlusMinusPi) }}
        }
    }

    pub struct NavigationComponent
    {
    }

    impl NavigationComponent
    {
        pub fn tick(&self, storage_view : MyStorage)
        {
            let loc_msg = storage_view.localization_msg;
            let imu_msg = storage_view.imu_msg;
        }
    }



    #[test]
    fn test_multi_subscriber() {

        let nav_component = NavigationComponent{};

        let mut my_storage = Box::new(MyStorage{imu_msg:Storage(None), localization_msg:Storage(None)});
        let fake_adapter = Box::new(FakeAdapter{});
        let mut loc_msg_pub = LocMsgPublisher::new(fake_adapter);

        nav_component.tick((*my_storage).clone());

        loc_msg_pub.register(&mut*my_storage);
        loc_msg_pub.publish(MsgWithMeta {
            protocol: ProtocolType::MQTT,
            msg_type: InputMsgType::Localization,
            payload: "Hello".to_string(),
        });

        nav_component.tick(*my_storage);
    }
}
