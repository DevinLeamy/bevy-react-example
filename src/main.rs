use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 800.,
            height: 800.,
            title: "I am a window!".to_string(),
            #[cfg(target_arch = "wasm32")]
            canvas: Some(String::from(".game")),
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.9, 0.3, 0.3)))
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}
