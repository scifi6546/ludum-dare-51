mod player;
mod ui;

use crate::{loading::TextureAssets, GameState};
use bevy::{
    input::{keyboard::KeyCode, Input},
    prelude::*,
    sprite::MaterialMesh2dBundle,
};
use bevy_rapier2d::{prelude::*, rapier::dynamics::RigidBodyBuilder};
use bevy_turborand::{prelude::*, *};
pub use player::{
    MaxScore, PlayerCoolingTimer, PlayerFuel, PlayerLabel, PlayerScore,
};
use std::time::Duration;

struct PlayerCollisions {
    entities: Vec<(Entity, Entity)>,
}
#[derive(Component)]
pub struct GameEntity;
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
            100.0,
        ))
        .insert_resource(FuelCurrentlySpawned::new(1000))
        .add_plugin(ui::UiPlugin)
        .add_plugin(RngPlugin::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(player::PlayerPlugin)
        .add_system_set(
            SystemSet::on_enter(GameState::Game)
                .with_system(spawn_scene)
                .with_system(insert_spawn),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Game)
                .with_system(handle_collision)
                .with_system(tick_spawn)
                .with_system(clean_up_fuel),
        )
        .add_system_to_stage(CoreStage::PostUpdate, collision)
        .add_system_set(
            SystemSet::on_exit(GameState::Game)
                .with_system(despawn_entity)
                .with_system(reset_fuel_spawn),
        );
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
fn spawn_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    textures: Res<TextureAssets>,
) {
    let mut transform =
        Transform::from_translation(Vec3::new(0.0, -300.0, 0.0));
    transform.scale = Vec3::new(4.0, 4.0, 4.0);
    commands
        .spawn_bundle(SpriteBundle {
            texture: textures.grass.clone(),
            transform,
            ..default()
        })
        .insert(Collider::cuboid(50.0, 50.0))
        .insert(GameEntity);
}
fn despawn_entity(
    mut commands: Commands,
    entities: Query<Entity, With<GameEntity>>,
) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
fn handle_collision(
    mut commands: Commands,
    mut set: ParamSet<(
        Query<(&mut PlayerFuel, &mut PlayerCoolingTimer, &CollisionTag), ()>,
        Query<(Entity, &CollisionTag), With<FuelTag>>,
        Query<&mut CollisionTag, ()>,
    )>,
) {
    for (mut fuel, mut player_cooling, tag) in set.p0().iter_mut() {
        if *tag == CollisionTag::Collided {
            fuel.add_fuel(100.0);
            player_cooling.refill_cooling();
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

#[derive(Default, Component)]
struct FuelSpawnerLabel;
#[derive(Component)]
struct FuelSpawner {
    timer: Timer,
}
impl FuelSpawner {
    const RESPAWN_TIME_SEC: f32 = 1.0;
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
    rng: RngComponent,
}
fn spawn_fn(x: f32) -> f32 {
    1.0 / x
}
const FUEL_RADIUS: f32 = 10.0;
fn insert_spawn(
    mut commands: Commands,
    mut global_rng: ResMut<GlobalRng>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut fuel_spawn: ResMut<FuelCurrentlySpawned>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    textures: Res<TextureAssets>,
) {
    commands.spawn_bundle(FuelSpawnerBundle {
        label: FuelSpawnerLabel,
        rng: RngComponent::from(&mut global_rng),
        ..default()
    });
}
struct FuelCurrentlySpawned {
    spawned: u32,
    max_spawn: u32,
}
impl FuelCurrentlySpawned {
    pub fn new(max_spawn: u32) -> Self {
        Self {
            spawned: 0,
            max_spawn,
        }
    }
}
fn tick_spawn(
    mut commands: Commands,
    mut query: Query<(&mut FuelSpawner, &mut RngComponent), ()>,
    player_query: Query<&Transform, With<PlayerLabel>>,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut fuel_spawn: ResMut<FuelCurrentlySpawned>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    textures: Res<TextureAssets>,
) {
    let player_transform = player_query.iter().next();
    if player_transform.is_none() {
        error!("player does not exist");
        return;
    }
    let player_transform = player_transform.unwrap();
    for (mut spawner, mut rng) in query.iter_mut() {
        if fuel_spawn.spawned >= fuel_spawn.max_spawn {
            return;
        }
        fuel_spawn.spawned += 1;
        spawner.timer.tick(time.delta());
        if spawner.timer.finished() {
            let x = 500.0 * rng.f32_normalized();

            let y =
                1000.0 * rng.f32_normalized() + player_transform.translation.y;
            let next_spawn = rng.f32()
                * player_transform.translation.y.max(100.0).ln()
                / 100.0;
            info!("next spawn: {}", next_spawn);
            let mut transform =
                Transform::from_translation(Vec3::new(x, y, 0.0));
            transform.scale = Vec3::new(4.0, 4.0, 4.0);
            spawner.timer.reset();
            spawner
                .timer
                .set_duration(Duration::from_secs_f32(next_spawn));
            commands
                .spawn_bundle(SpriteBundle {
                    texture: textures.fuel.clone(),
                    transform,
                    ..default()
                })
                .insert(Collider::ball(FUEL_RADIUS))
                .insert(ActiveEvents::all())
                .insert(Sensor)
                .insert(FuelTag)
                .insert(GameEntity);
        }
    }
}
const FUEL_MAX_Y_DIST: f32 = 1000.0;
fn clean_up_fuel(
    mut commands: Commands,
    mut p_set: ParamSet<(
        Query<(Entity, &Transform), With<FuelTag>>,
        Query<&Transform, With<PlayerLabel>>,
    )>,
    mut fuel_spawn: ResMut<FuelCurrentlySpawned>,
) {
    let player_query = p_set.p1();
    let player_transform = player_query.iter().next();
    if player_transform.is_none() {
        error!("player not found");
        return;
    }
    let player_transform = player_transform.unwrap();
    let player_y = player_transform.translation.y;
    let fuel_query = p_set.p0();
    for (entity, fuel_transform) in fuel_query.iter() {
        let fuel_y = fuel_transform.translation.y;
        if (fuel_y - player_y).abs() > FUEL_MAX_Y_DIST {
            commands.entity(entity).despawn_recursive();
            fuel_spawn.spawned -= 1;
        }
    }
}
fn reset_fuel_spawn(mut counter: ResMut<FuelCurrentlySpawned>) {
    counter.spawned = 0;
}
