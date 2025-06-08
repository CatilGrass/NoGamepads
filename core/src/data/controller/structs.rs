use crate::data::player::structs::Player;

/// Controller-side Data
/// Describes the basic information of the controller side
#[derive(Default)]
pub struct ControllerData {

    /// Player bound to the controller side
    pub(crate) player: Player
}