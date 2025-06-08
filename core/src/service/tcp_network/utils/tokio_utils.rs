use tokio::runtime::{Builder, Runtime};

pub fn build_tokio_runtime(name: String) -> Runtime {
    Builder::new_multi_thread()
        .thread_name(name)
        .thread_stack_size(32 * 1024 * 1024)
        .enable_all()
        .build()
        .unwrap()
}