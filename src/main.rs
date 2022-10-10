use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    render::camera::{CameraUpdateSystem, Viewport},
    window::{PresentMode, WindowId, WindowResized},
    winit::WinitSettings,
};

fn main() {
    App::new()
        .insert_resource(WinitSettings::desktop_app())
        .insert_resource(WindowDescriptor {
            title: "Bevy Viewport Bug".to_string(),
            present_mode: PresentMode::AutoNoVsync,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(world_setup)
        .add_system(set_camera_viewports)
        .add_system_to_stage(
            CoreStage::PostUpdate,
            update_camera_projection_when_camera_changes.before(CameraUpdateSystem),
        )
        .run();
}

fn world_setup(mut commands: Commands) {
    commands
        .spawn_bundle(Camera2dBundle {
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(Color::BLUE),
                ..default()
            },
            ..default()
        })
        .insert(GraphCamera);

    commands
        .spawn_bundle(Camera2dBundle {
            camera: Camera {
                priority: 1,
                ..default()
            },
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(Color::CRIMSON),
                ..default()
            },
            ..default()
        })
        .insert(ViewCamera);
}

#[derive(Component)]
struct GraphCamera;

#[derive(Component)]
struct ViewCamera;

fn set_camera_viewports(
    windows: Res<Windows>,
    mut resize_events: EventReader<WindowResized>,
    mut graph_camera: Query<&mut Camera, (With<GraphCamera>, Without<ViewCamera>)>,
    mut view_camera: Query<&mut Camera, With<ViewCamera>>,
) {
    for resize_event in resize_events.iter() {
        if resize_event.id == WindowId::primary() {
            let window = windows.primary();

            let mut view_camera = view_camera.single_mut();
            view_camera.viewport = Some(Viewport {
                physical_position: UVec2::new(0, 0),
                physical_size: UVec2::new(window.physical_width(), window.physical_height() / 2),
                ..default()
            });

            let mut graph_camera = graph_camera.single_mut();
            graph_camera.viewport = Some(Viewport {
                physical_position: UVec2::new(0, window.physical_height() / 2),
                physical_size: UVec2::new(window.physical_width(), window.physical_height() / 2),
                ..default()
            });

            info!(
                "window size: {:?}, {:?}",
                window.physical_width(),
                window.physical_height()
            );
            info!("view_camera: {:?}", view_camera);
            info!("graph_camera: {:?}", view_camera);
        }
    }
}

/// Workaround for bevy not updating camera projection when viewport changes:
/// <https://github.com/bevyengine/bevy/issues/5944>
fn update_camera_projection_when_camera_changes(
    mut changed_cameras: Query<&mut OrthographicProjection, Changed<Camera>>,
) {
    // For every changed camera
    for mut projection in &mut changed_cameras {
        // Force a change to the projection in order to trigger a re-building of the camera
        // projection for the new viewport.
        projection.set_changed();
    }
}
