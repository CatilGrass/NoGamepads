use std::collections::{HashMap, VecDeque};
use std::sync::atomic::Ordering::SeqCst;
use log::trace;
use crate::data::controller::runtime::structs::ControllerRuntime;
use crate::data::message::enums::{ControlMessage, GameMessage};
use crate::data::message::traits::MessageManager;
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
}