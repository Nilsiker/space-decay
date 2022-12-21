use bevy::{prelude::*, core_pipeline::bloom::BloomSettings};
use bevy_atmosphere::prelude::AtmosphereCamera;
pub struct RailCameraPlugin;
impl Plugin for RailCameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(RailSpeed(120.0))
            .add_startup_system(setup)
            .add_system(rail_movement);
    }
}

fn setup(mut commands: Commands) {
    let transform = Transform::from_xyz(0.0, 70.0, 0.0);
    commands
        .spawn(Camera3dBundle {
            transform,
            camera:Camera{
                hdr: true,
                ..Default::default()
            },
            projection: Projection::Perspective(PerspectiveProjection {
                fov: f32::to_radians(45.0),
                ..default()
            }),
            ..default()
        })
        .insert(AtmosphereCamera::default())
        .insert(BloomSettings::default());
}
#[derive(Resource)]
struct RailSpeed(f32);

fn rail_movement(
    mut query: Query<&mut Transform, With<Camera>>,
    speed: Res<RailSpeed>,
    time: Res<Time>,
) {
    let Ok(mut transform) = query.get_single_mut() else {return;};
    let move_vector = Vec3::new(0.0, 0.0, -speed.0 * time.delta_seconds());
    transform.translation += move_vector;
}
