use bevy::prelude::*;

pub const CANVAS_SIZE: Vec2 = Vec2::new(480., 270.);
pub const PLAYER_SIZE: f32 = 25.0;
const PIPE_SIZE: Vec2 = Vec2::new(32., CANVAS_SIZE.y);
const GAP_SIZE: f32 = 100.0;

pub struct PipePlugin;

impl Plugin for PipePlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(
      FixedUpdate,
      spawn_pipes.run_if(run_once),
    );
  }
}

#[derive(Component)]
pub struct Pipe;

#[derive(Component)]
pub struct PipeTop;

#[derive(Component)]
pub struct PipeBottom;

#[derive(Component)]
pub struct PointsGate;

fn spawn_pipes(mut commands: Commands,
asset_server: Res<AssetServer>,) {
  commands.spawn((
    Sprite {
      image: asset_server.load("pipe.png"),
      custom_size: Some(Vec2::new(32., 160.)),
      image_mode: SpriteImageMode::Sliced(
        TextureSlicer {
          border: BorderRect::axes(8., 19.),
          center_scale_mode: SliceScaleMode::Stretch,
          ..default()
        }
      ),
      ..default()
    },
    Transform::from_xyz(0., 0., 1.),
  ));

}
