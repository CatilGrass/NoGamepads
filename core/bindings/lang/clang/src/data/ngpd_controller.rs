use crate::data::ngpd_message::{FfiControlMessage, FfiGameMessage};
use crate::data::ngpd_player::FfiPlayer;
use nogamepads::entry_mutex;
use nogamepads_core::data::controller::controller_data::ControllerData;
use nogamepads_core::data::controller::controller_runtime::ControllerRuntime;
use nogamepads_core::data::message::message_enums::ControlMessage;
use nogamepads_core::data::player::player_data::Player;
use std::ffi::{c_char, c_double, c_void, CStr};
use std::ptr::null_mut;
use std::sync::{Arc, Mutex, MutexGuard};

#[repr(C)]
pub struct FfiControllerData(*mut c_void);

#[repr(C)]
pub struct FfiControllerRuntime {
    pub(crate) inner: *mut c_void,
    drop_fn: extern "C" fn(*mut c_void),
}

impl FfiControllerData {

    /// Create controller data
    #[unsafe(no_mangle)]
    pub extern "C" fn controller_data_new() -> *mut FfiControllerData {
        let controller_data = ControllerData::default();
        let raw = Box::into_raw(Box::new(FfiControllerData(Box::into_raw(Box::new(controller_data)) as *mut _)));
        raw
    }

    /// Bind player to controller
    #[unsafe(no_mangle)]
    pub extern "C" fn controller_data_bind_player(
        controller: *mut FfiControllerData,
        ffi_player: *mut FfiPlayer
    ) {
        if controller.is_null() || ffi_player.is_null() { return; }

        let ffi_player_ref = unsafe { &*ffi_player };

        // Convert FfiPlayer to Player
        let player = Player::try_from(&*ffi_player_ref).unwrap_or_default();

        // Get a mutable reference to the inner ControllerData
        let controller_inner = unsafe { &mut *((*controller).0 as *mut ControllerData) };
        controller_inner.bind_player(player);

        // Release the FFI Player
        drop(unsafe { Box::from_raw(ffi_player) });
    }

    /// Build runtime
    #[unsafe(no_mangle)]
    pub extern "C" fn controller_data_build_runtime(
        controller: *mut FfiControllerData
    ) -> *mut FfiControllerRuntime {
        let controller_inner = unsafe { &mut *((*controller).0 as *mut ControllerData) };
        let arc = controller_inner.runtime_with_borrowed_data();
        let ptr = Arc::into_raw(arc) as *mut c_void;

        let ffi_runtime = Box::new(FfiControllerRuntime {
            inner: ptr,
            drop_fn: Self::drop_controller_runtime,
        });

        Box::into_raw(ffi_runtime)
    }

    extern "C" fn drop_controller_runtime(ptr: *mut c_void) {
        if !ptr.is_null() {
            unsafe {
                let _ = Arc::<Mutex<ControllerRuntime>>::from_raw(ptr as *mut _);
            }
        }
    }

    /// Free ControllerData memory
    #[unsafe(no_mangle)]
    pub extern "C" fn free_controller_data(controller: *mut FfiControllerData) {
        if controller.is_null() {
            return;
        }

        let controller_box = unsafe { Box::from_raw(controller) };
        let raw_data = (*controller_box).0;

        if !raw_data.is_null() {
            unsafe {
                drop(Box::from_raw(raw_data as *mut ControllerData));
            }
        }
    }
}

// ControllerRuntime implementation
impl FfiControllerRuntime {

    fn operate_controller_runtime(
        runtime: *mut FfiControllerRuntime,
        callback: fn(guard: &mut MutexGuard<ControllerRuntime>)
    ) {
        unsafe {
            let ffi_runtime = &*runtime;
            Arc::increment_strong_count(ffi_runtime.inner);
            let arc = Arc::<Mutex<ControllerRuntime>>::from_raw(ffi_runtime.inner as *mut _);
            let arc_clone = Arc::clone(&arc);
            let _ = Arc::into_raw(arc);
            entry_mutex!(arc_clone, |guard| {
                callback(guard);
            });
            Arc::decrement_strong_count(ffi_runtime.inner);
        }
    }

    fn operate_controller_runtime_with_return<Input, Result>(
        runtime: *mut FfiControllerRuntime,
        input: Input,
        callback: fn(guard: &mut MutexGuard<ControllerRuntime>, input: Input) -> Option<Result>
    ) -> Option<Result> {
        unsafe {
            let ffi_runtime = &*runtime;
            Arc::increment_strong_count(ffi_runtime.inner);
            let arc = Arc::<Mutex<ControllerRuntime>>::from_raw(ffi_runtime.inner as *mut _);
            let arc_clone = Arc::clone(&arc);
            let _ = Arc::into_raw(arc);
            let mut result = None;
            entry_mutex!(arc_clone, |guard| {
                result = callback(guard, input);
            });
            Arc::decrement_strong_count(ffi_runtime.inner);
            result
        }
    }

    /// Close runtime and exit game
    #[unsafe(no_mangle)]
    pub extern "C" fn controller_runtime_close(
        runtime: *mut FfiControllerRuntime
    ) {
        if runtime.is_null() {
            return;
        }

        Self::operate_controller_runtime(runtime, |guard| {
            guard.close();
        });
    }

    /// Send control message
    #[unsafe(no_mangle)]
    pub extern "C" fn controller_runtime_send_message(
        runtime: *mut FfiControllerRuntime,
        control_message: *mut FfiControlMessage
    ) {
        if runtime.is_null() {
            return;
        }

        let msg = unsafe { ControlMessage::from(control_message.read()) };

        Self::operate_controller_runtime_with_return(runtime, msg, |guard, input| {
            guard.send_message(input);
            Some(())
        });
    }

    /// Send text message
    #[unsafe(no_mangle)]
    pub extern "C" fn controller_runtime_send_text_message(
        runtime: *mut FfiControllerRuntime,
        message_ptr: *const c_char
    ) {
        if runtime.is_null() || message_ptr.is_null() {
            return;
        }

        let c_str = unsafe { CStr::from_ptr(message_ptr) };
        let text = c_str.to_string_lossy().into_owned();
        let message = FfiControlMessage::from(ControlMessage::Msg(text));

        Self::controller_runtime_send_message(
            runtime,
            Box::into_raw(Box::new(message))
        )
    }

    /// Press a button
    #[unsafe(no_mangle)]
    pub extern "C" fn controller_runtime_press_a_button(
        runtime: *mut FfiControllerRuntime,
        key: u8
    ) {
        if runtime.is_null() {
            return;
        }

        let message = FfiControlMessage::from(
            ControlMessage::Pressed(key)
        );

        Self::controller_runtime_send_message(
            runtime,
            Box::into_raw(Box::new(message))
        )
    }

    /// Release a button
    #[unsafe(no_mangle)]
    pub extern "C" fn controller_runtime_release_a_button(
        runtime: *mut FfiControllerRuntime,
        key: u8
    ) {
        if runtime.is_null() {
            return;
        }

        let message = FfiControlMessage::from(
            ControlMessage::Released(key)
        );

        Self::controller_runtime_send_message(
            runtime,
            Box::into_raw(Box::new(message))
        )
    }

    /// Change axis value
    #[unsafe(no_mangle)]
    pub extern "C" fn controller_runtime_change_axis(
        runtime: *mut FfiControllerRuntime,
        key: u8,
        axis: c_double
    ) {
        if runtime.is_null() {
            return;
        }

        let message = FfiControlMessage::from(
            ControlMessage::Axis(key, axis)
        );

        Self::controller_runtime_send_message(
            runtime,
            Box::into_raw(Box::new(message))
        )
    }

    /// Change direction value
    #[unsafe(no_mangle)]
    pub extern "C" fn controller_runtime_change_direction(
        runtime: *mut FfiControllerRuntime,
        key: u8,
        x: c_double,
        y: c_double
    ) {
        if runtime.is_null() {
            return;
        }

        let message = FfiControlMessage::from(
            ControlMessage::Dir(key, (x, y))
        );

        Self::controller_runtime_send_message(
            runtime,
            Box::into_raw(Box::new(message))
        )
    }

    /// Pop a message from the queue
    #[unsafe(no_mangle)]
    pub extern "C" fn controller_runtime_pop(
        runtime: *mut FfiControllerRuntime
    ) -> *mut FfiGameMessage {
        if runtime.is_null() {
            return null_mut();
        }

        let arc_ptr = unsafe { (*runtime).inner as *const Arc<Mutex<ControllerRuntime>> };
        let arc_ref = unsafe { Arc::from_raw(arc_ptr).clone() };

        let result = {
            Self::operate_controller_runtime_with_return(runtime, (), |guard, _| {
                match guard.pop() {
                    None => { None }
                    Some(msg) => { Some(msg) }
                }
            })
        };

        // Reconstruct Arc
        let _ = Arc::into_raw(arc_ref);

        if let Some(msg) = result {
            // Convert to FFI type
            let ffi_msg = Box::new(FfiGameMessage::from(msg));
            Box::into_raw(ffi_msg)
        } else {
            null_mut()
        }
    }

    /// Free runtime memory
    #[unsafe(no_mangle)]
    pub extern "C" fn free_controller_runtime(runtime: *mut FfiControllerRuntime) {
        if runtime.is_null() {
            return;
        }

        let runtime_box = unsafe { Box::from_raw(runtime) };
        let drop_fn = (*runtime_box).drop_fn;
        let raw = (*runtime_box).inner;

        // Call custom drop function
        drop_fn(raw);
    }
}