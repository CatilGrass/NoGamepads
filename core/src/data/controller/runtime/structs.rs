use std::collections::{HashMap, VecDeque};
use std::sync::atomic::AtomicBool;
use crate::data::game::types::GameInfo;
use crate::data::message::enums::{ControlMessage, GameMessage};
use crate::data::player::structs::Player;
use crate::service::service_types::ServiceType;

/// Controller-side runtime
/// Stores all data involved in game pad_client interactions during runtime
#[derive(Default)]
pub struct ControllerRuntime {

    pub(crate) received: HashMap<(ServiceType, u8), VecDeque<GameMessage>>,
    pub(crate) send: HashMap<(ServiceType, u8), VecDeque<ControlMessage>>,

    pub(crate) player: Player,

    pub game_info: GameInfo,
    pub close: AtomicBool
}