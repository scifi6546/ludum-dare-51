use super::{GameEntity, GameState};
use crate::loading::TextureAssets;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;
use std::time::Duration;
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Game).with_system(spawn_player),
        )
        .insert_resource(MaxScore::default())
        .insert_resource(MaxScore::default())
        .add_system_set(
            SystemSet::on_update(GameState::Game)
                .with_system(input_system)
                .with_system(player_cooling)
                .with_system(update_score)
                .with_system(update_camera_height),
        );
    }
}
pub struct MaxScore {
    pub score: PlayerScore,
}
impl Default for MaxScore {
    fn default() -> Self {
        Self {
            score: PlayerScore::new(),
        }
    }
}
#[derive(Component)]
pub struct PlayerCamera;
#[derive(Component)]
pub struct PlayerScore {
    max_score: f32,
}

impl PlayerScore {
    pub fn set_score(&mut self, new_score: f32) {
        self.max_score = new_score.max(self.max_score);
    }
    pub fn get_score(&self) -> f32 {
        self.max_score
    }
    pub fn new() -> Self {
        Self { max_score: 0.0 }
    }
}
#[derive(Component)]
pub struct PlayerLabel;
#[derive(Component)]
pub struct PlayerCoolingTimer {
    pub timer: Timer,
}
impl PlayerCoolingTimer {
    pub const COOLING_TIME: Duration = Duration::from_secs(10);
    pub fn new() -> Self {
        Self {
            timer: Timer::new(Self::COOLING_TIME, false),
        }
    }
    /// gets cooling used (goes from 0.0 to 1.0
    pub fn get_frac_used(&self) -> f32 {
        1.0 - self.timer.percent_left()
    }
    ///resets cooling
    pub fn refill_cooling(&mut self) {
        self.timer.reset()
    }
}
#[derive(Component)]
pub struct PlayerFuel {
    amount: f32,
    max: f32,
}
impl PlayerFuel {
    pub fn new(max: f32) -> Self {
        Self { amount: max, max }
    }
    pub fn get_fuel(&self) -> f32 {
        self.amount
    }
    /// gets percent of fuel left in range 0.0 to 1.0
    pub fn get_fuel_ratio_left(&self) -> f32 {
        (self.amount) / self.max
    }
    pub fn set_fuel(&mut self, amount: f32) {
        self.amount = amount.max(0.0).min(self.max);
    }

    pub fn add_fuel(&mut self, amount: f32) {
        self.amount = (self.amount + amount).max(self.max)
    }
}
fn player_cooling(
    mut player_query: Query<&mut PlayerCoolingTimer, ()>,
    time: Res<Time>,
    mut game_state: ResMut<State<GameState>>,
) {
    for mut player in player_query.iter_mut() {
        player.timer.tick(time.delta());
        if player.timer.finished() {
            game_state.set(GameState::GameOver).unwrap()
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
        let required = mag.min(fuel.get_fuel());
        let change = required * req_change.normalize();

        if change.is_nan() {
            return;
        }
        p.linvel.x += change.x;
        p.linvel.y += change.y;
        let fuel_amount = fuel.get_fuel();
        fuel.set_fuel((fuel_amount - required).max(0.0));
    }
}
fn update_camera_height(
    mut queries: ParamSet<(
        Query<&Transform, With<PlayerLabel>>,
        Query<&mut Transform, With<PlayerCamera>>,
    )>,
) {
    let player_transform_query = queries.p0();
    let player_y = player_transform_query.iter().next().clone();
    if player_y.is_none() {
        error!("player not found");
        return;
    }
    let player_y = player_y.unwrap().translation.y;
    for mut camera_transform in queries.p1().iter_mut() {
        let camera_y = camera_transform.translation.y.max(player_y);
        camera_transform.translation.y = camera_y;
    }
}
fn update_score(
    mut player_query: Query<(&Transform, &mut PlayerScore), With<PlayerLabel>>,
    mut score: ResMut<MaxScore>,
) {
    for (transform, mut current_score) in player_query.iter_mut() {
        current_score.set_score(transform.translation.y);
        score.score.set_score(transform.translation.y);
    }
}
fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    textures: Res<TextureAssets>,
) {
    let mut transform = Transform::from_translation(Vec3::new(0.0, 0.0, 0.0));
    transform.scale = Vec3::new(4.0, 4.0, 4.0);
    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(GameEntity)
        .insert(PlayerCamera);
    let radius = 10.0;
    commands
        .spawn_bundle(SpriteBundle {
            texture: textures.spaceship.clone(),
            transform,
            ..default()
        })
        .insert(PlayerLabel)
        .insert(Collider::ball(radius))
        .insert(ActiveEvents::all())
        .insert(RigidBody::Dynamic)
        .insert(Velocity::zero())
        .insert(Restitution::new(1.0))
        .insert(PlayerFuel::new(1000.0))
        .insert(PlayerCoolingTimer::new())
        .insert(GameEntity)
        .insert(PlayerScore::new());
}
