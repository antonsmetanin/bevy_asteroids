use bevy::{input::{keyboard::KeyCode, Input}, prelude::*, tasks::ComputeTaskPool};

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(keyboard_input_system.system())
        .add_system(apply_velocity.system())
        .run();
}

struct Player;
struct Velocity(Vec2);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let player_sprite = asset_server.load("player.png");
    let asteroid_sprite = asset_server.load("rock.png");
    let bullet_sprite = asset_server.load("shot.png");

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(player_sprite.into()),
            ..Default::default()
        })
        .insert(Velocity(Vec2::new(0.0, 0.0)))
        .insert(Player);

    commands.spawn_bundle(SpriteBundle {
        material: materials.add(asteroid_sprite.into()),
        ..Default::default()
    });

    commands.spawn_bundle(SpriteBundle {
        material: materials.add(bullet_sprite.into()),
        ..Default::default()
    });
}

fn keyboard_input_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Transform), With<Player>>
) {
    if let Ok((mut velocity, mut transform)) = query.single_mut() {
        let shift = if keyboard_input.pressed(KeyCode::W) {
            Vec2::new(0.0, 1.0)
        } else if keyboard_input.pressed(KeyCode::S) {
            Vec2::new(0.0, -1.0)
        } else {
            Vec2::new(0.0, 0.0)
        };

        let angle : f32 = if keyboard_input.pressed(KeyCode::A) {
            1.0
        } else if keyboard_input.pressed(KeyCode::D) {
            -1.0
        } else {
            0.0
        };

        transform.rotation = transform.rotation * (Quat::from_rotation_z(time.delta_seconds() * angle));
        velocity.0 += (transform.rotation * (time.delta_seconds() * shift.extend(0.0))).truncate();
    }
}

fn apply_velocity(
    task_pool: Res<ComputeTaskPool>,
    mut query: Query<(&mut Transform, &Velocity)>
) {
    query.par_for_each_mut(&task_pool, 32, |(mut transform, velocity)| {
        transform.translation += velocity.0.extend(0.0);
    })
}
