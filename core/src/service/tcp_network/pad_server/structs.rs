use crate::data::game::runtime::structs::GameRuntime;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::sync::watch::{Receiver, Sender};

pub struct PadServerNetwork {
    pub(crate) addr: SocketAddr,
    pub(crate) runtime: Arc<Mutex<GameRuntime>>,

    pub(crate) close_tx: Sender<bool>,
    pub(crate) close_rx: Receiver<bool>,
}