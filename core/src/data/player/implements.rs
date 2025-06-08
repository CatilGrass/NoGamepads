use crate::data::player::structs::{Account, Customize, Player};
use crate::data::player::ACCOUNT_HASH_SALT;
use hex::encode;
use sha1::{Digest, Sha1};
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use nogamepads::string_utils::process_id_text;

impl Player {

    /// Create new player information using a username and password
    pub fn register(id: String, password: String) -> Player {
        let mut player = Player {
            customize: None,
            account: Account::default()
        };

        let processed_id = process_id_text(id);

        player.account.id = processed_id.clone();
        player.account.player_hash = Self::gen_hash(processed_id, password);
        player
    }

    pub fn check(&self, password: String) -> bool {
        let hash = Self::gen_hash(self.account.id.clone(), password);
        hash == self.account.player_hash
    }

    fn gen_hash(processed_id: String, password: String) -> String {
        let combined = format!("{}{}{}", processed_id, password, ACCOUNT_HASH_SALT);
        let mut hasher = Sha1::new();
        hasher.update(combined);
        let result = hasher.finalize();
        encode(&result[..])
    }
}

// Customize implements
impl Player {

    /// Set player nickname
    pub fn nickname(&mut self, name: &String) -> &mut Player {
        self.change(|custom| {
            custom.nickname = name.clone();
            custom
        })
    }

    /// Set the hue of the player's color
    pub fn hue(&mut self, mut hue: i32) -> &mut Player {
        hue = hue.clamp(0, 360);
        self.change(|custom| {
            custom.color_hue = hue.clone();
            custom
        })
    }

    /// Set the player's HSV values
    pub fn hsv(&mut self, mut hue: i32, mut saturation: f64, mut value: f64) -> &mut Player {
        hue = hue.clamp(0, 360);
        saturation = saturation.clamp(0.0, 1.0);
        value = value.clamp(0.0, 1.0);
        self.change(|custom| {
            custom.color_hue = hue.clone();
            custom.color_saturation = saturation.clone();
            custom.color_value = value.clone();
            custom
        })
    }

    fn init(&mut self) {
        if self.customize.is_none() {
            self.customize = Some(Customize::default());
        }
    }

    fn change<F>(&mut self, f: F) -> &mut Player
    where F: FnOnce(&mut Customize) -> &mut Customize {
        self.init();
        let mut customize = self.customize.clone().unwrap();
        f(&mut customize);
        self.customize = Some(customize);
        self
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.account == other.account
    }
}

impl Hash for Player {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.account.hash(state);
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.account.id.as_str())
    }
}

impl From<Account> for Player {
    fn from(account: Account) -> Self {
        Player {
            account,
            customize: None
        }
    }
}

impl Display for Account {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.id.as_str())
    }
}