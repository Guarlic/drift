use crate::SCORE;
use super::player::Player;
use super::attack::*;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

#[derive(Component)]
pub struct Camera;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), Camera));
}

pub fn spawn_grid(mut commands: Commands, asset_server: Res<AssetServer>) {
    let grid = SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(1920., 1080.)),
            ..default()
        },
        texture: asset_server.load("grid.png"),
        transform: Transform::from_xyz(0., 0., -1.),
        ..default()
    };

    commands.spawn(grid);
}

pub fn game_over(
    player_query: Query<(&Transform, &Sprite, Entity), With<Player>>,
    attack_query: Query<(&Transform, &Sprite, Entity), With<Attack>>,
    preview_query: Query<Entity, With<Preview>>,
    timer_query: Query<Entity, With<TimerStruct>>,
    query: Query<Entity, Without<Camera>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    for (player_transform, player_sprite, player) in player_query.iter() {
        for (attack_transform, attack_sprite, attack) in attack_query.iter() {
            let collision = collide(
                player_transform.translation,
                player_sprite.custom_size.unwrap(),
                attack_transform.translation,
                attack_sprite.custom_size.unwrap()
            );

            if collision.is_some() {
                println!("Game Over!");

                commands.entity(player).despawn();
                commands.entity(attack).despawn();

                for timer in timer_query.iter() {
                    commands.entity(timer).despawn();
                }

                for preview in preview_query.iter() {
                    commands.entity(preview).despawn();
                }

                unsafe {
                    let text = Text2dBundle {
                        text: Text::from_section(
                            format!("Game Over!\nScore: {}", SCORE),
                            TextStyle {
                                font_size: 100.,
                                font: asset_server.load("font.ttf"),
                                color: Color::rgb(1., 1., 0.),
                                ..default()
                            },
                        ),
                        ..default()
                    };

                    commands.spawn(text);
                }
            }
        }
    }
}
