use std::sync::{Arc, Mutex};
use nogamepads::entry_mutex;
use crate::data::game::runtime::structs::{GameControlRuntime, GameRuntime, GameRuntimeData};
use crate::data::game::structs::{GameControlData, GameData, GameRuntimeDataArchive};
use crate::data::game::types::{GameInfo, Players};
use crate::data::player::structs::Player;

impl Default for GameData {
    fn default() -> Self {
        GameData::new()
    }
}

impl GameData {

    /// Create new game data
    pub fn new() -> GameData {
        let mut game = GameData {
            info: GameInfo::default(),
            control: GameControlData::default(),
            archive: GameRuntimeDataArchive::default(),
        };

        game.name("Mini Hero".to_string());
        game.version(env!("PROJECT_VERSION").to_string());
        game
    }

    /// Add or modify game name information
    pub fn name(&mut self, name: String) -> &mut GameData {
        self.info("Game_Name".to_string(), name);
        self
    }

    /// Add or modify game version information
    pub fn version(&mut self, version: String) -> &mut GameData {
        self.info("Version".to_string(), version);
        self
    }

    /// Add or modify information for a specific entry
    pub fn info(&mut self, name: String, value: String) -> &mut GameData {
        self.info.insert(name, value);
        self
    }

    /// Read game runtime archive data
    pub fn load_data(&mut self, storage: GameRuntimeDataArchive) -> &mut GameData {
        self.archive = storage;
        self
    }

    /// Build the game-side runtime using game data
    pub fn runtime(self) -> Arc<Mutex<GameRuntime>> {
        let runtime = GameRuntime {
            info: self.info,
            data: self.archive.into(),
            control: GameControlRuntime {
                keys: self.control,
                ..Default::default()
            },

            writer_count: 0,
            reader_count: 0,
        };
        Arc::new(Mutex::new(runtime))
    }
}

impl From<GameRuntimeDataArchive> for GameRuntimeData {
    fn from(archive: GameRuntimeDataArchive) -> Self {
        let banned_mutex = Players::default();
        entry_mutex!(banned_mutex, |guard| {
            for account in archive.banned {
                let player_info = Player::from(account.clone());
                guard.entry(account).or_insert_with(|| player_info);
            }
        });
        GameRuntimeData {
            players_banned : banned_mutex,
            ..Self::default()
        }
    }
}

impl From<GameRuntimeData> for GameRuntimeDataArchive {
    fn from(data: GameRuntimeData) -> Self {
        let mut banned = Vec::new();
        entry_mutex!(data.players_online, |guard| {
            for account in guard.keys().into_iter() {
                banned.push(account.to_owned());
            }
        });

        GameRuntimeDataArchive {
            banned
        }
    }
}