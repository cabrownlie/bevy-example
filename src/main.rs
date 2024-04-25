use bevy::prelude::*;

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_systems(Startup, startup)
		.add_systems(Update, update)
		.run();
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct Player;

fn startup(mut commands: Commands) {
	// Player
	commands.spawn((
		Player,
		SpriteBundle {
			sprite: Sprite {
				custom_size: Some(Vec2::splat(50.0)),
				color: Color::TOMATO,
				..default()
			},
			..default()
		},
	));

	// MainCamera
	commands.spawn((MainCamera, Camera2dBundle::default()));
}

fn update(mut query: Query<&mut Transform, With<Player>>, time: Res<Time>) {
	for mut transform in &mut query {
		transform.rotate_z(time.delta_seconds() * 10.0);
	}
}
