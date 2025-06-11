use crate::data::message::message_enums::{ConnectionResponseMessage, JoinFailedMessage, ConnectionMessage, ControlMessage, ExitReason, GameMessage};
use crate::data::message::traits::MessageEncoder;

#[macro_export]
macro_rules! encoder {
    ($($msg:ident),+) => {
        $(
            impl MessageEncoder<$msg> for $msg {}
        )+
    };
}

encoder!(
    ControlMessage, GameMessage, ExitReason, ConnectionMessage, ConnectionResponseMessage, JoinFailedMessage
);