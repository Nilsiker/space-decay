mod camera;
mod light;
mod player;

use bevy::{
    prelude::*, reflect::erased_serde::__private::serde::__private::de, window::CursorGrabMode,
};
use bevy_atmosphere::{
    prelude::{AtmosphereModel, AtmospherePlugin, Gradient},
    settings::AtmosphereSettings,
};
use bevy_inspector_egui::WorldInspectorPlugin;
use camera::RailCameraPlugin;
use light::LightPlugin;
use player::PlayerPlugin;
use rustpg::terragen::{
    mesh::{ColorConfig, ColorRange, MeshConfig, RenderMode, TextureMode},
    TerragenPlugin,
};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: "SpaceDecay".into(),
                        cursor_grab_mode: CursorGrabMode::Confined,
                        cursor_visible: false,
                        ..default()
                    },
                    ..Default::default()
                }),
        )
        .add_plugin(SpaceDecayPlugin)
        .run()
}

struct SpaceDecayPlugin;
impl Plugin for SpaceDecayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(TerragenPlugin {
            mesh_config: get_mesh_config(),
            ..default()
        })
        .insert_resource(AtmosphereSettings {
            resolution: 32,
            dithering: false,
        })
        .insert_resource(AtmosphereModel::new(Gradient {
            ground: Color::BLACK,
            horizon: Color::DARK_GREEN,
            sky: Color::CRIMSON,
        }))
        .add_plugin(RailCameraPlugin)
        .add_plugin(AtmospherePlugin)
        .add_plugin(LightPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(WorldInspectorPlugin::default());
    }
}

fn get_mesh_config() -> MeshConfig {
    MeshConfig {
        grid_size: 33,
        scale: 1024.0,
        height_multiplier: 32.0,
        render_mode: RenderMode::Mesh,
        texture_mode: TextureMode::Color,
        color_config: ColorConfig {
            colors: vec![
                ColorRange {
                    color: Color::rgba_u8(132, 51, 51, 255),
                    start_height: -1.0,
                },
                ColorRange {
                    color: Color::rgba_u8(70, 70, 164, 255),
                    start_height: 0.7,
                },
            ],
        },
        ..Default::default()
    }
}
