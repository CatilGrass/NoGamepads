use bevy::prelude::{App};
use bevy::DefaultPlugins;
use bevy_tokio_tasks::TokioTasksPlugin;
use tokio::runtime::Builder;
use nogamepads_client::bevy_plugins::plugin_client_app::ClientAppPlugins;

fn main() {
    let mut app = App::new();

    // Default
    app.add_plugins(DefaultPlugins);

    // Bevy Tokio Tasks
    app.add_plugins(TokioTasksPlugin {
        make_runtime: Box::new(|| Builder::new_multi_thread().enable_all().build().unwrap()),
        ..TokioTasksPlugin::default()
    });

    // Client App Plugins
    app.add_plugins(ClientAppPlugins);

    app.run();
}
