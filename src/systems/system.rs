use super::player::Player;
use super::attack::Attack;
use super::attack::TimerStruct;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn game_over(
    player_query: Query<(&Transform, &Sprite, Entity), With<Player>>,
    attack_query: Query<(&Transform, &Sprite, Entity), With<Attack>>,
    timer_query: Query<Entity, With<TimerStruct>>,
    mut commands: Commands
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

                for timer in timer_query.iter() {
                    commands.entity(player).despawn();
                    commands.entity(attack).despawn();
                    commands.entity(timer).despawn();
                }

                let text = Text2dBundle {
                    text: Text::from_section(
                        "Game Over!".to_string(),
                        TextStyle {
                            font_size: 60.,
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
