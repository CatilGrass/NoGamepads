mod plugin_client;

use crate::plugin_client::PadClientPlugin;
use bevy::prelude::{App};
use bevy::DefaultPlugins;
use bevy_tokio_tasks::TokioTasksPlugin;
use tokio::runtime::Builder;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);
    app.add_plugins(TokioTasksPlugin {
        make_runtime: Box::new(|| {
            Builder::new_multi_thread().enable_all().build().unwrap()
        }),
        ..TokioTasksPlugin::default()
    });

    app.add_plugins(PadClientPlugin);

    app.run();
}
