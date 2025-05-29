pub mod layout_data {
    use std::collections::{HashMap, VecDeque};
    use bincode::{Decode, Encode};
    use serde::{Deserialize, Serialize};
    use crate::pad_data::pad_messages::nogamepads_messages::ControlMessage;
    use crate::pad_data::pad_player_info::nogamepads_player_info::PlayerInfo;
    use crate::pad_service::server::nogamepads_server::PadServer;


    #[derive(Encode, Decode, Serialize, Deserialize, PartialEq, Debug, Clone)]
    pub struct LayoutKeyRegisters {
        pub direction_keys : HashMap<u8, String>, // 注册方向键
        pub axis_keys : HashMap<u8, String>,  // 注册轴向键
        pub button_keys : HashMap<u8, String>, // 注册按钮
    }

    impl Default for LayoutKeyRegisters {
        fn default() -> LayoutKeyRegisters {
            LayoutKeyRegisters {
                direction_keys: Default::default(),
                axis_keys: Default::default(),
                button_keys: Default::default(),
            }
        }
    }

    pub struct LayoutKeyRuntimeData {
        directions : HashMap<u8, HashMap<String, (f64, f64)>>, // <键, <玩家Hash, (x, y)>>
        axes : HashMap<u8, HashMap<String, f64>>, // <键, <玩家Hash, 轴向>>
        button : HashMap<u8, HashMap<String, bool>>, // <键, <玩家Hash, 是否按下>>

        events : VecDeque<(String, ControlMessage)>, // (玩家Hash, 信息)
    }

    impl Default for LayoutKeyRuntimeData {
        fn default() -> Self {
            LayoutKeyRuntimeData {
                directions: Default::default(),
                axes: Default::default(),
                button: Default::default(),
                events: Default::default(),
            }
        }
    }

    impl LayoutKeyRuntimeData {

        // 输入控制信息到数据
        pub fn insert_control(&mut self, who: PlayerInfo, msg: ControlMessage) {
            match msg {
                // 消息放入信息队列待读取
                ControlMessage::Msg(_) => {
                    self.events.push_back((who.account.player_hash, msg))
                }

                // 按钮更新对应玩家的状态，并且放入信息队列待读取
                ControlMessage::Pressed(button_key) => {
                    self.button.entry(button_key)
                        .or_insert_with(HashMap::new)
                        .insert(who.account.player_hash.clone(), true);
                    self.events.push_back((who.account.player_hash, msg))
                }
                ControlMessage::Released(button_key) => {
                    self.button.entry(button_key)
                        .or_insert_with(HashMap::new)
                        .insert(who.account.player_hash.clone(), false);
                    self.events.push_back((who.account.player_hash, msg))
                }

                // 轴向更新直接传入玩家状态
                ControlMessage::Axis(axis_key, axis) => {
                    self.axes.entry(axis_key)
                        .or_insert_with(HashMap::new)
                        .insert(who.account.player_hash.clone(), axis);
                }
                ControlMessage::Dir(dir_key, (x, y)) => {
                    self.directions.entry(dir_key)
                        .or_insert_with(HashMap::new)
                        .insert(who.account.player_hash.clone(), (x, y));
                }
                _ => { }
            }
        }

        pub fn pop_control_event(&mut self, server: &PadServer) -> Option<(PlayerInfo, ControlMessage)> {
            let pop = self.events.pop_front();
            if pop.is_some() {
                let (hash, msg) = pop.unwrap();
                let info = server.find_online_player(hash);
                if info.is_some() {
                    Some((info.unwrap(), msg))
                } else {
                    None
                }
            } else {
                None
            }
        }

        pub fn get_direction(&self, who: &PlayerInfo, key: &u8) -> Option<(f64, f64)> {
            Self::get(&self.directions, who, key)
        }

        pub fn get_axis(&self, who: &PlayerInfo, key: &u8) -> Option<f64> {
            Self::get(&self.axes, who, key)
        }

        pub fn get_button_status(&self, who: &PlayerInfo, key: &u8) -> Option<bool> {
            Self::get(&self.button, who, key)
        }

        fn get<V: Clone>(map: &HashMap<u8, HashMap<String, V>>, who: &PlayerInfo, key: &u8) -> Option<V> {
            let key = map.get(key);
            if key.is_some() {
                let value = key.unwrap().get(&who.account.player_hash);
                if value.is_some() {
                    let result = value.unwrap();
                    Some(result.clone())
                } else { None }
            } else { None }
        }
    }
}

pub mod layout_gamepad {
    use bincode::{Decode, Encode};
    use serde::{Deserialize, Serialize};

    #[derive(Encode, Decode, Serialize, Deserialize, PartialEq, Debug)]
    pub struct PadLayout {

    }

    pub trait ButtonArea {

    }
}