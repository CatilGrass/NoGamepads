use std::net::{IpAddr, SocketAddr};
use std::sync::{Arc, Mutex};
use std::sync::atomic::Ordering::SeqCst;
use std::time::Duration;
use log::{error, info, warn};
use tokio::{join, spawn};
use tokio::time::sleep;
use nogamepads::entry_mutex;
use crate::data::controller::controller_runtime::ControllerRuntime;
use crate::data::message::message_enums::ConnectionMessage::{Join, RequestGameInfos};
use crate::data::message::message_enums::ConnectionResponseMessage;
use crate::service::service_runner::NoGamepadsService;
use crate::service::service_types::ServiceType;
use crate::service::tcp_network::DEFAULT_PORT;
use crate::service::tcp_network::utils::stream_utils::{read_msg, send_msg};
use crate::service::tcp_network::utils::tokio_utils::build_tokio_runtime;

pub struct PadClientNetwork {
    pub(crate) addr: SocketAddr,
    pub(crate) runtime: Arc<Mutex<ControllerRuntime>>
}

macro_rules! connect_once {
    ($addr:expr, |$conn:ident| $code:block) => {{
        use tokio::net::TcpStream;
        match TcpStream::connect($addr).await {
            Ok(mut $conn) => {
                $code
                true
            },
            Err(e) => {
                error!("[TCP Client] [Main] Connection failed {:?}", e);
                false
            }
        }
    }}
}

impl PadClientNetwork {

    pub fn build(runtime: Arc<Mutex<ControllerRuntime>>) -> PadClientNetwork {
        entry_mutex!(runtime, |guard| {
            guard.service_type = ServiceType::TCPConnection;
        });

        PadClientNetwork {
            addr: SocketAddr::from(([127, 0, 0, 1], DEFAULT_PORT)),
            runtime
        }
    }

    pub fn bind_addr(&mut self, addr: SocketAddr) -> &mut PadClientNetwork {
        self.addr = addr;
        self
    }

    pub fn bind_ip(&mut self, addr: IpAddr) -> &mut PadClientNetwork {
        self.addr.set_ip(addr);
        self
    }

    pub fn bind_port(&mut self, port: u16) -> &mut PadClientNetwork {
        self.addr.set_port(port);
        self
    }

    pub fn build_entry(self) -> NoGamepadsService {
        let arc = Arc::new(self);

        let entry = async move {
            // Connection thread: Download the relevant resources, verify connection eligibility, and attempt to join the game.
            let connection_thread = spawn({
                let client = Arc::clone(&arc);
                async move {
                    Self::connection_thread(client).await
                }
            });

            // Join
            let _ = join!(connection_thread);
        };

        Box::pin(entry)
    }

    pub fn connect(self) {
        let runtime = build_tokio_runtime("padclient_tcp".to_string());
        runtime.block_on(self.build_entry());
    }
}

impl PadClientNetwork {

    async fn connection_thread(self: Arc<PadClientNetwork>) {
        info!("[TCP Client] Connecting to {}:{}", self.addr.ip().to_string(), self.addr.port());

        let mut buffer = [0; 1024];

        // Requests game infos
        if !connect_once!(self.addr, |stream| {
            info!("[TCP Client] [Main] Requesting game infos.");
            send_msg(&mut stream, RequestGameInfos).await;
            let response : ConnectionResponseMessage = read_msg(&mut buffer, &mut stream).await;
            match response {
                ConnectionResponseMessage::GameInfos(infos) => {
                    entry_mutex!(self.runtime, |guard| {
                        guard.game_info = infos;
                    });
                    info!("[TCP Client] [Main] Download game infos successfully.");
                }
                ConnectionResponseMessage::Err => {
                    warn!("[TCP Client] [Main] Download game infos failed.");
                }
                _ => {
                    warn!("[TCP Client] [Main] Not found game infos.");
                }
            }
        }) {
            return;
        }

        // TODO :: Download game layouts

        // TODO :: Download skin assets

        // Try to join game
        let _ = connect_once!(self.addr, |connection| {
            let mut player = None;
            entry_mutex!(self.runtime, |guard| {
                player = Some(guard.player.clone());
            });
            if player.is_some() {
                info!("[TCP Client] [Main] Trying to join game.");
                send_msg(&mut connection, Join(player.unwrap())).await;
                let response : ConnectionResponseMessage = read_msg(&mut buffer, &mut connection).await;
                match response {
                    ConnectionResponseMessage::Welcome => {

                        // Long Connection
                        info!("[TCP Client] [Main] Welcome");
                        spawn(Self::start_long_connection(Arc::clone(&self), connection));
                    }
                    ConnectionResponseMessage::Deny(why) => {
                        error!("[TCP Client] [Main] Connection denied: {:?}", why);
                    }
                    _ => { }
                }
            } else {
                error!("[TCP Client] [Main] No player found.");
                return;
            }
        });

        loop {
            sleep(Duration::from_millis(1000)).await;
            entry_mutex!(self.runtime, |guard| {
                if guard.close.load(SeqCst) {
                    break;
                }
            })
        }

        info!("[TCP Client] [Main] Main thread closed.");
    }
}