use std::sync::{Arc, Mutex};
use crate::data::controller::runtime::structs::ControllerRuntime;
use crate::data::controller::structs::ControllerData;
use crate::data::player::structs::Player;

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
}