use bevy::prelude::*;
use rustpg::terragen::PlayerPositionChangedEvent;

#[derive(Component)]
struct Player;
#[derive(Resource)]
struct PlayerSpeed(f32);
#[derive(Component)]
struct Ammo {
    current: u8,
    max: u8,
    timer: Timer,
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerSpeed(10.0))
            .add_system(spawn_player)
            .add_system(move_player)
            .add_system(shoot)
            .add_system(regen_ammo)
            .add_system(move_projectiles)
            .add_system(despawn_projectiles)
            .add_system(send_player_position_events);
    }
}

fn regen_ammo(mut query: Query<&Ammo>) {}

fn spawn_player(
    mut query: Query<Entity, Added<Camera>>,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let Ok(camera) = query.get_single_mut() else {return;};

    let mesh = meshes.add(shape::Cube { size: 1.0 }.into());
    let material = materials.add(Color::DARK_GRAY.into());
    commands.entity(camera).with_children(|children| {
        children.spawn((
            MaterialMeshBundle {
                mesh,
                material: material,
                transform: Transform::from_xyz(0.0, 0.0, -20.0),
                ..default()
            },
            Player,
            Name::new("Player"),
            Ammo {
                current: 8,
                max: 8,
                timer: Timer::from_seconds(2.0, TimerMode::Once),
            },
        ));
    });
}

fn move_player(
    mut query: Query<&mut Transform, With<Player>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    player_speed: Res<PlayerSpeed>,
) {
    let Ok(mut player) = query.get_single_mut() else {return;};
    let mut move_vector = Vec3::ZERO;

    if input.pressed(KeyCode::W) {
        move_vector += Vec3::Y
    }
    if input.pressed(KeyCode::A) {
        move_vector -= Vec3::X
    }
    if input.pressed(KeyCode::S) {
        move_vector -= Vec3::Y
    }
    if input.pressed(KeyCode::D) {
        move_vector += Vec3::X
    }

    move_vector = move_vector.normalize_or_zero() * player_speed.0 * time.delta_seconds();

    player.translation += move_vector;
    player.translation.x = f32::clamp(player.translation.x, -13.0, 13.0);
    player.translation.y = f32::clamp(player.translation.y, -7.0, 7.0);
    player.rotation = Quat::from_euler(
        EulerRot::XYZ,
        player.translation.y / 150.0,
        -player.translation.x / 55.0,
        0.0,
    );
}

#[derive(Component)]
struct Projectile {
    direction: Vec3,
    speed: f32,
    timer: Timer,
}

fn shoot(
    mut commands: Commands,
    mut query: Query<(&GlobalTransform, &mut Ammo), With<Player>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    input: Res<Input<KeyCode>>,
) {
    let Ok((transform, mut ammo)) = query.get_single_mut() else {return;};

    // todo check for ammo

    if input.just_pressed(KeyCode::Space) {
        let mut material: StandardMaterial = Color::GOLD.into();
        material.emissive = Color::GOLD.into();
        commands.spawn((
            MaterialMeshBundle {
                mesh: meshes.add(
                    shape::Icosphere {
                        radius: 0.5,
                        subdivisions: 2,
                    }
                    .into(),
                ),
                material: materials.add(material),
                transform: Transform::from_translation(transform.translation() + Vec3::NEG_Z * 4.0),
                ..Default::default()
            },
            Projectile {
                direction: transform.forward(),
                speed: 500.0,
                timer: Timer::from_seconds(2.0, TimerMode::Once),
            },
        ));
    }
}

fn move_projectiles(mut query: Query<(&mut Transform, &Projectile)>, time: Res<Time>) {
    for (mut transform, projectile) in &mut query {
        transform.translation += projectile.direction * projectile.speed * time.delta_seconds();
    }
}

fn despawn_projectiles(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Projectile)>,
    time: Res<Time>,
) {
    for (projectile, mut stats) in &mut query {
        if stats.timer.finished() {
            commands.entity(projectile).despawn_recursive();
        } else {
            stats.timer.tick(time.delta());
        }
    }
}

fn send_player_position_events(
    query: Query<&GlobalTransform, With<Player>>,
    mut events: EventWriter<PlayerPositionChangedEvent>,
) {
    let Ok(transform) = query.get_single() else {return;};
    events.send(PlayerPositionChangedEvent(transform.translation()));
}
