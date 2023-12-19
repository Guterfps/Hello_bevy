
use bevy::prelude::{App, DefaultPlugins};
use crate::hello_plug::HelloPlugin;

mod hello_plug;

fn main() {
    App::new()
    .add_plugins((DefaultPlugins, HelloPlugin))
    .run();
}
