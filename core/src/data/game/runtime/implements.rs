use std::collections::{HashMap, VecDeque};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::SeqCst;
use log::{info, trace, warn};
use nogamepads::entry_mutex;
use crate::data::game::runtime::structs::{GameControlRuntime, GameRuntime, GameRuntimeData};
use crate::data::game::types::Players;
use crate::data::message::enums::{JoinFailedMessage, ControlMessage, ExitReason, GameMessage};
use crate::data::message::enums::JoinFailedMessage::{ContainIdenticalPlayer, GameLocked, PlayerBanned};
use crate::data::message::enums::ControlMessage::{Axis, Dir, Msg, Pressed, Released};
use crate::data::message::enums::ExitReason::{YouAreBanned, YouAreKicked};
use crate::data::message::enums::GameMessage::{EventTrigger, LetExit};
use crate::data::message::traits::MessageManager;
use crate::data::player::structs::{Account, Player};
use crate::service::service_types::ServiceType;
use crate::service::service_types::ServiceType::TCPConnection;

impl GameRuntime {

    /// Attempt to have the specified player join the game
    pub fn try_join_player(&mut self, player: Player) -> Result<(), JoinFailedMessage> {
        let join = self.can_join_game(&player.account);
        match join {
            Ok(_) => {
                self.data.sign_player_online_status(&player, TCPConnection, true);
                trace!("[Game Runtime] Player \"{}\" joined", player.account);
                Ok(())
            }
            Err(why) => {
                warn!("[Game Runtime] Player \"{}\" join failed: {:?}", player.account, why);
                Err(why)
            }
        }
    }

    fn can_join_game(&self, account: &Account) -> Result<bool, JoinFailedMessage> {

        if self.is_game_locked() {
            Err(GameLocked)
        } else if self.data.is_account_banned(account) {
            Err(PlayerBanned)
        } else if self.data.is_account_online(account) {
            Err(ContainIdenticalPlayer)
        } else {
            Ok(true)
        }
    }

    /// Request an account to exit
    pub fn let_account_exit(&mut self, account: &Account, reason: ExitReason, service_type: ServiceType) {
        // Send a leave message to the pad_client and wait for it to actively disconnect
        if self.data.is_account_online(account) {
            self.send((account.clone(), LetExit(reason)), account.clone(), service_type);
        }
    }

    pub fn kick_player(&mut self, player: &Player, service_type: ServiceType) {
        // Send a leave message to the pad_client and wait for it to actively disconnect
        if self.data.is_account_online(&player.account) {
            self.send((player.account.clone(), LetExit(YouAreKicked)), player.account.clone(), service_type);
        }
    }

    pub fn ban_player(&mut self, player: &Player, service_type: ServiceType) {
        if self.data.is_account_online(&player.account) {
            self.send((player.account.clone(), LetExit(YouAreBanned)), player.account.clone(), service_type);
            entry_mutex!(self.data.players_banned, |guard| {
                guard.insert(player.account.clone(), player.clone());
            });
        }
    }

    pub fn pardon_player(&mut self, player: &Player) {
        entry_mutex!(self.data.players_banned, |guard| {
            guard.remove(&player.account);
        });
    }

    /// Check if the game is locked
    pub fn is_game_locked(&self) -> bool {
        self.data.locked.load(SeqCst)
    }

    /// Lock the game
    pub fn lock_game(&self) {
        if !self.data.locked.load(SeqCst) {
            self.data.locked.store(true, SeqCst);
            info!("[Game Runtime] Game locked!");
        }
    }

    /// Unlock the game
    pub fn unlock_game(&self) {
        if self.data.locked.load(SeqCst) {
            self.data.locked.store(false, SeqCst);
            info!("[Game Runtime] Game unlocked!");
        }
    }

    /// Close the Game
    pub fn close_game(&self) {
        if !self.data.close.load(SeqCst) {
            self.data.close.store(true, SeqCst);
            info!("[Game Runtime] Game closed!");
        }
    }

    /// Send a GameMessage to account
    pub fn send_game_message(&mut self, account: &Account, message: GameMessage, service_type: ServiceType) {
        self.send((account.clone(), message), account.clone(), service_type);
    }

    pub fn send_event(&mut self, account: &Account, event_trigger: u8, service_type: ServiceType) {
        self.send_game_message(account, EventTrigger(event_trigger), service_type);
    }

    pub fn send_message(&mut self, account: &Account, message: String, service_type: ServiceType) {
        self.send_game_message(account, GameMessage::Msg(message), service_type);
    }

    /// Pop an event message
    pub fn pop_event(&mut self) -> Option<(Account, ControlMessage)> {
        let pop = self.control.events.pop_front();
        if pop.is_some() {
            let (account, msg) = pop.unwrap();
            if self.data.is_account_online(&account) {
                trace!("[Control Runtime] Message: {:?} from \"{}\" ", &msg, account);
                Some((account, msg))
            } else {
                warn!("[Control Runtime] Invalid message: Player \"{}\" is not online!", account);
                None
            }
        } else {
            None
        }
    }
}

/// Message manager for game pad_client runtime
/// After the service starts, it can be accessed or relevant messages can be stored.
impl MessageManager<(Account, ControlMessage), (Account, GameMessage), Account> for GameRuntime {
    fn borrow_received_list_mut(&mut self) -> &mut HashMap<(ServiceType, Account), VecDeque<(Account, ControlMessage)>> {
        &mut self.data.received
    }

    fn borrow_send_list_mut(&mut self) -> &mut HashMap<(ServiceType, Account), VecDeque<(Account, GameMessage)>> {
        &mut self.data.send
    }

    fn pop_from_send_list(&mut self, key: Account, service: ServiceType) -> Option<(Account, GameMessage)> {
        let key = (service, key);
        self.borrow_send_list_mut()
            .entry(key)
            .or_insert_with(VecDeque::new)
            .pop_front()
    }

    fn put_into_receive_list(&mut self, message: (Account, ControlMessage), _key: Account, _service: ServiceType) {
        let result = self.control.process_control_message(&message.0, message.1);
        if result.is_err() {
            let result = result.unwrap_err();
            warn!("[Game Runtime] Can't process message: {:?}", result);
            drop(result);
        }
    }
}

impl Default for GameRuntimeData {
    fn default() -> Self {
        Self {
            received: Default::default(),
            send: Default::default(),
            players_online: Players::default(),
            players_banned: Players::default(),
            account_service_type: Default::default(),

            locked: AtomicBool::new(false),
            close: AtomicBool::new(false)
        }
    }
}

impl GameRuntimeData {

    /// Mark a player as online
    pub fn sign_player_online_status(&mut self, player: &Player, service_type: ServiceType, value: bool) {
        let online = self.is_account_online(&player.account);
        if online && !value {

            // Remove player
            entry_mutex!(self.players_online, |guard| {
                guard.remove_entry(&player.account);
            });

            info!("[Game Runtime] Signed player \"{}\" is [OFFLINE]!", player.account);

            // Reset runtime
            let key = (service_type, player.account.clone());
            let get_received = self.received.get_mut(&key);
            let get_send = self.send.get_mut(&key);
            if let Some(list) = get_received {
                list.clear();
            }
            if let Some(list) = get_send {
                list.clear();
            }

        } else if !online & value {

            // Insert player
            entry_mutex!(self.players_online, |guard| {
                guard.entry(player.account.clone())
                .or_insert_with(|| player.clone());
            });

            info!("[Game Runtime] Signed player \"{}\" is [ONLINE]!", player.account);

            // Record service type
            entry_mutex!(self.account_service_type, |guard| {
                guard.entry(player.account.clone())
                .or_insert_with(|| TCPConnection);
            })
        }
    }

    /// Returns all online accounts
    pub fn online_accounts(&self) -> Vec<Account> {
        let mut vec = Vec::new();
        entry_mutex!(self.players_online, |guard| {
            for account in guard.keys().into_iter() {
                vec.push(account.clone());
            }
        });
        vec
    }

    /// Check if specified account is online
    pub fn is_account_online(&self, account: &Account) -> bool {
        entry_mutex!(self.players_online, |guard| {
            if guard.contains_key(account) {
                return true;
            }
        });
        false
    }

    /// Returns all banned accounts
    pub fn banned_accounts(&self) -> Vec<Account> {
        let mut vec = Vec::new();
        entry_mutex!(self.players_banned, |guard| {
            for account in guard.keys().into_iter() {
                vec.push(account.clone());
            }
        });
        vec
    }

    /// Check if account is banned
    pub fn is_account_banned(&self, account: &Account) -> bool {
        entry_mutex!(self.players_banned, |guard| {
            if guard.contains_key(account) {
                true;
            }
        });
        false
    }

    /// Get service type of account
    pub fn get_service_type(&self, account: &Account) -> Option<ServiceType> {
        let mut result = None;
        entry_mutex!(self.account_service_type, |guard| {
            result = guard.get(account).cloned();
        });
        result
    }
}

impl GameControlRuntime {

    /// Process a control message
    fn process_control_message(&mut self, who: &Account, msg: ControlMessage) -> Result<(), ControlMessage> {
        match msg {
            Msg(_) => {
                self.send_event(who, msg);
                Ok(())
            }

            Pressed(button_key) => {
                let key_valid = self.check_key(&self.keys.button_keys, &button_key);
                if key_valid {
                    Self::change_value(&mut self.button, button_key, who, true);
                    self.send_event(who, msg);
                    trace!("[Control Runtime] Player \"{}\" pressed btn_{}", &who.id, button_key);
                } else {
                    warn!("[Control Runtime] Key btn_{} not registered!", button_key);
                }
                Ok(())
            }

            Released(button_key) => {
                if self.check_key(&self.keys.button_keys, &button_key) {
                    Self::change_value(&mut self.button, button_key, who, false);
                    self.send_event(who, msg);
                    trace!("[Control Runtime] Player \"{}\" released btn_{}", &who.id, button_key);
                } else {
                    warn!("[Control Runtime] Key btn_{} not registered!", button_key);
                }
                Ok(())
            }

            Axis(axis_key, axis) => {
                if self.check_key(&self.keys.button_keys, &axis_key) {
                    Self::change_value(&mut self.axes, axis_key, who, axis);
                    trace!("[Control Runtime] Player \"{}\" changed ax_{} to ({})", &who.id, axis_key, axis);
                } else {
                    warn!("[Control Runtime] Key ax_{} not registered!", axis_key);
                }
                Ok(())
            }

            Dir(dir_key, dir) => {
                if self.check_key(&self.keys.button_keys, &dir_key) {
                    Self::change_value(&mut self.directions, dir_key, who, dir);
                    trace!("[Control Runtime] Player \"{}\" changed dir_{} to ({}, {})", &who.id, dir_key, dir.0, dir.1);
                } else {
                    warn!("[Control Runtime] Key dir_{} not registered!", dir_key);
                }
                Ok(())
            }

            _ => {
                Err(msg)
            }
        }
    }

    /// Get specified player's direction value
    pub fn get_direction(&self, who: &Account, key: &u8) -> Option<(f64, f64)> {
        Self::get(&self.directions, who, key)
    }

    /// Get specified player's axis value
    pub fn get_axis(&self, who: &Account, key: &u8) -> Option<f64> {
        Self::get(&self.axes, who, key)
    }

    /// Get specified player's button status
    pub fn get_button_status(&self, who: &Account, key: &u8) -> Option<bool> {
        Self::get(&self.button, who, key)
    }

    fn check_key(&self, map: &HashMap<u8, String>, key: &u8) -> bool {
        map.contains_key(key)
    }

    fn get<V: Clone>(map: &HashMap<u8, HashMap<Account, V>>, who: &Account, key: &u8) -> Option<V> {
        let key = map.get(key);
        if key.is_some() {
            let value = key.unwrap().get(who);
            if value.is_some() {
                let result = value.unwrap();
                Some(result.clone())
            } else { None }
        } else { None }
    }

    fn change_value<T>(map: &mut HashMap<u8, HashMap<Account, T>>, key: u8, who: &Account, msg: T) {
        map.entry(key)
            .or_insert_with(HashMap::new)
            .insert(who.clone(), msg);
    }

    fn send_event(&mut self, who: &Account, msg: ControlMessage) {
        self.events.push_back((who.clone(), msg));
    }
}