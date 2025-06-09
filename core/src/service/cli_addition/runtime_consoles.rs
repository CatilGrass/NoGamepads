use crate::service::cli_addition::utils::read_cli;
use clap::{Command, FromArgMatches};
use std::sync::{Arc, Mutex};
use log::info;
use tokio::{join, spawn};
use crate::service::service_runner::NoGamepadsService;

pub struct RuntimeConsole<PadService, Cmd>
where Cmd: FromArgMatches {
    command: Command,
    prefix: String,
    service: Arc<Mutex<PadService>>,
    process_command: fn(Arc<Mutex<PadService>>, Cmd) -> bool,
}

impl<PadService: Send + 'static, Cmd: FromArgMatches + 'static> RuntimeConsole<PadService, Cmd> {
    pub fn build(command: Command,
                 prefix: String,
                 service: Arc<Mutex<PadService>>,
                 process_command: fn(Arc<Mutex<PadService>>, Cmd) -> bool,
    ) -> RuntimeConsole<PadService, Cmd> {
        RuntimeConsole { command, prefix, service, process_command }
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

        loop {
            let option : Option<Cmd> = read_cli(
                format!("{}> ", prefix_uppercase),
                &prefix_lowercase,
                self.command.clone()
            ).await;
            match option {
                None => {}
                Some(cmd) => {
                    if !(self.process_command)(Arc::clone(&self.service), cmd) {
                        break;
                    }
                }
            }
        }
        info!("[Console] Shutdown.")
    }
}