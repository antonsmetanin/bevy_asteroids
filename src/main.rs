use bevy::{input::{keyboard::KeyCode, Input}, prelude::*, tasks::ComputeTaskPool};

fn main() {
    App::build()
        .insert_resource(BulletMatHandle(None))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(keyboard_input_system.system())
        .add_system(apply_velocity.system())
        .add_system(apply_friction.system())
        .add_system(fire_input_system.system())
        .run();
}

struct Player;
struct Bullet;
struct BulletMatHandle(Option<Handle<ColorMaterial>>);
#[derive(Copy, Clone)]
struct Velocity(Vec2);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut bullet_handle_res: ResMut<BulletMatHandle>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let player_sprite = asset_server.load("player.png");
    let asteroid_sprite = asset_server.load("rock.png");
    let bullet_sprite = asset_server.load("shot.png");
    let bullet_mat = materials.add(bullet_sprite.into());
    bullet_handle_res.0 = Some(bullet_mat);

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
        velocity.0 += (transform.rotation * (time.delta_seconds() * shift.extend(0.0) * 80.0)).truncate();
    }
}


fn fire_input_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut commands: Commands,
    bullet_mat_res: Res<BulletMatHandle>,
    mut query: Query<(&Velocity, &Transform), With<Player>>
) {
    if let Ok((velocity, transform)) = query.single_mut() {
        if keyboard_input.just_pressed(KeyCode::Space) {
            commands.spawn_bundle(SpriteBundle {
                material: bullet_mat_res.0.clone().unwrap(),
                transform: Transform::from_translation(transform.translation),
                ..Default::default()
            })
            .insert(Velocity(velocity.0 + (transform.rotation * (Vec3::Y * 100.0)).truncate()))
            .insert(Bullet);
        }
    }
}

fn apply_velocity(
    time: Res<Time>,
    task_pool: Res<ComputeTaskPool>,
    windows: Res<Windows>,
    mut query: Query<(&mut Transform, &Velocity)>
) {
    query.par_for_each_mut(&task_pool, 32, |(mut transform, velocity)| {
        transform.translation += velocity.0.extend(0.0) * time.delta_seconds();
        let window = windows.get_primary().unwrap();

        wrap_position(&mut transform.translation.x, window.width());
        wrap_position(&mut transform.translation.y, window.height());

        fn wrap_position(position: &mut f32, size: f32) {
            if *position > size / 2.0 {
                *position -= size;
            } else if *position < -size / 2.0 {
                *position += size;
            }
        }
    })
}

fn apply_friction(
    time: Res<Time>,
    mut query: Query<&mut Velocity, With<Player>>
) {
    if let Ok(mut velocity) = query.single_mut() {
        if let Some(friction) = velocity.0.try_normalize() {
            velocity.0 -= friction * 30.0 * time.delta_seconds();
        };
    }
}