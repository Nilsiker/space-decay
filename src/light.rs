use std::f32::consts::{FRAC_PI_4, PI, FRAC_PI_8, FRAC_PI_2};

use bevy::prelude::*;
pub struct LightPlugin;
impl Plugin for LightPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut commands: Commands) {
    let transform =
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, 0.0, FRAC_PI_4, -FRAC_PI_4));
    commands.spawn(DirectionalLightBundle {
        transform,
        ..default()
    });
}
