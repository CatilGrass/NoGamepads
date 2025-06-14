use std::ffi::{c_char, c_double, c_int, CStr, CString};
use std::ptr;
use nogamepads_core::data::player::player_data::{Account, Customize, Player};

#[repr(C)]
pub struct FfiAccount {
    id: *mut c_char,
    player_hash: *mut c_char,
}

#[repr(C)]
pub struct FfiCustomize {
    nickname: *mut c_char,
    color_hue: c_int,
    color_saturation: c_double,
    color_value: c_double,
}

#[repr(C)]
pub struct FfiPlayer {
    account: FfiAccount,
    customize: *mut FfiCustomize,
}

impl From<Account> for FfiAccount {
    fn from(acc: Account) -> Self {
        FfiAccount {
            id: CString::new(acc.id).unwrap().into_raw(),
            player_hash: CString::new(acc.player_hash).unwrap().into_raw(),
        }
    }
}

impl From<&Player> for FfiPlayer {
    fn from(player: &Player) -> Self {
        let account = FfiAccount::from(player.account.clone());
        let customize = player.clone().customize.map(|c| {
            Box::into_raw(Box::new(FfiCustomize {
                nickname: CString::new(c.nickname).unwrap().into_raw(),
                color_hue: c.color_hue,
                color_saturation: c.color_saturation,
                color_value: c.color_value,
            }))
        }).unwrap_or(ptr::null_mut());

        FfiPlayer { account, customize }
    }
}

impl TryFrom<&FfiPlayer> for Player {
    type Error = ();

    fn try_from(ffi: &FfiPlayer) -> Result<Self, Self::Error> {
        let account = Account {
            id: unsafe { CStr::from_ptr(ffi.account.id) }.to_str().map_err(|_| ())?.to_owned(),
            player_hash: unsafe { CStr::from_ptr(ffi.account.player_hash) }.to_str().map_err(|_| ())?.to_owned(),
        };

        let customize = if !ffi.customize.is_null() {
            let c = unsafe { &*ffi.customize };
            Some(Customize {
                nickname: unsafe { CStr::from_ptr(c.nickname) }.to_str().map_err(|_| ())?.to_owned(),
                color_hue: c.color_hue,
                color_saturation: c.color_saturation,
                color_value: c.color_value,
            })
        } else {
            None
        };

        Ok(Player { account, customize })
    }
}

/// Register a player
#[unsafe(no_mangle)]
pub extern "C" fn player_register(id: *const c_char, password: *const c_char) -> *mut FfiPlayer {
    let id_str = unsafe { CStr::from_ptr(id) }.to_string_lossy();
    let pass_str = unsafe { CStr::from_ptr(password) }.to_string_lossy();

    let player = Player::register(id_str.into_owned(), pass_str.into_owned());
    Box::into_raw(Box::new(FfiPlayer::from(&player)))
}

/// Check if the player's password is correct
#[unsafe(no_mangle)]
pub extern "C" fn player_check(player: *const FfiPlayer, password: *const c_char) -> bool {
    let player = unsafe { &*player };
    let pass_str = unsafe { CStr::from_ptr(password) }.to_string_lossy();

    Player::try_from(player)
        .map(|p| p.check(pass_str.into_owned()))
        .unwrap_or(false)
}

/// Set the player's nickname
#[unsafe(no_mangle)]
pub extern "C" fn player_set_nickname(player: *mut FfiPlayer, nickname: *const c_char) {
    let nickname_str = unsafe { CStr::from_ptr(nickname) }.to_string_lossy().into_owned();
    let mut rust_player : Player = unsafe { (&*player).try_into().unwrap() };

    rust_player.nickname(&nickname_str);
    *unsafe { &mut *player } = FfiPlayer::from(&rust_player);
}

/// Set the player's hue
#[unsafe(no_mangle)]
pub extern "C" fn player_set_hue(player: *mut FfiPlayer, hue: c_int) {
    let mut rust_player : Player = unsafe { (&*player).try_into().unwrap() };

    rust_player.hue(hue);
    *unsafe { &mut *player } = FfiPlayer::from(&rust_player);
}

/// Set the player's HSV color
#[unsafe(no_mangle)]
pub extern "C" fn player_set_hsv(
    player: *mut FfiPlayer, hue: c_int, saturation: c_double, value: c_double
) {
    let mut rust_player : Player = unsafe { (&*player).try_into().unwrap() };

    rust_player.hsv(hue, saturation, value);
    *unsafe { &mut *player } = FfiPlayer::from(&rust_player);
}

/// Free the player
#[unsafe(no_mangle)]
pub extern "C" fn free_player(player: *mut FfiPlayer) {
    if player.is_null() { return; }

    unsafe {
        let player = &mut *player;

        if !player.account.id.is_null() {
            drop(CString::from_raw(player.account.id));
            player.account.id = ptr::null_mut();
        }

        if !player.account.player_hash.is_null() {
            drop(CString::from_raw(player.account.player_hash));
            player.account.player_hash = ptr::null_mut();
        }

        if !player.customize.is_null() {
            let customize = &mut *player.customize;
            if !customize.nickname.is_null() {
                drop(CString::from_raw(customize.nickname));
            }
            player.customize = ptr::null_mut();
        }

        drop(Box::from_raw(player));
    }
}