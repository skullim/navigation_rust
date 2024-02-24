use std::borrow::Borrow;
use std::borrow::BorrowMut;
use std::collections::HashMap;

use crate::communication_msgs::LocalizationMsg;
use crate::communication_msgs::ImuMsg;
use crate::communication_msgs::SupportedControlTrait;

///Once the Storage is created the object might still not be created and therefore the type is wrapped into Option
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

pub struct MsgWithMeta
{
    pub protocol: ProtocolType,
    pub msg_type: InputMsgType,
    pub payload: String //todo change to &str if feasible
}

// pub trait DataTypeConverter {
//     fn convert(&self, input : MsgWithMeta) -> ConvertedMsg;
// }

// pub enum ConvertedMsg
// {
//     Localization(LocalizationMsg)
// }

// pub struct MsgReceiver
// {
//     converters_map : HashMap<(ProtocolType, InputMsgType), Box<dyn DataTypeConverter>>
// }

// impl MsgReceiver
// {
//     pub fn receive(&self, msg_with_meta: MsgWithMeta)
//     {
//         let key = (msg_with_meta.protocol, msg_with_meta.msg_type);
//         match self.converters_map.get(&key) {
//             Some(converter) => {
//                 let value = converter.convert(msg_with_meta);
//             }
//             None => {print!("Converter not found!")}
//         }
//     }
// }

pub struct OutputData<T: SupportedControlTrait>
{
    pub ctrl_msg: T
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

pub struct SubscriberRegistry<T>
{
    subscribers : Vec<Box<dyn Subscribe<T>>>
}

impl <T> SubscriberRegistry<T>
{
    pub fn register(&mut self, subscriber: Box<dyn Subscribe<T>>)
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

pub struct LocMsgPublisher
{
    adapter : Box<dyn AdaptToLocMsg>,
    subscriber_registry : SubscriberRegistry<LocalizationMsg>
}

impl Publish<MsgWithMeta> for LocMsgPublisher
{
    fn publish(&mut self, input: MsgWithMeta) {
        let adapted_msg = self.adapter.adapt(input);
        self.subscriber_registry.notify(adapted_msg);
    }
}

impl LocMsgPublisher
{
    pub fn register(&mut self, subscriber: Box<dyn Subscribe<LocalizationMsg>>)
    {
        self.subscriber_registry.register(subscriber);
    }
}


// Define integration tests
#[cfg(test)]
mod tests {

    use crate::geometry::{Angle, Pose};

    use super::*;

    #[derive(Clone)]
    pub struct DummyMsg
    {
        val : i32
    }

    pub struct MyStorage
    {
        pub dummy_msg : Storage<DummyMsg>,
        pub imu_msg : Storage<ImuMsg>,
        pub localization_msg : Storage<LocalizationMsg>,

    }

    impl Subscribe<DummyMsg> for MyStorage
    {
        fn subscribe(&mut self, message : &DummyMsg)
        {
            println!("Received new Dummy msg");
            self.dummy_msg.set(message.clone())
        }
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

    #[test]
    fn test_multi_subscriber() {



        let mut my_storage = MyStorage{dummy_msg:Storage(None), imu_msg:Storage(None), localization_msg:Storage(None)};

        let loc_msg = LocalizationMsg{pose: Pose{ x: 2.0, y: 5.0, theta: Angle::new(1.0, crate::geometry::AngleWrapping::PlusMinusPi) }};
        let imu_msg = ImuMsg{};
        let dummy_msg = DummyMsg{val:10};

        my_storage.subscribe(&imu_msg);
        my_storage.subscribe(&dummy_msg);


        
    }
}
