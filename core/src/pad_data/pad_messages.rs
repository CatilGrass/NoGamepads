pub mod nogamepads_messages {
    use bincode::{Decode, Encode};
    use crate::pad_data::game_profile::game_profile::GameProfile;
    use crate::pad_data::pad_player_info::nogamepads_player_info::PlayerInfo;

    #[derive(Encode, Decode, PartialEq, Debug, Clone)]
    pub enum ControlMessage {

        Msg(String),

        Pressed(u8),

        Released(u8),

        Axis(u8, f64),

        Dir(u8, (f64, f64)),

        Exit,

        Err
    }

    #[derive(Encode, Decode, PartialEq, Debug, Clone)]
    pub enum GameMessage {

        SkinEventTrigger(u8),

        DisableKey(u8),

        EnableKey(u8),

        Leave(LeaveReason),

        Err
    }

    #[derive(Encode, Decode, PartialEq, Debug, Clone)]
    pub enum LeaveReason {

        GameOver,

        ServerClosed,

        YouAreKicked,

        YouAreBanned
    }

    #[derive(Encode, Decode, PartialEq, Debug, Clone)]
    pub enum ConnectionMessage {

        Connection(PlayerInfo),

        RequestProfile,

        RequestLayoutConfigure,

        RequestSkinPackage,

        Ready,

        Err
    }

    #[derive(Encode, Decode, PartialEq, Debug, Clone)]
    pub enum ConnectionCallbackMessage {

        Profile(GameProfile),

        Deny(ConnectionErrorType),

        Fail(ConnectionErrorType),

        Ok,

        Welcome,

        Err
    }

    #[derive(Encode, Decode, PartialEq, Debug, Clone)]
    pub enum ConnectionErrorType {

        ContainSamePlayer,

        PlayerBanned,

        Timeout,

        GameLocked,

        WhatTheHell
    }
}

pub mod nogamepads_message_encoder {
    use bincode::{Decode, Encode};
    use crate::{BINCODE_CONFIG, BINCODE_CONVERT_FAILED};
    use crate::pad_data::pad_messages::nogamepads_messages::{ConnectionCallbackMessage, ConnectionMessage, ControlMessage, GameMessage};

    pub trait NgpdMessageEncoder<Message: Encode + Decode<()>> {
        fn err_result_decode () -> Message;
        fn err_result_encode () -> Vec<u8> {
            BINCODE_CONVERT_FAILED
        }

        fn en(&self) -> Vec<u8> where Self : Encode {
            bincode::encode_to_vec(self, BINCODE_CONFIG)
                .unwrap_or_else(|_| Self::err_result_encode())
        }

        fn de(encoded : Vec<u8>) -> Message {
            match bincode::decode_from_slice(&encoded[..], BINCODE_CONFIG) {
                Ok((decoded, _)) => decoded,
                Err(_) => Self::err_result_decode()
            }
        }
    }

    impl NgpdMessageEncoder<ControlMessage> for ControlMessage {
        fn err_result_decode() -> ControlMessage {
            ControlMessage::Err
        }
    }

    impl NgpdMessageEncoder<GameMessage> for GameMessage {
        fn err_result_decode() -> GameMessage {
            GameMessage::Err
        }
    }

    impl NgpdMessageEncoder<ConnectionMessage> for ConnectionMessage {
        fn err_result_decode() -> ConnectionMessage {
            ConnectionMessage::Err
        }
    }

    impl NgpdMessageEncoder<ConnectionCallbackMessage> for ConnectionCallbackMessage {
        fn err_result_decode() -> ConnectionCallbackMessage {
            ConnectionCallbackMessage::Err
        }
    }
}

pub mod nogamepads_message_transfer {
    use bincode::{Decode, Encode};
    use log::error;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpStream;
    use crate::pad_data::pad_messages::nogamepads_message_encoder::NgpdMessageEncoder;

    pub async fn send_msg <Message>(stream: &mut TcpStream, msg: impl NgpdMessageEncoder<Message> + Decode<()> + Encode)
    where Message: NgpdMessageEncoder<Message> + Decode<()> + Encode {
        match stream.write_all(NgpdMessageEncoder::en(&msg).as_slice()).await {
            Ok(_) => {}
            Err(_) => {
                error!("Failed to send message.");
            }
        }
    }

    pub async fn read_msg<Message>(buffer: &mut [u8], stream: &mut TcpStream) -> Message
    where Message: NgpdMessageEncoder<Message> + Decode<()> + Encode {
        match stream.read(buffer).await {
            Ok(read) => {
                let received = &buffer[..read];
                <Message as NgpdMessageEncoder<Message>>::de(Vec::from(received))
            }
            Err(err) => {
                error!("Error reading from socket: {}", err);
                <Message as NgpdMessageEncoder<Message>>::err_result_decode()
            }
        }
    }
}