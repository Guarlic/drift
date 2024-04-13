use super::player::Player;
use super::attack::Attack;
use bevy::prelude::*;
use bevy::app::AppExit;
use bevy::sprite::collide_aabb::collide;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn game_over(
    mut exit_events: ResMut<Events<AppExit>>,
    player_query: Query<(&mut Transform, &Sprite), With<Player>>,
    attack_query: Query<(&mut Transform, &Sprite), With<Attack>>
) {
    exit_events.send(AppExit);
}
