use crate::data::ngpd_controller::FfiControllerRuntime;
use crate::data::ngpd_game::FfiGameRuntime;
use nogamepads_core::service::tcp_network::pad_client::pad_client_service::PadClientNetwork;
use nogamepads_core::service::tcp_network::pad_server::pad_server_service::PadServerNetwork;
use nogamepads_core::service::tcp_network::utils::tokio_utils::build_tokio_runtime;
use std::ffi::{c_char, c_void, CStr};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::ptr::null_mut;
use std::sync::{Arc, Mutex};
use nogamepads_core::data::controller::controller_runtime::ControllerRuntime;
use nogamepads_core::data::game::game_runtime::GameRuntime;

#[repr(C)]
pub struct FfiTcpClientService(*mut c_void);

#[repr(C)]
pub struct FfiTcpServerService(*mut c_void);

impl FfiTcpClientService {

    /// Build tcp client
    #[unsafe(no_mangle)]
    pub extern "C" fn tcp_client_build(
        runtime: *mut FfiControllerRuntime,
    ) -> *mut FfiTcpClientService {

        if runtime.is_null() {
            return null_mut();
        }

        let runtime_ref = unsafe { &*runtime };
        let data_ptr = runtime_ref.inner as *const Mutex<ControllerRuntime>;

        let arc = unsafe {
            let atomic_arc = Arc::from_raw(data_ptr);
            Arc::clone(&atomic_arc)
        };

        let client = PadClientNetwork::build(arc.clone());

        let client_box = Box::new(client);
        let service = FfiTcpClientService(Box::into_raw(client_box) as *mut _);
        Box::into_raw(Box::new(service))
    }

    /// Bind ipv4 address
    #[unsafe(no_mangle)]
    pub extern "C" fn tcp_client_bind_ipv4(
        service: *mut FfiTcpClientService,
        a0: u8,
        a1: u8,
        a2: u8,
        a3: u8
    ) {
        if service.is_null() { return; }

        let inner = unsafe { &mut *((*service).0 as *mut PadClientNetwork) };
        let ip = IpAddr::V4(Ipv4Addr::new(a0, a1, a2, a3));
        inner.bind_ip(ip);
    }

    /// Bind ipv6 address
    #[unsafe(no_mangle)]
    pub extern "C" fn tcp_client_bind_ipv6(
        service: *mut FfiTcpClientService,
        ip_str: *const c_char
    ) -> bool {

        if service.is_null() { return false; }

        if ip_str.is_null() {
            return false;
        }

        let inner = unsafe { &mut *((*service).0 as *mut PadClientNetwork) };

        let c_str = unsafe { CStr::from_ptr(ip_str) };
        if let Ok(ip_str) = c_str.to_str() {
            if let Ok(ipv6) = ip_str.parse::<Ipv6Addr>() {
                inner.bind_ip(IpAddr::V6(ipv6));
                return true;
            }
        }

        false
    }

    /// Bind port
    #[unsafe(no_mangle)]
    pub extern "C" fn tcp_client_bind_port(
        service: *mut FfiTcpClientService,
        port: u16
    ) {
        if service.is_null() { return; }

        let inner = unsafe { &mut *((*service).0 as *mut PadClientNetwork) };
        inner.bind_port(port);
    }

    /// Bind address with ipv4
    #[unsafe(no_mangle)]
    pub extern "C" fn tcp_client_bind_address_v4(
        service: *mut FfiTcpClientService,
        a0: u8,
        a1: u8,
        a2: u8,
        a3: u8,
        port: u16
    ) {
        if service.is_null() { return; }

        let inner = unsafe { &mut *((*service).0 as *mut PadClientNetwork) };
        let addr = SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(a0, a1, a2, a3)),
            port
        );
        inner.bind_ip(addr.ip());
        inner.bind_port(addr.port());
    }

    /// Bind address with ipv6
    #[unsafe(no_mangle)]
    pub extern "C" fn tcp_client_bind_address_v6(
        service: *mut FfiTcpClientService,
        ip_str: *const c_char,
        port: u16
    ) -> bool {

        if ip_str.is_null() || service.is_null() {
            return false;
        }

        let inner = unsafe { &mut *((*service).0 as *mut PadClientNetwork) };

        let c_str = unsafe { CStr::from_ptr(ip_str) };
        if let Ok(ip_str) = c_str.to_str() {
            if let Ok(ipv6) = ip_str.parse::<Ipv6Addr>() {
                let addr = SocketAddr::new(
                    IpAddr::V6(ipv6),
                    port
                );
                inner.bind_ip(addr.ip());
                inner.bind_port(addr.port());
                return true;
            }
        }

        false
    }

    /// Connect
    #[unsafe(no_mangle)]
    pub extern "C" fn tcp_client_connect(
        service: *mut FfiTcpClientService
    ) {
        if service.is_null() { return; }

        let inner = unsafe { &mut *((*service).0 as *mut PadClientNetwork) };
        let service = unsafe { Box::from_raw(inner) };

        let rt = build_tokio_runtime("nogamepads-c-tcp".to_string());
        rt.block_on(service.build_entry());
    }

    /// Free tcp client
    #[unsafe(no_mangle)]
    pub extern "C" fn free_tcp_client(
        service: *mut FfiTcpClientService
    ) {
        if service.is_null() {
            return;
        }

        let wrapper = unsafe { Box::from_raw(service) };
        let client_ptr = wrapper.0;

        unsafe {
            if !client_ptr.is_null() {
                let _ = Box::from_raw(client_ptr as *mut PadClientNetwork);
            }
        }
    }
}

impl FfiTcpServerService {

    /// Build tcp server
    #[unsafe(no_mangle)]
    pub extern "C" fn tcp_server_build(
        runtime: *mut FfiGameRuntime,
    ) -> *mut FfiTcpServerService {

        if runtime.is_null() {
            return null_mut();
        }

        let runtime_ref = unsafe { &*runtime };
        let data_ptr = runtime_ref.inner as *const Mutex<GameRuntime>;

        let arc = unsafe {
            let atomic_arc = Arc::from_raw(data_ptr);
            Arc::clone(&atomic_arc)
        };

        let server = PadServerNetwork::build(arc.clone());

        let server_box = Box::new(server);
        let service = FfiTcpServerService(Box::into_raw(server_box) as *mut _);
        Box::into_raw(Box::new(service))
    }

    /// Bind ipv4 address
    #[unsafe(no_mangle)]
    pub extern "C" fn tcp_server_bind_ipv4(
        service: *mut FfiTcpServerService,
        a0: u8,
        a1: u8,
        a2: u8,
        a3: u8
    ) {
        if service.is_null() { return; }

        let inner = unsafe { &mut *((*service).0 as *mut PadServerNetwork) };
        let ip = IpAddr::V4(Ipv4Addr::new(a0, a1, a2, a3));
        inner.bind_ip(ip);
    }

    /// Bind ipv6 address
    #[unsafe(no_mangle)]
    pub extern "C" fn tcp_server_bind_ipv6(
        service: *mut FfiTcpServerService,
        ip_str: *const c_char
    ) -> bool {
        if ip_str.is_null() || service.is_null() { return false; }

        let inner = unsafe { &mut *((*service).0 as *mut PadServerNetwork) };

        let c_str = unsafe { CStr::from_ptr(ip_str) };
        if let Ok(ip_str) = c_str.to_str() {
            if let Ok(ipv6) = ip_str.parse::<Ipv6Addr>() {
                inner.bind_ip(IpAddr::V6(ipv6));
                return true;
            }
        }

        false
    }

    /// Bind port
    #[unsafe(no_mangle)]
    pub extern "C" fn tcp_server_bind_port(
        service: *mut FfiTcpServerService,
        port: u16
    ) {
        if service.is_null() { return; }

        let inner = unsafe { &mut *((*service).0 as *mut PadServerNetwork) };
        inner.bind_port(port);
    }

    /// Bind address with ipv4
    #[unsafe(no_mangle)]
    pub extern "C" fn tcp_server_bind_address_v4(
        service: *mut FfiTcpServerService,
        a0: u8,
        a1: u8,
        a2: u8,
        a3: u8,
        port: u16
    ) {
        if service.is_null() { return; }

        let inner = unsafe { &mut *((*service).0 as *mut PadServerNetwork) };
        let addr = SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(a0, a1, a2, a3)),
            port
        );
        inner.bind_ip(addr.ip());
        inner.bind_port(addr.port());
    }

    /// Bind address with ipv6
    #[unsafe(no_mangle)]
    pub extern "C" fn tcp_server_bind_address_v6(
        service: *mut FfiTcpServerService,
        ip_str: *const c_char,
        port: u16
    ) -> bool {
        if ip_str.is_null() || service.is_null() { return false; }

        let inner = unsafe { &mut *((*service).0 as *mut PadServerNetwork) };

        let c_str = unsafe { CStr::from_ptr(ip_str) };
        if let Ok(ip_str) = c_str.to_str() {
            if let Ok(ipv6) = ip_str.parse::<Ipv6Addr>() {
                let addr = SocketAddr::new(
                    IpAddr::V6(ipv6),
                    port
                );
                inner.bind_ip(addr.ip());
                inner.bind_port(addr.port());
                return true;
            }
        }

        false
    }

    /// Start listening
    #[unsafe(no_mangle)]
    pub extern "C" fn tcp_server_listening_block_on(
        service: *mut FfiTcpServerService
    ) {
        if service.is_null() { return; }

        let inner = unsafe { &mut *((*service).0 as *mut PadServerNetwork) };
        let service = unsafe { Box::from_raw(inner) };

        let rt = build_tokio_runtime("nogamepads-c-tcp".to_string());
        rt.block_on(service.build_entry());
    }

    /// Free tcp server
    #[unsafe(no_mangle)]
    pub extern "C" fn free_tcp_server(
        service: *mut FfiTcpServerService
    ) {
        if !service.is_null() {
            unsafe {
                let service = Box::from_raw(service);

                if !service.0.is_null() {
                    Arc::from_raw(service.0 as *mut _);
                }
            }
        }
    }
}