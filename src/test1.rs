use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Resource, Default)]
struct MyWorldCoords(Vec2);

#[derive(Component)]
struct MainCamera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(MyWorldCoords::default())
        .add_systems(Startup, setup)
        .add_systems(Update, my_cursor_system)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

fn my_cursor_system(
    mut mycoords: ResMut<MyWorldCoords>,
    windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = q_camera.single();
    let window = windows.single();

    if let Some(cursor_position) = window.cursor_position() {
        if let Some(world_position) = camera
            .viewport_to_world(camera_transform, cursor_position)
            .map(|ray| ray.origin.truncate())
        {
            mycoords.0 = world_position;
            println!("Cursor position in world coordinates: {:?}", world_position);
        }
    }
}
