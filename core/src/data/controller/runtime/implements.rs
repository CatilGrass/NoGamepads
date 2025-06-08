use std::collections::{HashMap, VecDeque};
use std::sync::atomic::Ordering::SeqCst;
use log::trace;
use crate::data::controller::runtime::structs::ControllerRuntime;
use crate::data::message::enums::{ControlMessage, GameMessage};
use crate::data::message::traits::MessageManager;
use crate::data::player::structs::Account;
use crate::service::service_types::ServiceType;

/// Message manager for controller-side runtime
/// After the service starts, it can be accessed or relevant messages can be stored.
impl MessageManager<GameMessage, ControlMessage, u8> for ControllerRuntime {
    fn borrow_received_list_mut(&mut self) -> &mut HashMap<(ServiceType, u8), VecDeque<GameMessage>> {
        &mut self.received
    }

    fn borrow_send_list_mut(&mut self) -> &mut HashMap<(ServiceType, u8), VecDeque<ControlMessage>> {
        &mut self.send
    }
}

impl ControllerRuntime {

    pub fn close(&mut self) {
        if !self.close.load(SeqCst) {
            self.close.store(true, SeqCst);
            trace!("[Controller Runtime] Closed.");
        }
    }

    pub fn message(&mut self, message: String) {
        self.send_message(ControlMessage::Msg(message));
    }

    pub fn press_button(&mut self, key: u8) {
        self.send_message(ControlMessage::Pressed(key));
    }

    pub fn release_button(&mut self, key: u8) {
        self.send_message(ControlMessage::Released(key));
    }

    pub fn change_axis(&mut self, key: u8, ax_val: f64) {
        self.send_message(ControlMessage::Axis(key, ax_val));
    }

    pub fn change_direction(&mut self, key: u8, x: f64, y: f64) {
        self.send_message(ControlMessage::Dir(key, (x, y)));
    }

    pub fn pop(&mut self) -> Option<GameMessage> {
        self.receive(0, self.service_type.clone())
    }

    fn send_message (&mut self, msg: ControlMessage) {
        let service = self.service_type.clone();
        self.send(msg, 0, service);
    }
}