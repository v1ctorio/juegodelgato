#![windows_subsystem = "windows"]
use bevy::{prelude::*, window::WindowTheme};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Gato".into(),
                window_theme: Some(WindowTheme::Dark),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, sprite_movement)
        .run();
}

#[derive(Component)]
enum Direction {
    Down,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("gato2.png"),
            transform: Transform::from_xyz(0., 1000., 0.),
        
            ..default()
        },
        Direction::Down,
    ));
}

fn sprite_movement(time: Res<Time>, mut sprite_position: Query<(&mut Direction, &mut Transform)>) {
    for (mut gato, mut transform) in &mut sprite_position {
        match *gato {
            Direction::Down => transform.translation.y -= 150. * time.delta_seconds(),
        }
            *gato = Direction::Down;
    }
}