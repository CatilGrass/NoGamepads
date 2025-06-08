use std::pin::Pin;
use crate::service::tcp_network::utils::tokio_utils::build_tokio_runtime;
use tokio::spawn;

#[macro_export]
#[allow(unused_macros)]
macro_rules! run_services {
    ($($service:expr),+ $(,)?) => {
        nogamepads_core::service::service_runner::ServiceRunner::run(Vec::from([$($service),+]));
    };
}

pub type NoGamepadsService = Pin<Box<dyn Future<Output = ()> + Send>>;

pub struct ServiceRunner;

impl ServiceRunner {
    pub fn run(futures: Vec<NoGamepadsService>) {
        let runtime = build_tokio_runtime("nogamepads".to_string());
        runtime.block_on(async {
            let mut handles = Vec::new();
            for fut in futures {
                handles.push(spawn(fut));
            }

            for handle in handles {
                handle.await.expect("Task panicked");
            }
        });
    }
}