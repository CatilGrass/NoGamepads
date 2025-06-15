use crate::data::ngpd_message::{free_control_message, FfiControlMessage, FfiExitReason, FfiGameMessage};
use crate::data::ngpd_player::{free_player, FfiPlayer};
use crate::service::ngpd_service_types::FfiServiceType;
use nogamepads::entry_mutex;
use nogamepads_core::data::game::game_data::{GameData, GameRuntimeDataArchive};
use nogamepads_core::data::game::game_runtime::GameRuntime;
use nogamepads_core::data::message::message_enums::{ExitReason, GameMessage};
use nogamepads_core::data::player::player_data::Player;
use nogamepads_core::service::service_types::ServiceType;
use std::ffi::{c_char, c_double, c_void, CStr};
use std::ptr::null;
use std::sync::{Arc, Mutex, MutexGuard};

#[repr(C)]
pub struct FfiGameData(*mut c_void);

#[repr(C)]
pub struct FfiGameRuntimeArchive(*mut c_void);

#[repr(C)]
pub struct FfiControlEvent {
    player: FfiPlayer,
    message: FfiControlMessage
}

#[repr(C)]
pub struct FfiGameRuntime {
    inner: *mut c_void,
    drop_fn: extern "C" fn(*mut c_void),
}

#[repr(C)]
pub struct FfiButtonStatus {
    found: bool,
    pressed: bool,
    released: bool
}

#[repr(C)]
pub struct FfiAxis {
    found: bool,
    axis: c_double
}

#[repr(C)]
pub struct FfiDirection {
    found: bool,
    x: c_double,
    y: c_double,
}

#[repr(C)]
pub struct FfiBooleanResult {
    found: bool,
    result: bool
}

#[repr(C)]
pub struct FfiPlayerList {
    players: *mut FfiPlayer,
    len: usize,
    cap: usize,
}

impl FfiGameData {

    /// Create game data
    #[unsafe(no_mangle)]
    pub extern "C" fn game_data_new() -> *mut FfiGameData {
        let game_data = GameData::default();
        let raw = Box::into_raw(Box::new(FfiGameData(Box::into_raw(Box::new(game_data)) as *mut _)));
        raw
    }

    /// Load data archive
    #[unsafe(no_mangle)]
    pub extern "C" fn game_data_load_archive(
        data: *mut FfiGameData,
        archive: *mut FfiGameRuntimeArchive,
    ) -> *mut FfiGameData {
        if data.is_null() {
            return std::ptr::null_mut();
        }

        if archive.is_null() {
            return std::ptr::null_mut();
        }

        let data_inner = unsafe { &mut *((*data).0 as *mut GameData) };
        let archive_inner = unsafe { &mut *((*archive).0 as *mut GameRuntimeDataArchive) };
        let archive = *unsafe { Box::from_raw(archive_inner) };
        data_inner.load_data(archive);

        let raw = Box::into_raw(Box::new(FfiGameData(Box::into_raw(Box::new(data)) as *mut _)));
        raw
    }

    /// Build runtime by data
    #[unsafe(no_mangle)]
    pub extern "C" fn game_data_build_runtime(
        data: *mut FfiGameData
    ) -> *mut FfiGameRuntime {
        if data.is_null() {
            return std::ptr::null_mut();
        }

        let data_inner = unsafe { &mut *((*data).0 as *mut GameData) };
        let game_data = *unsafe { Box::from_raw(data_inner) };

        let runtime = game_data.runtime();

        extern "C" fn drop_runtime(raw: *mut c_void) {
            unsafe {
                let arc_ptr = raw as *const Arc<Mutex<GameRuntime>>;
                drop(Arc::from_raw(arc_ptr));
            }
        }

        let arc_raw = Arc::into_raw(runtime.clone()) as *mut c_void;

        Box::into_raw(Box::new(FfiGameRuntime {
            inner: arc_raw,
            drop_fn: drop_runtime,
        }))
    }

    /// Free data
    #[unsafe(no_mangle)]
    pub extern "C" fn free_game_data(data: *mut FfiGameData) {
        if data.is_null() {
            return;
        }
        let ffi_wrapper = unsafe { Box::from_raw(data) };
        let inner_ptr = ffi_wrapper.0 as *mut GameData;
        let _ = unsafe { Box::from_raw(inner_ptr) };
    }
}

impl FfiGameRuntimeArchive {

    /// Create game archive data
    #[unsafe(no_mangle)]
    pub extern "C" fn game_archive_data_new() -> *mut FfiGameRuntimeArchive {
        let archive_data = GameRuntimeDataArchive::default();
        let raw = Box::into_raw(Box::new(FfiGameRuntimeArchive(Box::into_raw(Box::new(archive_data)) as *mut _)));
        raw
    }

    /// Add ban player
    #[unsafe(no_mangle)]
    pub extern "C" fn game_archive_data_add_ban_player(
        data: *mut FfiGameRuntimeArchive,
        ffi_player: *mut FfiPlayer
    ) -> *mut FfiGameRuntimeArchive {
        if data.is_null() {
            return std::ptr::null_mut();
        }

        if ffi_player.is_null() {
            return std::ptr::null_mut();
        }

        let data_inner = unsafe { &mut *((*data).0 as *mut GameRuntimeDataArchive) };
        let ffi_player_ref = unsafe { &*ffi_player };
        let player = Player::try_from(&*ffi_player_ref).unwrap_or_default();

        data_inner.banned.push(player.account);

        Box::into_raw(Box::new(FfiGameRuntimeArchive(Box::into_raw(Box::new(data_inner)) as *mut _)))
    }

    /// Free data
    #[unsafe(no_mangle)]
    pub extern "C" fn free_game_archive_data(data: *mut FfiGameRuntimeArchive) {
        if data.is_null() {
            return;
        }
        let ffi_wrapper = unsafe { Box::from_raw(data) };
        let inner_ptr = ffi_wrapper.0 as *mut GameRuntimeDataArchive;
        let _ = unsafe { Box::from_raw(inner_ptr) };
    }
}

impl FfiGameRuntime {

    fn send_message_to(
        runtime: *mut FfiGameRuntime,
        player: *const FfiPlayer,
        service_type: *mut FfiServiceType,
        message: GameMessage,
    ) {
        if runtime.is_null() || player.is_null() { return; }
        let arc_ptr = unsafe { (*runtime).inner as *const Arc<Mutex<GameRuntime>> };
        let arc_ref = unsafe { Arc::from_raw(arc_ptr) };
        let ffi_player_ref = unsafe { &*player };
        let player = Player::try_from(&*ffi_player_ref).unwrap_or_default();
        let service = unsafe { ServiceType::from(&service_type.read()) };

        entry_mutex!(*arc_ref, |mutex_guard| {
            mutex_guard.send_game_message(&player.account, message, service);
        });
    }

    fn do_on_rt(
        runtime: *mut FfiGameRuntime,
        do_on: fn(guard: &mut MutexGuard<GameRuntime>)
    ) {
        if runtime.is_null() { return; }
        let arc_ptr = unsafe { (*runtime).inner as *const Arc<Mutex<GameRuntime>> };
        let arc_ref = unsafe { Arc::from_raw(arc_ptr) };

        entry_mutex!(*arc_ref, |mutex_guard| {
            do_on(mutex_guard);
        });
    }

    /// Send a message to
    #[unsafe(no_mangle)]
    pub extern "C" fn game_runtime_send_message_to(
        runtime: *mut FfiGameRuntime,
        player: *const FfiPlayer,
        message: *mut FfiGameMessage,
        service_type: *mut FfiServiceType
    ) {
        if message.is_null() { return; }
        let msg = unsafe { GameMessage::from(message.read()) };
        Self::send_message_to(runtime, player, service_type, msg);
    }

    /// Send a text message
    #[unsafe(no_mangle)]
    pub extern "C" fn game_runtime_send_text_message(
        runtime: *mut FfiGameRuntime,
        player: *const FfiPlayer,
        service_type: *mut FfiServiceType,
        text: *const c_char
    ) {
        let text_str = unsafe { CStr::from_ptr(text) }.to_string_lossy().into_owned().to_string();
        Self::send_message_to(runtime, player, service_type, GameMessage::Msg(text_str));
    }

    /// Send a event message
    #[unsafe(no_mangle)]
    pub extern "C" fn game_runtime_send_event(
        runtime: *mut FfiGameRuntime,
        player: *const FfiPlayer,
        service_type: *mut FfiServiceType,
        key: u8
    ) {
        Self::send_message_to(runtime, player, service_type, GameMessage::EventTrigger(key));
    }

    /// Pop a control event
    #[unsafe(no_mangle)]
    pub extern "C" fn game_runtime_pop_control_event(runtime: *mut FfiGameRuntime) -> *mut FfiControlEvent {
        if runtime.is_null() { return std::ptr::null_mut(); }
        let arc_ptr = unsafe { (*runtime).inner as *const Arc<Mutex<GameRuntime>> };
        let arc_ref = unsafe { Arc::from_raw(arc_ptr) };
        let mut control_event = None;
        entry_mutex!(*arc_ref, |mutex_guard| {
            control_event = mutex_guard.pop_control_event();
        });
        match control_event {
            None => { std::ptr::null_mut() }
            Some((account, message)) => {
                let hash = account.player_hash;
                let player = FfiPlayer::from(&Player::register_from_hash(hash));
                let message = FfiControlMessage::from(message);
                Box::into_raw(Box::new(FfiControlEvent { player, message }))
            }
        }
    }

    /// Let player exit
    #[unsafe(no_mangle)]
    pub extern "C" fn game_runtime_let_exit(
        runtime: *mut FfiGameRuntime,
        player: *const FfiPlayer,
        service_type: *mut FfiServiceType,
        reason: *mut FfiExitReason
    ) {
        if reason.is_null() { return; }
        let reason = unsafe { reason.read() };
        let reason_msg = GameMessage::LetExit(ExitReason::from(&reason));
        Self::send_message_to(runtime, player, service_type, reason_msg);
    }

    /// Kick a player
    #[unsafe(no_mangle)]
    pub extern "C" fn game_runtime_kick_player(
        runtime: *mut FfiGameRuntime,
        player: *const FfiPlayer,
        service_type: *mut FfiServiceType
    ) {
        if runtime.is_null() || player.is_null() || service_type.is_null() { return; }

        let ffi_player_ref = unsafe { &*player };
        let player = Player::try_from(&*ffi_player_ref).unwrap_or_default();
        let service = unsafe { ServiceType::from(&service_type.read()) };

        if runtime.is_null() { return; }
        let arc_ptr = unsafe { (*runtime).inner as *const Arc<Mutex<GameRuntime>> };
        let arc_ref = unsafe { Arc::from_raw(arc_ptr) };
        entry_mutex!(*arc_ref, |mutex_guard| {
            mutex_guard.kick_player(&player, service);
        });
    }

    /// Ban a player (And kick)
    #[unsafe(no_mangle)]
    pub extern "C" fn game_runtime_ban_player(
        runtime: *mut FfiGameRuntime,
        player: *const FfiPlayer,
        service_type: *mut FfiServiceType
    ) {
        if runtime.is_null() || player.is_null() || service_type.is_null() { return; }

        let ffi_player_ref = unsafe { &*player };
        let player = Player::try_from(&*ffi_player_ref).unwrap_or_default();
        let service = unsafe { ServiceType::from(&service_type.read()) };

        if runtime.is_null() { return; }
        let arc_ptr = unsafe { (*runtime).inner as *const Arc<Mutex<GameRuntime>> };
        let arc_ref = unsafe { Arc::from_raw(arc_ptr) };
        entry_mutex!(*arc_ref, |mutex_guard| {
            mutex_guard.ban_player(&player, service);
        });
    }

    /// Pardon a player
    #[unsafe(no_mangle)]
    pub extern "C" fn game_runtime_pardon_player(
        runtime: *mut FfiGameRuntime,
        player: *const FfiPlayer
    ) {
        if runtime.is_null() || player.is_null() { return; }

        let ffi_player_ref = unsafe { &*player };
        let player = Player::try_from(&*ffi_player_ref).unwrap_or_default();

        if runtime.is_null() { return; }
        let arc_ptr = unsafe { (*runtime).inner as *const Arc<Mutex<GameRuntime>> };
        let arc_ref = unsafe { Arc::from_raw(arc_ptr) };
        entry_mutex!(*arc_ref, |mutex_guard| {
            mutex_guard.pardon_player(&player);
        });
    }

    /// Close runtime
    #[unsafe(no_mangle)]
    pub extern "C" fn game_runtime_close(runtime: *mut FfiGameRuntime) {
        if runtime.is_null() { return; }
        Self::do_on_rt(runtime, |guard| {
            guard.close_game();
        })
    }

    /// Lock game
    #[unsafe(no_mangle)]
    pub extern "C" fn game_runtime_lock(runtime: *mut FfiGameRuntime) {
        if runtime.is_null() { return; }
        Self::do_on_rt(runtime, |guard| {
            guard.lock_game();
        })
    }

    /// Unlock game
    #[unsafe(no_mangle)]
    pub extern "C" fn game_runtime_unlock(runtime: *mut FfiGameRuntime) {
        if runtime.is_null() { return; }
        Self::do_on_rt(runtime, |guard| {
            guard.unlock_game();
        })
    }

    /// Get game lock status
    #[unsafe(no_mangle)]
    pub extern "C" fn game_runtime_get_lock_status(runtime: *mut FfiGameRuntime) -> bool {
        if runtime.is_null() { return false; }

        let mut locked = false;
        let arc_ptr = unsafe { (*runtime).inner as *const Arc<Mutex<GameRuntime>> };
        let arc_ref = unsafe { Arc::from_raw(arc_ptr) };
        entry_mutex!(*arc_ref, |mutex_guard| {
            locked = mutex_guard.is_game_locked();
        });

        locked
    }

    /// Get button status of player
    #[unsafe(no_mangle)]
    pub extern "C" fn game_runtime_get_button_status(
        runtime: *mut FfiGameRuntime,
        player: *const FfiPlayer,
        key: u8
    ) -> FfiButtonStatus {

        let ffi_player_ref = unsafe { &*player };
        let player = Player::try_from(&*ffi_player_ref).unwrap_or_default();
        let account = player.account;

        let mut status = None;
        let arc_ptr = unsafe { (*runtime).inner as *const Arc<Mutex<GameRuntime>> };
        let arc_ref = unsafe { Arc::from_raw(arc_ptr) };
        entry_mutex!(*arc_ref, |mutex_guard| {
            status = mutex_guard.control.get_button_status(&account, &key);
        });

        match status {
            None => { FfiButtonStatus { found: false, pressed: false, released: false } }
            Some(status) => {
                FfiButtonStatus { found: true, pressed: status, released: !status }
            }
        }
    }

    /// Get axis value of player
    #[unsafe(no_mangle)]
    pub extern "C" fn game_runtime_get_axis(
        runtime: *mut FfiGameRuntime,
        player: *const FfiPlayer,
        key: u8
    ) -> FfiAxis {

        let ffi_player_ref = unsafe { &*player };
        let player = Player::try_from(&*ffi_player_ref).unwrap_or_default();
        let account = player.account;

        let mut status = None;
        let arc_ptr = unsafe { (*runtime).inner as *const Arc<Mutex<GameRuntime>> };
        let arc_ref = unsafe { Arc::from_raw(arc_ptr) };
        entry_mutex!(*arc_ref, |mutex_guard| {
            status = mutex_guard.control.get_axis(&account, &key);
        });

        match status {
            None => { FfiAxis { found: false, axis: 0.0 } }
            Some(status) => {
                FfiAxis { found: true, axis: status }
            }
        }
    }

    /// Get direction value of player
    #[unsafe(no_mangle)]
    pub extern "C" fn game_runtime_get_direction(
        runtime: *mut FfiGameRuntime,
        player: *const FfiPlayer,
        key: u8
    ) -> FfiDirection {

        let ffi_player_ref = unsafe { &*player };
        let player = Player::try_from(&*ffi_player_ref).unwrap_or_default();
        let account = player.account;

        let mut status = None;
        let arc_ptr = unsafe { (*runtime).inner as *const Arc<Mutex<GameRuntime>> };
        let arc_ref = unsafe { Arc::from_raw(arc_ptr) };
        entry_mutex!(*arc_ref, |mutex_guard| {
            status = mutex_guard.control.get_direction(&account, &key);
        });

        match status {
            None => { FfiDirection { found: false, x: 0.0, y: 0.0 } }
            Some(status) => {
                FfiDirection { found: true, x: status.0, y: status.1 }
            }
        }
    }

    /// Get service type of player
    #[unsafe(no_mangle)]
    pub extern "C" fn game_runtime_get_service_type(
        runtime: *mut FfiGameRuntime,
        player: *const FfiPlayer
    ) -> *const FfiServiceType {

        let ffi_player_ref = unsafe { &*player };
        let player = Player::try_from(&*ffi_player_ref).unwrap_or_default();
        let account = player.account;

        let mut service_type = None;
        let arc_ptr = unsafe { (*runtime).inner as *const Arc<Mutex<GameRuntime>> };
        let arc_ref = unsafe { Arc::from_raw(arc_ptr) };
        entry_mutex!(*arc_ref, |mutex_guard| {
            service_type = mutex_guard.data.get_service_type(&account);
        });

        match service_type {
            None => { null() }
            Some(r) => {
                Box::into_raw(Box::new(FfiServiceType::from(&r)))
            }
        }
    }

    /// Is player banned
    #[unsafe(no_mangle)]
    pub extern "C" fn game_runtime_is_player_banned(
        runtime: *mut FfiGameRuntime,
        player: *const FfiPlayer
    ) -> FfiBooleanResult {

        let ffi_player_ref = unsafe { &*player };
        let player = Player::try_from(&*ffi_player_ref).unwrap_or_default();
        let account = player.account;

        let mut banned = None;
        let arc_ptr = unsafe { (*runtime).inner as *const Arc<Mutex<GameRuntime>> };
        let arc_ref = unsafe { Arc::from_raw(arc_ptr) };
        entry_mutex!(*arc_ref, |mutex_guard| {
            banned = Some(mutex_guard.data.is_account_banned(&account));
        });

        match banned {
            None => { FfiBooleanResult { found: false, result: false } }
            Some(r) => {
                FfiBooleanResult { found: false, result: r }
            }
        }
    }

    /// Is player online
    #[unsafe(no_mangle)]
    pub extern "C" fn game_runtime_is_player_online(
        runtime: *mut FfiGameRuntime,
        player: *const FfiPlayer
    ) -> FfiBooleanResult {

        let ffi_player_ref = unsafe { &*player };
        let player = Player::try_from(&*ffi_player_ref).unwrap_or_default();
        let account = player.account;

        let mut online = None;
        let arc_ptr = unsafe { (*runtime).inner as *const Arc<Mutex<GameRuntime>> };
        let arc_ref = unsafe { Arc::from_raw(arc_ptr) };
        entry_mutex!(*arc_ref, |mutex_guard| {
            online = Some(mutex_guard.data.is_account_online(&account));
        });

        match online {
            None => { FfiBooleanResult { found: false, result: false } }
            Some(r) => {
                FfiBooleanResult { found: false, result: r }
            }
        }
    }

    /// Get online list
    #[unsafe(no_mangle)]
    pub extern "C" fn game_runtime_get_online_list(runtime: *mut FfiGameRuntime) -> FfiPlayerList {

        let mut online_list = None;
        let arc_ptr = unsafe { (*runtime).inner as *const Arc<Mutex<GameRuntime>> };
        let arc_ref = unsafe { Arc::from_raw(arc_ptr) };
        entry_mutex!(*arc_ref, |mutex_guard| {
            online_list = Some(mutex_guard.data.online_accounts());
        });

        let mut result : Vec<FfiPlayer> = vec![];
        if online_list.is_some() {
            for online in online_list.unwrap() {
                result.push(FfiPlayer::from(&Player::register_from_hash(online.player_hash)));
            }
        }

        let len = result.len();
        let cap = result.capacity();
        let ptr = result.as_mut_ptr();
        FfiPlayerList {
            players: ptr,
            len,
            cap,
        }
    }

    /// Get banned list
    #[unsafe(no_mangle)]
    pub extern "C" fn game_runtime_get_banned_list(runtime: *mut FfiGameRuntime) -> FfiPlayerList {

        let mut banned_list = None;
        let arc_ptr = unsafe { (*runtime).inner as *const Arc<Mutex<GameRuntime>> };
        let arc_ref = unsafe { Arc::from_raw(arc_ptr) };
        entry_mutex!(*arc_ref, |mutex_guard| {
            banned_list = Some(mutex_guard.data.banned_accounts());
        });

        let mut result : Vec<FfiPlayer> = vec![];
        if banned_list.is_none() {
            for online in banned_list.unwrap() {
                result.push(FfiPlayer::from(&Player::register_from_hash(online.player_hash)));
            }
        }

        let len = result.len();
        let cap = result.capacity();
        let ptr = result.as_mut_ptr();
        FfiPlayerList {
            players: ptr,
            len,
            cap,
        }
    }
}

impl FfiControlEvent {

    /// Free control event
    #[unsafe(no_mangle)]
    pub extern "C" fn free_control_event(event: *mut FfiControlEvent) {
        if event.is_null() {
            return;
        }

        let event_box = unsafe { Box::from_raw(event) };

        let player_ptr = Box::into_raw(Box::new(event_box.player));
        free_player(player_ptr);

        let message_ptr = Box::into_raw(Box::new(event_box.message));
        free_control_message(message_ptr);
    }
}

impl FfiPlayerList {

    /// Free player list
    #[unsafe(no_mangle)]
    pub extern "C" fn free_player_list(list: FfiPlayerList) {
        if !list.players.is_null() {
            let _ = unsafe {
                Vec::from_raw_parts(
                    list.players,
                    list.len,
                    list.cap,
                )
            };
        }
    }
}