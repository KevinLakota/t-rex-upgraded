use bevy::prelude::*;

use crate::constants::*;

#[derive(Component)]
pub struct Background;

#[derive(Component)]
pub struct GroundVisual;

pub fn spawn_background(
    commands: &mut Commands,
    asset_server: &AssetServer,
    visible_width: f32,
) {
    let texture = asset_server.load("background.png");

    let total_width = visible_width + BACKGROUND_WIDTH * 2.0;
    let tiles_needed = (total_width / BACKGROUND_WIDTH).ceil() as i32;

    let start_x = -visible_width / 2.0 - BACKGROUND_WIDTH;

    for i in 0..tiles_needed {
        let x = start_x + BACKGROUND_WIDTH * 0.5 + i as f32 * BACKGROUND_WIDTH;

        commands.spawn((
            Sprite {
                image: texture.clone(),
                custom_size: Some(Vec2::new(BACKGROUND_WIDTH, BACKGROUND_HEIGHT)),
                ..default()
            },
            Transform::from_xyz(x, BACKGROUND_Y, -20.0),
            Background,
        ));
    }
}

pub fn spawn_ground_visual(
    commands: &mut Commands,
    visible_width: f32,
) {
    commands.spawn((
        Sprite::from_color(
            Color::srgb(0.35, 0.35, 0.35),
            Vec2::new(visible_width + 200.0, 50.0),
        ),
        Transform::from_xyz(0.0, GROUND_Y - 20.0, -5.0),
        GroundVisual,
    ));
}

pub fn move_background(
    mut query: Query<&mut Transform, With<Background>>,
    time: Res<Time>,
) {
    for mut transform in &mut query {
        transform.translation.x -= BACKGROUND_SPEED * time.delta_secs();
    }
}

pub fn loop_background(
    mut query: Query<&mut Transform, With<Background>>,
    window: Single<&Window, With<bevy::window::PrimaryWindow>>,
) {
    let visible_left = -window.width() / 2.0;

    let rightmost_x = query
        .iter()
        .map(|t| t.translation.x)
        .fold(f32::NEG_INFINITY, f32::max);

    for mut transform in &mut query {
        let right_edge = transform.translation.x + BACKGROUND_WIDTH / 2.0;

        if right_edge < visible_left - BACKGROUND_WIDTH {
            transform.translation.x = rightmost_x + BACKGROUND_WIDTH;
        }
    }
}