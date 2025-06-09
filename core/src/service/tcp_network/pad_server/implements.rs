use std::net::{IpAddr, SocketAddr};
use std::sync::{Arc, Mutex};
use std::sync::atomic::Ordering::SeqCst;
use std::time::Duration;
use log::{error, info, trace, warn};
use tokio::{join, select, spawn};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::watch::{channel};
use tokio::time::sleep;
use nogamepads::entry_mutex;
use crate::data::game::runtime::structs::GameRuntime;
use crate::data::message::enums::ConnectionMessage;
use crate::data::message::enums::ConnectionMessage::{Join, RequestGameInfos, RequestLayoutConfigure, RequestSkinPackage, Ready};
use crate::data::message::enums::ConnectionResponseMessage::{Deny, GameInfos, Welcome};
use crate::service::service_runner::NoGamepadsService;
use crate::service::tcp_network::DEFAULT_PORT;
use crate::service::tcp_network::pad_server::structs::PadServerNetwork;
use crate::service::tcp_network::utils::stream_utils::{get_target_address, read_msg, send_msg};
use crate::service::tcp_network::utils::tokio_utils::build_tokio_runtime;

impl PadServerNetwork {

    pub fn build(runtime: Arc<Mutex<GameRuntime>>) -> PadServerNetwork {
        let (close_tx, close_rx) = channel(false);
        PadServerNetwork {
            addr: SocketAddr::from(([127, 0, 0, 1], DEFAULT_PORT)),
            runtime,
            close_tx,
            close_rx
        }
    }

    pub fn bind_ip(&mut self, ip: IpAddr) -> &mut PadServerNetwork {
        self.addr.set_ip(ip);
        self
    }

    pub fn bind_port(&mut self, port: u16) -> &mut PadServerNetwork {
        self.addr.set_port(port);
        self
    }

    pub fn build_entry(self) -> NoGamepadsService {
        let arc = Arc::new(self);

        let entry = async move {
            // Main thread: Used to handle connection requests, data requests, and transfer skin assets
            let main_thread = spawn({
                let server = Arc::clone(&arc);
                async move {
                    Self::main_thread(server).await
                }
            });

            let close_checker = {
                let server = Arc::clone(&arc);
                async move {
                    Self::close_checker(server).await
                }
            };

            // Join
            let _ = join!(close_checker, main_thread);
        };

        Box::pin(entry)
    }

    pub fn listening_block_on(self) {
        let runtime = build_tokio_runtime("padserver_tcp".to_string());

        info!("[TCP Server] Server start.");
        runtime.block_on(self.build_entry());
        info!("[TCP Server] Finished.");
    }
}

impl PadServerNetwork {

    async fn main_thread(self: Arc<PadServerNetwork>) {

        info!("[TCP Server] [Main] Server listening at {}", self.addr.to_string());

        let listener = TcpListener::bind(self.addr).await;
        if listener.is_err() {
            error!("[TCP Server] [Main] Failed to bind to {}", self.addr.to_string());
            return;
        }

        let listener = listener.unwrap();
        info!("[TCP Server] [Main] Listener created, start listening.");

        let mut local_close_rx = self.close_rx.clone();

        loop {
            select! {
                _ = local_close_rx.changed() => {
                    if *local_close_rx.borrow() {
                        break;
                    }
                }

                accept = listener.accept() => {
                    match accept {
                        Ok((stream, _)) => {
                            spawn(Self::process_connection(Arc::clone(&self), stream));
                        }
                        Err(error) => {
                            warn!("[TCP Server] [Main] Failed to accept TCP connections: {}", error);
                        }
                    }
                }
            }
        }

        info!("[TCP Server] [Main] Main thread closed.");
    }

    async fn process_connection(self: Arc<Self>, mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        let message: ConnectionMessage = read_msg(&mut buffer, &mut stream).await;
        let from_address = get_target_address(&stream);

        match message {

            Join(player) => {
                trace!("[TCP Server] [Main] Trying to join Player \"{}\"", &player.account.id);
                let mut result = Ok(());
                entry_mutex!(self.runtime, |guard| {
                    match guard.try_join_player(player.clone()) {
                        Ok(_) => { result = Ok(()); }
                        Err(why) => { result = Err(why); }
                    }
                });
                if result.is_err() {
                    let fail_message = result.unwrap_err();
                    error!("[TCP Server] [Main] Player join failed: {:?}", &fail_message);
                    send_msg(&mut stream, Deny(fail_message)).await;
                } else {

                    // Long Connection
                    info!("[TCP Server] [Main] Player joined, begin long connection.");
                    send_msg(&mut stream, Welcome).await;
                    spawn(Self::start_long_connection(Arc::clone(&self), player, stream));
                }
            }

            RequestGameInfos => {
                info!("[TCP Server] [Main] Client({}) requests game infos.", from_address);
                let mut info = Default::default();
                entry_mutex!(self.runtime, |guard| {
                    info = guard.info.clone();
                });
                send_msg(&mut stream, GameInfos(info)).await;
                info!("[TCP Server] [Main] Game infos sent.");
            }

            RequestLayoutConfigure => {
                info!("[TCP Server] [Main] Client({}) requests layout configures.", from_address);
            }

            RequestSkinPackage => {
                info!("[TCP Server] [Main] Client({}) requests to download skin package.", from_address);
            }

            Ready => {
                info!("[TCP Server] [Main] Client({}) is ready!", from_address);
                warn!("[TCP Server] [Main] But I don't know who he is.....")
            }

            _ => { }
        }
    }

    async fn close_checker(self: Arc<Self>) {
        loop {
            sleep(Duration::from_millis(1000)).await;
            entry_mutex!(self.runtime, |guard| {
                if guard.data.close.load(SeqCst) {
                    let _ = self.close_tx.send(true);
                    break;
                }
            })
        }
    }
}