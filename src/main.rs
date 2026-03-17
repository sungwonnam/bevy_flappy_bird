use bevy::{camera::ScalingMode, image::ImageSamplerDescriptor, prelude::*};

use flappy_bird::*;

#[derive(Component)]
#[require(Gravity(1000.), Velocity)]
struct Player;

#[derive(Component)]
struct Gravity(f32);

#[derive(Component, Default)]
struct Velocity(f32);


fn main() {
  App::new()
    .add_plugins(DefaultPlugins.set(ImagePlugin {
      default_sampler: ImageSamplerDescriptor::nearest(),
    }))
    .add_plugins(PipePlugin)
    .add_systems(Startup, startup)
    .add_systems(Update, bird_control)
    .add_systems(FixedUpdate, (gravity, check_in_bounds))
    .add_observer(respawn_on_endgame)
    .run();
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
  commands.spawn((
    Camera2d,
    Projection::Orthographic(OrthographicProjection {
      scaling_mode: ScalingMode::AutoMax {
        max_width: CANVAS_SIZE.x,
        max_height: CANVAS_SIZE.y,
      },
      ..OrthographicProjection::default_2d()
    }),
  ));

  commands.spawn((
    Sprite {
      custom_size: Some(Vec2::splat(PLAYER_SIZE)),
      image: asset_server.load("bevy-bird.png"),
      ..default()
    },
    Transform::from_xyz(-CANVAS_SIZE.x / 4., 0., 1.),
    Player,
  ));
}

fn gravity(
  mut transforms: Query<(&mut Transform, &mut Velocity, &Gravity)>,
  time: Res<Time>,
) {
  for (mut transform, mut velocity, gravity) in &mut transforms {
    velocity.0 -= gravity.0 * time.delta_secs();
    transform.translation.y += velocity.0 * time.delta_secs();
  }
}

fn check_in_bounds(
  player: Single<&Transform, With<Player>>,
  mut commands: Commands,
) {
  info!("player y pos: {}", player.translation.y);
  if player.translation.y < -CANVAS_SIZE.y / 2.0 - PLAYER_SIZE
    || player.translation.y > CANVAS_SIZE.y / 2.0 + PLAYER_SIZE
  {
    commands.trigger(EndGame);
  }
}

fn bird_control(
  mut velocity: Single<&mut Velocity, With<Player>>,
  buttons: Res<ButtonInput<MouseButton>>,
) {
  if buttons.any_just_pressed([
    MouseButton::Left,
    MouseButton::Right,
  ]) {
    velocity.0 = 400.;
  }
}


#[derive(Event)]
struct EndGame;

// Observer
fn respawn_on_endgame(
  _: On<EndGame>, // don't care about event's value hence _
  mut commands: Commands,
  player: Single<Entity, With<Player>>,
) {
  commands.entity(*player).insert((
    Transform::from_xyz(-CANVAS_SIZE.x / 4., 0., 1.),
    Velocity(0.),
  ));

}
