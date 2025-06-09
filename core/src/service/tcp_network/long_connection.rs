use crate::data::player::structs::Player;
use crate::service::tcp_network::pad_client::structs::PadClientNetwork;
use crate::service::tcp_network::pad_server::structs::PadServerNetwork;
use std::sync::Arc;
use std::sync::atomic::Ordering::SeqCst;
use log::{error, info, trace, warn};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpStream;
use tokio::spawn;
use nogamepads::entry_mutex;
use crate::data::message::enums::{ControlMessage, GameMessage};
use crate::data::message::enums::ExitReason::GameOver;
use crate::data::message::enums::GameMessage::{End, LetExit};
use crate::data::message::traits::{MessageEncoder, MessageManager};
use crate::service::service_types::ServiceType;
use crate::service::service_types::ServiceType::TCPConnection;

impl PadServerNetwork {

    pub async fn start_long_connection(self: Arc<Self>, player: Player, stream: TcpStream) {
        let (reader, writer) = stream.into_split();
        spawn(Self::read_task(Arc::clone(&self), player.clone(), reader));
        spawn(Self::write_task(Arc::clone(&self), player.clone(), writer));
    }

    async fn read_task(self: Arc<Self>, player: Player, mut reader: OwnedReadHalf) {
        info!("[TCP Server] [Runtime] Reader started.");
        entry_mutex!(self.runtime, |guard| {
            guard.reader_count += 1;
        });

        let mut buffer = [0u8; 1024];
        let mut err_message_counter = 0;

        loop {
            let read = reader.read(&mut buffer).await;
            match read {
                Ok(size) => {
                    let message : ControlMessage = ControlMessage::de(buffer[0..size].to_vec());

                    // Preprocess messages: handle exit messages.
                    match message {
                        ControlMessage::Exit => {
                            info!("[TCP Server] [Runtime] Player {} exited.", player.account.id);
                            entry_mutex!(self.runtime, |guard| {
                                guard.send((player.account.clone(), End), player.account.clone(), ServiceType::TCPConnection);
                            });
                            break;
                        }

                        ControlMessage::Err => {
                            info!("[TCP Server] [Runtime] Received error message from {}.", player.account.id);
                            if err_message_counter < 16 {
                                err_message_counter += 1;
                            } else {
                                warn!("[TCP Server] [Runtime] Too many error messages! Connection closed.");
                                entry_mutex!(self.runtime, |guard| {
                                    guard.send((player.account.clone(), End), player.account.clone(), ServiceType::TCPConnection);
                                });
                                break;
                            }
                        }

                        _ => {}
                    }

                    // Process messages
                    entry_mutex!(self.runtime, |guard| {
                        trace!("[TCP Server] [Runtime] Received: {:?}", &message);
                        guard.put_into_receive_list((player.account.clone(), message), player.account.clone(), ServiceType::TCPConnection);
                    });
                }

                Err(error) => {
                    warn!("[TCP Server] [Runtime] Error reading from socket: {:?}", error);
                    break;
                }
            }

            // Check close
            entry_mutex!(self.runtime, |guard| {
                if guard.data.close.load(SeqCst) {
                    break;
                }
            });
        }

        info!("[TCP Server] [Runtime] Reader between {} closed.", player.account.id);
        entry_mutex!(self.runtime, |guard| {
            guard.send((player.account.clone(), End), player.account.clone(), TCPConnection);
            guard.reader_count -= 1;
        })
    }

    async fn write_task(self: Arc<Self>, player: Player, mut writer: OwnedWriteHalf) {
        info!("[TCP Server] [Runtime] Writer started.");
        entry_mutex!(self.runtime, |guard| {
            guard.writer_count += 1;
        });

        let mut closed = false;

        loop {
            // Check close
            entry_mutex!(self.runtime, |guard| {
                if guard.data.close.load(SeqCst) && !closed {
                    guard.send((player.account.clone(), LetExit(GameOver)), player.account.clone(), ServiceType::TCPConnection);
                    closed = true;
                }
            });

            let mut message = None;
            entry_mutex!(self.runtime, |guard| {
                message = guard.pop_from_send_list(player.account.clone(), ServiceType::TCPConnection);
            });

            if let Some(message) = message {

                // Preprocess messages: handle end messages.
                match message.1 {
                    End => {
                        break;
                    }
                    GameMessage::Err => {
                        continue;
                    }
                    _ => {}
                }

                // Process messages
                match writer.write_all(GameMessage::en(&message.1).as_slice()).await {
                    Ok(_) => {
                        trace!("[TCP Server] [Runtime] Sent {:?} to {}", &message.1, player.account.id);
                        writer.flush().await.expect("[TCP Client] [Runtime] Writer encountered an error");
                    }
                    Err(error) => {
                        warn!("[TCP Server] [Runtime] Sent {:?} to {} failed: {}", &message.1, player.account.id, error);
                        break;
                    }
                }
            }
        }

        info!("[TCP Server] [Runtime] Writer between {} closed.", player.account.id);
        entry_mutex!(self.runtime, |guard| {
            guard.data.sign_player_online_status(&player, TCPConnection, false);
            guard.writer_count -= 1;
        })
    }
}

impl PadClientNetwork {

    pub async fn start_long_connection(self: Arc<Self>, stream: TcpStream) {
        let (reader, writer) = stream.into_split();
        spawn(Self::read_task(Arc::clone(&self), reader));
        spawn(Self::write_task(Arc::clone(&self), writer));
    }

    async fn read_task(self: Arc<Self>, mut reader: OwnedReadHalf) {
        info!("[TCP Client] [Runtime] Reader started.");

        let mut buffer = [0u8; 1024];
        let mut err_message_counter = 0;
        loop {
            // Check close
            entry_mutex!(self.runtime, |guard| {
                if guard.close.load(SeqCst) {
                    break;
                }
            });

            let read = reader.read(&mut buffer).await;
            match read {
                Ok(size) => {
                    let message : GameMessage = GameMessage::de(buffer[0..size].to_vec());

                    // Preprocess messages: handle exit messages.
                    match message {
                        LetExit(reason) => {
                            info!("[TCP Client] [Runtime] Server let you exit: {:?}", reason);
                            entry_mutex!(self.runtime, |guard| {
                                guard.send(ControlMessage::Exit, 0, TCPConnection);
                                guard.send(ControlMessage::End, 0, TCPConnection);
                            });
                            break;
                        }

                        GameMessage::Err => {
                            info!("[TCP Client] [Runtime] Received error message from server.");
                            if err_message_counter < 16 {
                                err_message_counter += 1;
                            } else {
                                warn!("[TCP Client] [Runtime] Too many error messages! Connection closed.");
                                entry_mutex!(self.runtime, |guard| {
                                    guard.send(ControlMessage::End, 0, TCPConnection);
                                });
                                break;
                            }
                        }

                        _ => {}
                    }

                    // Process messages
                    entry_mutex!(self.runtime, |guard| {
                        trace!("[TCP Client] [Runtime] Received: {:?}", &message);
                        guard.put_into_receive_list(message, 0, TCPConnection);
                    });
                }
                Err(err) => {
                    error!("[TCP Client] [Runtime] Reader encountered an error: {}", err);
                    break;
                }
            }
        }

        info!("[TCP Client] [Runtime] Reader closed.");
    }

    async fn write_task(self: Arc<Self>, mut writer: OwnedWriteHalf) {
        info!("[TCP Client] [Runtime] Writer started.");

        let mut closed = false;

        loop {
            // Check close
            if !closed {
                entry_mutex!(self.runtime, |guard| {
                    if guard.close.load(SeqCst) {
                        guard.send(ControlMessage::Exit, 0, TCPConnection);
                        guard.send(ControlMessage::End, 0, TCPConnection);
                        closed = true;
                    }
                });
            }

            let mut message = None;
            entry_mutex!(self.runtime, |guard| {
                message = guard.pop_from_send_list(0, TCPConnection);
            });

            if let Some(message) = message {

                // Preprocess messages: handle exit messages.
                match message {
                    ControlMessage::End => { break; }
                    ControlMessage::Err => {
                        continue;
                    }
                    _ => {}
                }

                // Process messages
                match writer.write_all(ControlMessage::en(&message).as_slice()).await {
                    Ok(_) => {
                        trace!("[TCP Client] [Runtime] Sent {:?}.", &message);
                        writer.flush().await.expect("[TCP Client] [Runtime] Writer encountered an error");
                    }
                    Err(error) => {
                        warn!("[TCP Client] [Runtime] Sent {:?} failed: {}", &message, error);
                        break;
                    }
                }
            }
        }

        info!("[TCP Client] [Runtime] Writer closed.");
        entry_mutex!(self.runtime, |guard| {
            guard.close();
        })
    }
}