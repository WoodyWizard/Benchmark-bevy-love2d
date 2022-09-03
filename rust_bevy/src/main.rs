use bevy::{prelude::* , window::PresentMode};
use std::{thread, time};
use rand::Rng;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

fn main() {
    App::new()
        .insert_resource(WindowDescriptor{
            width: 1200.,
            height: 700.,
            present_mode: PresentMode::AutoNoVsync,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_system(sprite_movement)
        .run();
}

#[derive(Component)]
enum Direction {
    Up,
    Down,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut rng = rand::thread_rng();

    commands.spawn_bundle(Camera2dBundle::default());
    for i in 0..10000 {

        commands.spawn_bundle(SpriteBundle {
            texture: asset_server.load("bush.png"),
            transform: Transform::from_xyz(rng.gen_range(-600.0..600.0), rng.gen_range(-300.0..300.0), 0.),
            ..default()
        }).insert(Direction::Up);

    }
}



fn sprite_movement(time: Res<Time>, mut sprite_position: Query<(&mut Direction, &mut Transform)>) {
    for (mut logo, mut transform) in &mut sprite_position {

        match *logo {
            Direction::Up => transform.translation.y += 150. * time.delta_seconds(),
            Direction::Down => transform.translation.y -= 150. * time.delta_seconds(),
        }

        if transform.translation.y > 300. {
            *logo = Direction::Down;
        } else if transform.translation.y < -300. {
            *logo = Direction::Up;
        }
    }
}
