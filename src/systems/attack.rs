use bevy::prelude::*;
use rand::Rng;

static mut ATTACK_SIZE: f32 = 0.;
static mut ATTACK_TRANSFORM: Transform = Transform::from_xyz(0., 0., 0.);

#[derive(Component)]
pub struct Attack;

#[derive(Component)]
pub struct Preview;

#[derive(Component)]
pub struct TimerStruct(pub(crate) Timer);

#[derive(Component)]
pub struct PreviewTimer;

#[derive(Component)]
pub struct AttackTimer;

#[derive(Component)]
pub struct DespawnTimer;

pub fn spawn_timers(mut commands: Commands) {
    commands.spawn((
        TimerStruct(Timer::from_seconds(5., TimerMode::Repeating)),
        PreviewTimer
    ));

    commands.spawn((
        TimerStruct(Timer::from_seconds(6., TimerMode::Repeating)),
        AttackTimer
    ));

    commands.spawn((
        TimerStruct(Timer::from_seconds(7., TimerMode::Repeating)),
        DespawnTimer
    ));
}

pub fn spawn_preview(
    time: Res<Time>,
    mut timer_query: Query<&mut TimerStruct, With<PreviewTimer>>,
    mut commands: Commands
) {
    let mut random = rand::thread_rng();

    let x = random.gen_range(-700..=700) as f32;
    let y = random.gen_range(-300..=300) as f32;

    for mut timer in timer_query.iter_mut() {
        if timer.0.tick(time.delta()).just_finished() {
            unsafe {
                ATTACK_TRANSFORM = Transform::from_xyz(x, y, 0.);
                ATTACK_SIZE = random.gen_range(600..=900) as f32;

                let attack_preview = SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(ATTACK_SIZE, ATTACK_SIZE)),
                        color: Color::rgba(0.9, 0.6, 0.6, 0.3),
                        ..default()
                    },
                    transform: ATTACK_TRANSFORM,
                    ..default()
                };

                commands.spawn((attack_preview.clone(), Preview));
            }
        }
    }
}

pub fn spawn_attack(
    time: Res<Time>,
    mut timer_query: Query<&mut TimerStruct, With<AttackTimer>>,
    preview_query: Query<Entity, With<Preview>>,
    mut commands: Commands
) {
    for mut timer in timer_query.iter_mut() {
        if timer.0.tick(time.delta()).just_finished() {
            for preview in preview_query.iter() {
                commands.entity(preview).despawn();

                unsafe {
                    let attack = SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(ATTACK_SIZE, ATTACK_SIZE)),
                            color: Color::rgb(0.9, 0.6, 0.6),
                            ..default()
                        },
                        transform: ATTACK_TRANSFORM,
                        ..default()
                    };

                    commands.spawn((attack.clone(), Attack));
                }
            }
        }
    }
}

pub fn despawn_attack(
    time: Res<Time>,
    mut timer_query: Query<&mut TimerStruct, With<DespawnTimer>>,
    attack_query: Query<Entity, With<Attack>>,
    mut commands: Commands
) {
    for mut timer in timer_query.iter_mut() {
        if timer.0.tick(time.delta()).just_finished() {
            for attack in attack_query.iter() {
                commands.entity(attack).despawn();
            }
        }
    }
}
