pub mod game_profile {
    use std::fmt::Display;
    use bincode::{Decode, Encode};
    use serde::{Deserialize, Serialize};

    #[derive(Encode, Decode, Serialize, Deserialize, PartialEq, Debug)]
    pub struct GameProfile {

        // 游戏名称
        pub game_name: String,

        // 游戏描述
        pub game_description: String,

        // 游戏组织
        pub organization: String,

        // 游戏版本
        pub version: String,

        // 工作室 & 游戏 主页
        pub website: String,

        // 交流邮箱
        pub email: String
    }

    impl Default for GameProfile {
        fn default() -> Self {
            GameProfile {
                game_name: "Unnamed Game".to_string(),
                game_description: "".to_string(),
                organization: "".to_string(),
                version: "0.1".to_string(),
                website: "".to_string(),
                email: "".to_string()
            }
        }
    }

    impl Clone for GameProfile {
        fn clone(&self) -> Self {
            GameProfile {
                game_name: self.game_name.clone(),
                game_description: self.game_description.clone(),
                organization: self.organization.clone(),
                version: self.version.clone(),
                website: self.website.clone(),
                email: self.email.clone()
            }
        }
    }

    impl Display for GameProfile {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut string = String::new();
            string += format!("Game Name: {}\n", self.game_name).as_str();
            if !self.game_description.eq("") { string += format!("Description: {}\n", self.game_description).as_str(); }
            if !self.organization.eq("") { string += format!("Org: {}\n", self.organization).as_str(); }
            if !self.website.eq("") { string += format!("- Web: {}\n", self.website).as_str(); }
            if !self.version.eq("") { string += format!("Version: {}\n", self.version).as_str(); }
            if !self.email.eq("") { string += format!("- E-mail: {}\n", self.email).as_str(); }

            write!(f, "{}", string)
        }
    }

    impl GameProfile {
        pub fn game_name(&mut self, game_name: &str) -> &mut GameProfile {
            self.game_name = game_name.to_string();
            self
        }

        pub fn game_description(&mut self, game_description: &str) -> &mut GameProfile {
            self.game_description = game_description.to_string();
            self
        }

        pub fn organization(&mut self, organization: &str) -> &mut GameProfile {
            self.organization = organization.to_string();
            self
        }

        pub fn version(&mut self, version: &str) -> &mut GameProfile {
            self.version = version.to_string();
            self
        }

        pub fn website(&mut self, website: &str) -> &mut GameProfile {
            self.website = website.to_string();
            self
        }

        pub fn email(&mut self, email: &str) -> &mut GameProfile {
            self.email = email.to_string();
            self
        }
    }
}