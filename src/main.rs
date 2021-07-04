use bevy::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let player_sprite = asset_server.load("player.png");
    let asteroid_sprite = asset_server.load("rock.png");
    let bullet_sprite = asset_server.load("shot.png");

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands.spawn_bundle(SpriteBundle {
        material: materials.add(player_sprite.into()),
        ..Default::default()
    });

    commands.spawn_bundle(SpriteBundle {
        material: materials.add(asteroid_sprite.into()),
        ..Default::default()
    });

    commands.spawn_bundle(SpriteBundle {
        material: materials.add(bullet_sprite.into()),
        ..Default::default()
    });
}