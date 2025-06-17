use std::sync::{Arc, Mutex};
use crate::data::controller::controller_runtime::ControllerRuntime;
use crate::data::player::player_data::Player;

/// Controller-side Data
/// Describes the basic information of the controller side
#[derive(Default, Clone)]
pub struct ControllerData {

    /// Player bound to the controller side
    pub(crate) player: Player
}

impl ControllerData {

    pub fn bind_player(&mut self, player: Player) -> &mut ControllerData {
        self.player = player;
        self
    }

    /// Build the controller-side runtime using controller data
    pub fn runtime(self) -> Arc<Mutex<ControllerRuntime>> {
        let runtime = ControllerRuntime {
            player: self.player,
            ..Default::default()
        };
        Arc::new(Mutex::new(runtime))
    }

    /// Build the controller-side runtime using controller data (borrowed)
    pub fn runtime_with_borrowed_data(&self) -> Arc<Mutex<ControllerRuntime>> {
        let runtime = ControllerRuntime {
            player: self.player.clone(),
            ..Default::default()
        };
        Arc::new(Mutex::new(runtime))
    }
}