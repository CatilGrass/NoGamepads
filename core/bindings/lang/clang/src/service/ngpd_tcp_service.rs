use std::ffi::{c_char, c_void, CStr};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::sync::{Arc, Mutex};
use nogamepads_core::data::controller::controller_runtime::ControllerRuntime;
use nogamepads_core::data::game::game_runtime::GameRuntime;
use nogamepads_core::service::tcp_network::pad_client::pad_client_service::PadClientNetwork;
use nogamepads_core::service::tcp_network::pad_server::pad_server_service::PadServerNetwork;
use crate::data::ngpd_controller::FfiControllerRuntime;
use crate::data::ngpd_game::FfiGameRuntime;

#[repr(C)]
pub struct FfiTcpClientService(*mut c_void);

#[repr(C)]
pub struct FfiTcpServerService(*mut c_void);

impl FfiTcpClientService {

    fn as_inner(&self) -> &mut PadClientNetwork {
        unsafe { &mut *(self.0 as *mut PadClientNetwork) }
    }

    /// Build tcp client
    #[unsafe(no_mangle)]
    pub extern "C" fn tcp_client_build(
        runtime: *mut FfiControllerRuntime,
    ) -> FfiTcpClientService {

        let arc_ptr = unsafe { (*runtime).inner as *const Arc<Mutex<ControllerRuntime>> };
        let arc_clone: Arc<Mutex<ControllerRuntime>> = unsafe { (&*arc_ptr).clone() };

        let service = Box::new(PadClientNetwork::build(arc_clone));
        let ptr = Box::into_raw(service);

        FfiTcpClientService(ptr as *mut c_void)
    }

    /// Bind ipv4 address
    #[unsafe(no_mangle)]
    pub extern "C" fn tcp_client_bind_ipv4(
        service: FfiTcpClientService,
        a0: u8,
        a1: u8,
        a2: u8,
        a3: u8
    ) {
        let inner = service.as_inner();
        let ip = IpAddr::V4(Ipv4Addr::new(a0, a1, a2, a3));
        inner.bind_ip(ip);
    }

    /// Bind ipv6 address
    #[unsafe(no_mangle)]
    pub extern "C" fn tcp_client_bind_ipv6(
        service: FfiTcpClientService,
        ip_str: *const c_char
    ) -> bool {
        if ip_str.is_null() {
            return false;
        }

        let inner = service.as_inner();

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
        service: FfiTcpClientService,
        port: u16
    ) {
        service.as_inner().bind_port(port);
    }

    /// Bind address with ipv4
    #[unsafe(no_mangle)]
    pub extern "C" fn tcp_client_bind_address_v4(
        service: FfiTcpClientService,
        a0: u8,
        a1: u8,
        a2: u8,
        a3: u8,
        port: u16
    ) {
        let inner = service.as_inner();
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
        service: FfiTcpClientService,
        ip_str: *const c_char,
        port: u16
    ) -> bool {
        if ip_str.is_null() {
            return false;
        }

        let inner = service.as_inner();

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
        service: FfiTcpClientService
    ) {
        let service_ptr = service.0 as *mut PadClientNetwork;
        assert!(!service_ptr.is_null(), "Service pointer is null");

        let service = unsafe { Box::from_raw(service_ptr) };

        service.connect();
    }

    /// Free tcp client
    #[unsafe(no_mangle)]
    pub extern "C" fn free_tcp_client(
        service: FfiTcpClientService
    ) {
        if service.0.is_null() {
            return;
        }

        let service_ptr = service.0 as *mut PadClientNetwork;

        unsafe {
            let _ = Box::from_raw(service_ptr);
        }
    }
}

impl FfiTcpServerService {

    fn as_inner(&self) -> &mut PadServerNetwork {
        unsafe { &mut *(self.0 as *mut PadServerNetwork) }
    }

    /// Build tcp server
    #[unsafe(no_mangle)]
    pub extern "C" fn tcp_server_build(
        runtime: *mut FfiGameRuntime,
    ) -> FfiTcpServerService {

        let arc_ptr = unsafe { (*runtime).inner as *const Arc<Mutex<GameRuntime>> };
        let arc_clone: Arc<Mutex<GameRuntime>> = unsafe { (&*arc_ptr).clone() };

        let service = Box::new(PadServerNetwork::build(arc_clone));
        let ptr = Box::into_raw(service);

        FfiTcpServerService(ptr as *mut c_void)
    }

    /// Bind ipv4 address
    #[unsafe(no_mangle)]
    pub extern "C" fn tcp_server_bind_ipv4(
        service: FfiTcpServerService,
        a0: u8,
        a1: u8,
        a2: u8,
        a3: u8
    ) {
        let inner = service.as_inner();
        let ip = IpAddr::V4(Ipv4Addr::new(a0, a1, a2, a3));
        inner.bind_ip(ip);
    }

    /// Bind ipv6 address
    #[unsafe(no_mangle)]
    pub extern "C" fn tcp_server_bind_ipv6(
        service: FfiTcpServerService,
        ip_str: *const c_char
    ) -> bool {
        if ip_str.is_null() {
            return false;
        }

        let inner = service.as_inner();

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
        service: FfiTcpServerService,
        port: u16
    ) {
        service.as_inner().bind_port(port);
    }

    /// Bind address with ipv4
    #[unsafe(no_mangle)]
    pub extern "C" fn tcp_server_bind_address_v4(
        service: FfiTcpServerService,
        a0: u8,
        a1: u8,
        a2: u8,
        a3: u8,
        port: u16
    ) {
        let inner = service.as_inner();
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
        service: FfiTcpServerService,
        ip_str: *const c_char,
        port: u16
    ) -> bool {
        if ip_str.is_null() {
            return false;
        }

        let inner = service.as_inner();

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
        service: FfiTcpServerService
    ) {
        let service_ptr = service.0 as *mut PadServerNetwork;
        assert!(!service_ptr.is_null(), "Service pointer is null");

        let service = unsafe { Box::from_raw(service_ptr) };

        service.listening_block_on();
    }

    /// Free tcp server
    #[unsafe(no_mangle)]
    pub extern "C" fn free_tcp_server(
        service: FfiTcpServerService
    ) {
        if service.0.is_null() {
            return;
        }

        let service_ptr = service.0 as *mut PadServerNetwork;

        unsafe {
            let _ = Box::from_raw(service_ptr);
        }
    }
}