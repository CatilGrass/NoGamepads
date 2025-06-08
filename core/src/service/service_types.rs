use bincode::{Decode, Encode};
use crate::data::message::traits::MessageEncoder;
use crate::encoder;

#[derive(Default, Encode, Decode, PartialEq, Debug, Clone, Eq, Hash)]
pub enum ServiceType {
    #[default]
    TCPConnection,
    BlueTooth,
    USB,
}

encoder!(ServiceType);