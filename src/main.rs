use bevy::prelude::*;
use bevy_template::prelude::*;

fn main() {
    App::new().add_plugins((CorePlugins, ClientPlugins)).run();
}
