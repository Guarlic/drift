use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

pub fn spawn_player(mut commands: Commands) {
    let player = SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(50., 50.)),
            ..default()
        },
        transform: Transform::from_xyz(0., 0., 0.),
        ..default()
    };

    commands.spawn((player, Player));
}

pub fn move_player(
    keys: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>
) {
    for mut player_transform in player_query.iter_mut() {
        const MOVEMENT_SIZE: f32 = 100.;

        let movements = [
            (KeyCode::Up, Vec3::new(0., MOVEMENT_SIZE, 0.)),
            (KeyCode::Down, Vec3::new(0., -MOVEMENT_SIZE, 0.)),
            (KeyCode::Left, Vec3::new(-MOVEMENT_SIZE, 0., 0.)),
            (KeyCode::Right, Vec3::new(MOVEMENT_SIZE, 0., 0.)),
        ];

        for (key, direction) in movements.iter() {
            if keys.just_pressed(*key) {
                const CLAMP_X: f32 = 900.;
                const CLAMP_Y: f32 = 500.;
                
                player_transform.translation += *direction;

                player_transform.translation.x = player_transform.translation.x.clamp(-CLAMP_X, CLAMP_X);
                player_transform.translation.y = player_transform.translation.y.clamp(-CLAMP_Y, CLAMP_Y);
            }
        }
    }
}
