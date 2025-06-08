use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use crate::data::controller::runtime::structs::ControllerRuntime;

pub struct PadClientNetwork {
    pub(crate) addr: SocketAddr,
    pub(crate) runtime: Arc<Mutex<ControllerRuntime>>
}