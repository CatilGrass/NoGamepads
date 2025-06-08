use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;
use bincode::{Decode, Encode};
use crate::data::{BINCODE_CONFIG, BINCODE_CONVERT_FAILED};
use crate::service::service_types::ServiceType;

/// Message Manager
/// Provides the ability to store and retrieve messages from a VecDeque
pub trait MessageManager<In, Out, Key>
where Key: Eq + Hash {
    fn borrow_received_list_mut(&mut self) -> &mut HashMap<(ServiceType, Key), VecDeque<In>>;

    fn borrow_send_list_mut(&mut self) -> &mut HashMap<(ServiceType, Key), VecDeque<Out>>;

    fn send(&mut self, message: Out, key: Key, service: ServiceType) {
        self.borrow_send_list_mut()
            .entry((service, key))
            .or_insert_with(VecDeque::new)
            .push_back(message);
    }

    fn receive(&mut self, key: Key, service: ServiceType) -> Option<In> {
        self.borrow_received_list_mut()
            .entry((service, key))
            .or_insert_with(VecDeque::new)
            .pop_front()
    }

    fn pop_from_send_list(&mut self, key: Key, service: ServiceType) -> Option<Out> {
        self.borrow_send_list_mut()
            .entry((service, key))
            .or_insert_with(VecDeque::new)
            .pop_front()
    }

    fn put_into_receive_list(&mut self, message: In, key: Key, service: ServiceType) {
        self.borrow_received_list_mut()
            .entry((service, key))
            .or_insert_with(VecDeque::new)
            .push_back(message);
    }
}

/// Message Encoder
/// Provides the ability to encode messages into binary data or decode them from binary data
pub trait MessageEncoder<M: Encode + Decode<()> + Default + Debug> {
    fn err_result_decode () -> M {
        M::default()
    }

    fn err_result_encode () -> Vec<u8> {
        BINCODE_CONVERT_FAILED
    }

    fn en(&self) -> Vec<u8> where Self : Encode {
        bincode::encode_to_vec(self, BINCODE_CONFIG)
            .unwrap_or_else(|_| Self::err_result_encode())
    }

    fn de(encoded : Vec<u8>) -> M {
        match bincode::decode_from_slice(&encoded[..], BINCODE_CONFIG) {
            Ok((decoded, _)) => decoded,
            Err(_) => Self::err_result_decode()
        }
    }
}