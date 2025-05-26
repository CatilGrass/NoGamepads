pub mod nogamepads_player_info {

    use bincode::{Decode, Encode};
    use hex::encode;
    use sha1::{Digest, Sha1};
    use serde::{Deserialize, Serialize};

    pub const ACCOUNT_HASH_SALT : &str = "Mr.Weicao";

    #[derive(Encode, Decode,
        Serialize, Deserialize,
        PartialEq, Debug)]
    pub struct PlayerInfo {
        pub account: PlayerAccountInfo,
        pub customize: PlayerCustomizeInfo
    }

    #[derive(Encode, Decode,
        Serialize, Deserialize,
        PartialEq, Debug)]
    pub struct PlayerAccountInfo {
        pub id: String,
        pub player_hash: String
    }

    #[derive(Encode, Decode,
        Serialize, Deserialize,
        PartialEq, Debug)]
    pub struct PlayerCustomizeInfo {
        pub nickname: String,

        pub color_hue: i32, // 0 - 360
        pub color_saturation: f64, // 0 - 1
        pub color_value: f64 // 0 - 1
    }

    impl PlayerInfo {

        pub fn new() -> PlayerInfo {
            PlayerInfo {
                customize: PlayerCustomizeInfo::default(),
                account: PlayerAccountInfo::default()
            }
        }

        pub fn set_nickname(&mut self, name: &str) -> &mut PlayerInfo {
            self.customize.nickname = String::from(name);
            self
        }

        pub fn set_customize_color_hue(&mut self, mut hue: i32) -> &mut PlayerInfo {
            hue = hue.clamp(0, 360);
            self.customize.color_hue = hue;
            self
        }

        pub fn set_customize_color_hsv(&mut self, mut hue: i32, mut saturation: f64, mut value: f64) -> &mut PlayerInfo {
            hue = hue.clamp(0, 360);
            saturation = saturation.clamp(0.0, 1.0);
            value = value.clamp(0.0, 1.0);

            self.customize.color_hue = hue;
            self.customize.color_saturation = saturation;
            self.customize.color_value = value;
            self
        }

        pub fn setup_account_info(&mut self, id: &str, password: &str) -> &mut PlayerInfo {

            let combined = format!("{}{}{}", id, password, ACCOUNT_HASH_SALT);
            let mut hasher = Sha1::new();
            hasher.update(combined);
            let result = hasher.finalize();

            self.account.id = String::from(id);
            self.account.player_hash = encode(&result[..]);
            self
        }
    }

    impl Clone for PlayerInfo {
        fn clone(&self) -> PlayerInfo {
            PlayerInfo {
                account: PlayerAccountInfo {
                    id: String::from(self.account.id.clone()),
                    player_hash: String::from(self.account.player_hash.clone())
                },
                customize: PlayerCustomizeInfo {
                    nickname: self.customize.nickname.clone(),
                    color_hue: self.customize.color_hue.clone(),
                    color_saturation: self.customize.color_saturation.clone(),
                    color_value: self.customize.color_value.clone()
                }
            }
        }
    }

    impl Default for PlayerCustomizeInfo {
        fn default() -> Self {
            PlayerCustomizeInfo {
                nickname: String::from("unnamed"),

                color_hue: 120,
                color_saturation: 1.0,
                color_value: 1.0
            }
        }
    }

    impl Default for PlayerAccountInfo {
        fn default() -> Self {
            PlayerAccountInfo {
                id: String::from("empty"),
                player_hash: String::from("")
            }
        }
    }
}

#[cfg(test)]
mod player_info_test {
    #[test]
    fn test_player_info_setup() {

    }
}