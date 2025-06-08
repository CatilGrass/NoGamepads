use std::fmt::Debug;
use bincode::{Decode, Encode};
use log::{error, trace};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use crate::data::message::traits::MessageEncoder;

pub async fn send_msg<Message>(
    stream: &mut TcpStream,
    msg: impl MessageEncoder<Message> + Encode + Decode<()> + Default + Debug
)
where Message: MessageEncoder<Message> + Encode + Decode<()> + Default + Debug {
    match stream.write_all(MessageEncoder::en(&msg).as_slice()).await {
        Ok(_) => { trace!("[Message Sender] Sent {:?} to {}", msg, get_target_address(stream)); }
        Err(err) => { error!("[Message Sender] Failed to send message: {}", err); }
    }
}

pub async fn read_msg<Message>(
    buffer: &mut [u8],
    stream: &mut TcpStream
) -> Message
where Message: MessageEncoder<Message> + Encode + Decode<()> + Default + Debug {
    match stream.read(buffer).await {
        Ok(read) => {
            let received = Message::de(Vec::from(&buffer[..read]));
            trace!("[Message Reader] Received {:?} from {}", received, get_target_address(stream));
            received
        }
        Err(err) => {
            error!("[Message Reader] Error reading from stream: {}", err);
            Message::err_result_decode()
        }
    }
}

pub fn get_target_address(stream: &TcpStream) -> String {
    let p = stream.peer_addr();
    if p.is_ok() {
        p.unwrap().to_string()
    } else {
        "Unknown".to_string()
    }
}