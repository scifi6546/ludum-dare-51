use crate::{GameState, PlayerLabel};
use bevy::{
    input::{keyboard::KeyCode, Input},
    prelude::*,
    sprite::MaterialMesh2dBundle,
};
use std::time::Duration;

use bevy_rapier2d::{prelude::*, rapier::dynamics::RigidBodyBuilder};
struct PlayerCollisions {
    entities: Vec<(Entity, Entity)>,
}
pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
            100.0,
        ))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_system_set(
            SystemSet::on_enter(GameState::Game)
                .with_system(spawn_boss)
                .with_system(spawn_player)
                .with_system(insert_spawn),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Game)
                .with_system(input_system)
                .with_system(handle_collision)
                .with_system(tick_spawn),
        )
        .add_system_to_stage(CoreStage::PostUpdate, collision);
    }
}
#[derive(Component, Clone, Copy, PartialEq)]
pub enum CollisionTag {
    Collided,
    NotCollided,
}
#[derive(Component)]
pub struct FuelTag;
#[derive(Bundle, Default)]
struct FuelBundle {}
fn spawn_boss(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let radius = 80.0;
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(radius).into()).into(),
            material: materials.add(ColorMaterial::from(Color::RED)),
            transform: Transform::from_translation(Vec3::new(0.0, -100.0, 0.0)),
            ..default()
        })
        .insert(Collider::ball(radius))
        .insert(ActiveEvents::all())
        .insert(Sensor)
        .insert(FuelTag);
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Box::new(100.0, 100.0, 100.0).into())
                .into(),
            material: materials.add(ColorMaterial::from(Color::GRAY)),
            transform: Transform::from_translation(Vec3::new(0.0, -300.0, 0.0)),
            ..default()
        })
        .insert(Collider::cuboid(50.0, 50.0));
}
fn handle_collision(
    mut commands: Commands,
    mut set: ParamSet<(
        Query<(&mut PlayerFuel, &CollisionTag), ()>,
        Query<(Entity, &CollisionTag), With<FuelTag>>,
        Query<&mut CollisionTag, ()>,
    )>,
) {
    for (mut fuel, tag) in set.p0().iter_mut() {
        if *tag == CollisionTag::Collided {
            fuel.add_fuel(100.0)
        }
    }
    for (entity, tag) in set.p1().iter() {
        if *tag == CollisionTag::Collided {
            commands.entity(entity).despawn();
        }
    }
    for mut tag in set.p2().iter_mut() {
        *tag = CollisionTag::NotCollided
    }
}
fn collision(
    mut commands: Commands,
    mut collision_event: EventReader<CollisionEvent>,
) {
    for collision in collision_event.iter() {
        match collision {
            CollisionEvent::Started(e1, e2, f) => {
                commands.entity(*e1).insert(CollisionTag::Collided);
                commands.entity(*e2).insert(CollisionTag::Collided);
                info!("collision started")
            }
            CollisionEvent::Stopped(e1, e12, f) => info!("ended"),
        }
    }
}
fn input_system(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player_position: Query<
        (&mut Velocity, &mut PlayerFuel),
        With<PlayerLabel>,
    >,
) {
    let player_speed = 1000.0;
    for (mut p, mut fuel) in player_position.iter_mut() {
        let mut req_change = Vec2::new(0.0, 0.0);

        if keyboard_input.pressed(KeyCode::A) {
            req_change.x -= time.delta_seconds() * player_speed;
        }
        if keyboard_input.pressed(KeyCode::D) {
            req_change.x += time.delta_seconds() * player_speed;
        }
        if keyboard_input.pressed(KeyCode::W) {
            req_change.y += time.delta_seconds() * player_speed;
        }
        if keyboard_input.pressed(KeyCode::S) {
            req_change.y -= time.delta_seconds() * player_speed;
        }

        let mag = req_change.length();
        let required = mag.min(fuel.amount);
        let change = required * req_change.normalize();

        if change.is_nan() {
            return;
        }
        p.linvel.x += change.x;
        p.linvel.y += change.y;
        fuel.amount = (fuel.amount - required).max(0.0);
    }
}
#[derive(Component)]
struct PlayerFuel {
    amount: f32,
    max: f32,
}
impl PlayerFuel {
    pub fn add_fuel(&mut self, amount: f32) {
        self.amount = (self.amount + amount).max(self.max)
    }
}
fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(Camera2dBundle::default());
    let radius = 10.0;

    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(radius).into()).into(),
            material: materials.add(ColorMaterial::from(Color::BLUE)),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..default()
        })
        .insert(PlayerLabel)
        .insert(Collider::ball(radius))
        .insert(ActiveEvents::all())
        .insert(RigidBody::Dynamic)
        .insert(Velocity::zero())
        .insert(Restitution::new(1.0))
        .insert(PlayerFuel {
            amount: 100.0,
            max: 100.0,
        });
}
#[derive(Default, Component)]
struct FuelSpawnerLabel;
#[derive(Component)]
struct FuelSpawner {
    timer: Timer,
}
impl FuelSpawner {
    const RESPAWN_TIME_SEC: f32 = 10.0;
}
impl Default for FuelSpawner {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(Self::RESPAWN_TIME_SEC, false),
        }
    }
}
#[derive(Bundle, Default)]
struct FuelSpawnerBundle {
    spawner: FuelSpawner,
    label: FuelSpawnerLabel,
}

fn insert_spawn(mut commands: Commands) {
    commands.spawn_bundle(FuelSpawnerBundle {
        label: FuelSpawnerLabel,
        ..default()
    });
}
fn tick_spawn(
    mut commands: Commands,
    mut query: Query<&mut FuelSpawner, ()>,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let radius = 10.0;
    for mut spawner in query.iter_mut() {
        spawner.timer.tick(time.delta());
        if spawner.timer.finished() {
            spawner.timer.reset();
            commands
                .spawn_bundle(MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(radius).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::RED)),
                    transform: Transform::from_translation(Vec3::new(
                        0.0, -100.0, 0.0,
                    )),
                    ..default()
                })
                .insert(Collider::ball(radius))
                .insert(ActiveEvents::all())
                .insert(Sensor)
                .insert(FuelTag);
        }
    }
}
