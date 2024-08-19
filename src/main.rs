use bevy::{
    input::mouse::MouseButtonInput,
    prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}
};
use bevy::window::{PrimaryWindow, Window, WindowPlugin, WindowMode, ExitCondition};
use bevy::input::mouse::MouseMotion;

use bevy::{
    math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume},
    prelude::*,
};
// use bevy_flycam::prelude::*;

const BOT_SPEED: f32 = 200.0;

#[derive(Component)]
struct Velocity(Vec3);

#[derive(Component)]
struct Bot;

#[derive(Component)]
struct Collider;

#[derive(Resource, Default)]
struct MyWorldCoords(Vec2);

#[derive(Component)]
struct MainCamera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                mode: WindowMode::Windowed,  // Set the window mode to fullscreen
                ..default()
            }),
            exit_condition: ExitCondition::OnPrimaryClosed,  // Handle window exit condition
            close_when_requested: true,  // Close the window when requested
            ..default()
        }))
        .insert_resource(MyWorldCoords::default())
        .add_systems(Startup, setup)
        .add_systems(Update, (move_land, cursor_move_land, ))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // commands.spawn(Camera2dBundle::default());
    commands.spawn((Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 1000.0),
        ..default()
    }, MainCamera));

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                scale: Vec3::new(0.3,0.25,0.3),
                rotation: Quat::from_xyzw(-0.0, -0.0, -0.0, 0.0),
                ..default()
            },
            texture: asset_server.load("bot.png"),
            
            sprite: Sprite {
                // color: Color::srgb(0.3, 0.3, 0.7),
                ..default()
            },
            ..default()
        },
        Bot,
        Collider,
    ));
}

fn move_land(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Bot>>,
    time: Res<Time>,
) {
    let mut bot_transform = query.single_mut();
    let mut direction = Vec3::new(0.0, 0.0, 0.0);

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }

    // Calculate the new horizontal paddle position based on player input
    let new_bot_position = bot_transform.translation + direction * BOT_SPEED * time.delta_seconds();

    // Update the paddle position,
    // making sure it doesn't cause the paddle to leave the arena
    // let left_bound = LEFT_WALL + WALL_THICKNESS / 2.0 + PADDLE_SIZE.x / 2.0 + PADDLE_PADDING;
    // let right_bound = RIGHT_WALL - WALL_THICKNESS / 2.0 - PADDLE_SIZE.x / 2.0 - PADDLE_PADDING;

    bot_transform.translation = new_bot_position //.clamp(left_bound, right_bound);
}

fn cursor_move_land(
    mut query: Query<&mut Transform, With<Bot>>, 
    mut my_coords: ResMut<MyWorldCoords>,
            windows: Query<&Window, With<PrimaryWindow>>,
            q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mouse_button1: Res<ButtonInput<MouseButton>>, mouse_button2: Res<ButtonInput<MouseButton>> ) {
    // let the_mouse_button_input = mouse_button_input(mouse_button);
    let (cursor_position, button_input) = cursor_input(mouse_button1, my_coords, windows, q_camera);
    // let new_bot_position = bot_transform.translation + direction * BOT_SPEED * time.delta_seconds();
    // println!(" lsakhdfKJHGJHJKHJ: {:?}, {:?}", cursor_position, button_input);
    // make it a vec3
    
    let mut bot_transform = query.single_mut();

    if mouse_button2.pressed(MouseButton::Left) || mouse_button2.pressed(MouseButton::Right) {
                // Print the cursor position
                println!("Cursor Pos: {:?}", cursor_position);
    
                // Update the bot's translation to the cursor position, extending to Vec3
                if let Some(bot_position) = cursor_position {
                    bot_transform.translation = bot_position.extend(0.0);
                }
}
    }

fn my_cursor_system(
    mut my_coords: ResMut<MyWorldCoords>,
    windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) -> Option<Vec2> {
    let (camera, camera_transform) = q_camera.single();
    let window = windows.single();

    if let Some(cursor_position) = window.cursor_position() {
        if let Some(world_position) = camera
            .viewport_to_world(camera_transform, cursor_position)
            .map(|ray| ray.origin.truncate())
        {
            my_coords.0 = world_position;
            return Some(world_position)
        }
        else {None}
    }
    else {None}
}

fn cursor_input(
    mouse_button: Res<ButtonInput<MouseButton>>,
    my_coords: ResMut<MyWorldCoords>,
        windows: Query<&Window, With<PrimaryWindow>>,
        q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) -> (Option<Vec2>, Option<MouseButton>) {
    let the_mouse_button_input = mouse_button_input(mouse_button);

    // Get the last cursor moved event's position
    let cursor_position = my_cursor_system(my_coords, windows, q_camera);

    return (cursor_position, the_mouse_button_input);
}


fn mouse_button_input(
    buttons: Res<ButtonInput<MouseButton>>,
) -> Option<MouseButton> {
    if buttons.just_released(MouseButton::Left) {
        // Left Button was released
        Some(MouseButton::Left)
    }
    else if buttons.just_released(MouseButton::Right) {
        // Right Button is being held down
        return Some(MouseButton::Left);
    }
    // we can check multiple at once with `.any_*`
    else if buttons.just_released(MouseButton::Middle) {
        return Some(MouseButton::Middle);
    }
    else {
        None
    }
}
