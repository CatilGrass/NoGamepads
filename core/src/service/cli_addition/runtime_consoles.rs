use std::process::exit;
use crate::service::cli_addition::utils::read_cli;
use clap::{Command, FromArgMatches};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use log::{info, warn};
use tokio::{count, join, select, spawn};
use tokio::signal::ctrl_c;
use tokio::time::sleep;
use crate::service::service_runner::NoGamepadsService;

pub struct RuntimeConsole<PadService, Cmd> where Cmd: FromArgMatches {
    command: Command,
    prefix: String,
    service: Arc<Mutex<PadService>>,
    process_command: fn(Arc<Mutex<PadService>>, Cmd) -> bool,
    check_close: fn(rt: Arc<Mutex<PadService>>) -> bool,
    close: fn(rt: Arc<Mutex<PadService>>),
}

impl<PadService, Cmd> RuntimeConsole<PadService, Cmd>
where PadService: Send + 'static, Cmd: FromArgMatches + 'static {
    pub fn build(command: Command,
                 prefix: String,
                 service: Arc<Mutex<PadService>>,
                 process_command: fn(Arc<Mutex<PadService>>, Cmd) -> bool,
                 check_close: fn(rt: Arc<Mutex<PadService>>) -> bool,
                 close: fn(rt: Arc<Mutex<PadService>>),
    ) -> RuntimeConsole<PadService, Cmd> {
        RuntimeConsole { command, prefix, service, process_command, check_close, close }
    }

    pub fn build_entry(self) -> NoGamepadsService {
        let arc = Arc::new(self);
        let entry = async move {
            let console_main = spawn({
                let console = Arc::clone(&arc);
                async move {
                    Self::console_main(console).await
                }
            });

            // Join
            let _ = join!(console_main);
        };

        Box::pin(entry)
    }

    async fn console_main(self: Arc<RuntimeConsole<PadService, Cmd>>) {
        let prefix_uppercase = self.prefix.to_uppercase();
        let prefix_lowercase = self.prefix.to_lowercase();
        let mut confirm_close = false;

        loop {
            select! {
                // Auto check
                _ = Self::auto_check_close(self.clone()) => { break; },

                // Ctrl + C
                _ = ctrl_c() => {
                    if !confirm_close {
                        warn!("To close the process, press Ctrl + C again.");
                        confirm_close = true;
                    } else {
                        (self.close)(self.service.clone());
                        break;
                    }
                }

                // Command Line
                option = read_cli(
                    format!("{}> ", prefix_uppercase),
                    &prefix_lowercase,
                    self.command.clone()
                ) => {
                    match option {
                        None => {}
                        Some(cmd) => {
                            confirm_close = false;
                            if !(self.process_command)(Arc::clone(&self.service), cmd) { break; }
                        }
                    }
                }
            }
        }
        info!("[Console] Shutdown.")
    }

    async fn auto_check_close(self: Arc<RuntimeConsole<PadService, Cmd>>) {
        loop {
            sleep(Duration::from_secs(1)).await;
            if (self.check_close)(Arc::clone(&self.service)) { break; }
        }
    }
}