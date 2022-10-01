use crate::GameState;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Game).with_system(spawn_boss),
        );
    }
}
fn spawn_boss(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(10.0).into()).into(),
        material: materials.add(ColorMaterial::from(Color::RED)),
        transform: Transform::from_translation(Vec3::new(100.0, 0.0, 0.0)),
        ..default()
    });
}
