use hecs::{PreparedQuery, World};
use std::any::Any;
use std::f32::consts::FRAC_PI_2;
use ultraviolet::{Isometry3, Mat4, Rotor3, Vec3};

use winit::{dpi::PhysicalPosition, event::*, window::Window};
use zen_app::{EventQueue, Resource, TimeDelta};
use zen_input::{KeyboardInput, MouseInput, MouseMotion, MouseWheel};
use zen_render::{
    camera::{Camera, Projection},
    Renderer,
};

// TODO: impl Bundle if generics are fixed in hecs
pub struct FirstPersonCameraBundle {
    pub camera: Camera,
    pub controller: FirstPersonController,
    pub projection: Projection,
    pub time: Resource<TimeDelta>,
    pub keyboard_input: EventQueue<KeyboardInput>,
    pub mouse_motion: EventQueue<MouseMotion>,
    pub mouse_input: EventQueue<MouseInput>,
}

impl FirstPersonCameraBundle {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            camera: Camera::new(Vec3::new(0.0, 10.0, 0.0), Rotor3::from_rotation_yz(45.0)),
            controller: FirstPersonController::new(4.0, 0.4),
            projection: Projection::new(width, height, 45.0, 0.1, 1000.0),
            time: Resource::new(std::time::Duration::ZERO),
            keyboard_input: EventQueue::new(),
            mouse_motion: EventQueue::new(),
            mouse_input: EventQueue::new(),
        }
    }
}

#[derive(Debug)]
pub struct FirstPersonController {
    pub amount_left: f32,
    pub amount_right: f32,
    pub amount_forward: f32,
    pub amount_backward: f32,
    pub amount_up: f32,
    pub amount_down: f32,
    pub rotate_horizontal: f32,
    pub rotate_vertical: f32,
    pub scroll: f32,
    pub speed: f32,
    pub sensitivity: f32,
    pub cursor_grab: bool,
}

impl FirstPersonController {
    pub fn new(speed: f32, sensitivity: f32) -> Self {
        Self {
            amount_left: 0.0,
            amount_right: 0.0,
            amount_forward: 0.0,
            amount_backward: 0.0,
            amount_up: 0.0,
            amount_down: 0.0,
            rotate_horizontal: 0.0,
            rotate_vertical: 0.0,
            scroll: 0.0,
            speed,
            sensitivity,
            cursor_grab: false,
        }
    }
}

pub fn on_button(world: &mut World, window: &Window) {
    for (_id, (controller, mouse_input)) in
        world.query_mut::<(&mut FirstPersonController, &mut EventQueue<MouseInput>)>()
    {
        while let Some(event) = mouse_input.pop() {
            if event.button == MouseButton::Left && event.state == ElementState::Pressed {
                controller.cursor_grab = !controller.cursor_grab;
                let result = window.set_cursor_grab(controller.cursor_grab);
                if let Err(err) = result {
                    println!("{:?}", err);
                }
                window.set_cursor_visible(!controller.cursor_grab);
            }
        }
    }
}

pub fn on_scroll(world: &mut World) {
    for (_id, (controller, mouse_wheel)) in
        world.query_mut::<(&mut FirstPersonController, &mut EventQueue<MouseWheel>)>()
    {
        while let Some(event) = mouse_wheel.pop() {
            controller.scroll = -match event.delta {
                // I'm assuming a line is about 100 pixels
                MouseScrollDelta::LineDelta(_, scroll) => scroll * 100.0,
                MouseScrollDelta::PixelDelta(PhysicalPosition { y: scroll, .. }) => scroll as f32,
            };
        }
    }
}

pub fn on_motion(world: &mut World) {
    for (_id, (controller, mouse_motion)) in
        world.query_mut::<(&mut FirstPersonController, &mut EventQueue<MouseMotion>)>()
    {
        while let Some(MouseMotion { delta: (dx, dy) }) = mouse_motion.pop() {
            controller.rotate_horizontal = dx as f32;
            controller.rotate_vertical = dy as f32;
        }
    }
}

pub fn on_key(world: &mut World) {
    for (_id, (controller, keyboard_input)) in
        world.query_mut::<(&mut FirstPersonController, &mut EventQueue<KeyboardInput>)>()
    {
        while let Some(KeyboardInput { state, code }) = keyboard_input.pop() {
            let amount = if state == ElementState::Pressed {
                1.0
            } else {
                0.0
            };
            match code {
                VirtualKeyCode::W | VirtualKeyCode::Up => controller.amount_forward = amount,
                VirtualKeyCode::S | VirtualKeyCode::Down => controller.amount_backward = amount,
                VirtualKeyCode::A | VirtualKeyCode::Left => controller.amount_left = amount,
                VirtualKeyCode::D | VirtualKeyCode::Right => controller.amount_right = amount,
                VirtualKeyCode::Space => controller.amount_up = amount,
                VirtualKeyCode::LShift => controller.amount_down = amount,
                _ => {}
            }
        }
    }
}

pub fn update_camera(world: &mut World) {
    for (_id, (camera, controller, delta)) in world.query_mut::<(
        &mut Camera,
        &mut FirstPersonController,
        &mut Resource<TimeDelta>,
    )>() {
        // println!("{:?}", camera);
        let delta = delta.inner().as_secs_f32();

        // Move forward/backward and left/right
        let up = (controller.amount_up - controller.amount_down) * controller.speed * delta;
        let right = (controller.amount_right - controller.amount_left) * controller.speed * delta;
        let forward =
            (controller.amount_forward - controller.amount_backward) * controller.speed * delta;
        camera.prepend_translation(Vec3::new(right, up, -forward));
        // camera.prepend_translation(Vec3::new(right, 0.0, 0.0));
        // camera.prepend_translation(Vec3::new(0.0, up, 0.0));
        // camera.prepend_translation(Vec3::new(0.0, 0.0, forward));
        // let (yaw_sin, yaw_cos) = camera.yaw.sin_cos();
        // let forward = Vec3::new(yaw_cos, 0.0, yaw_sin).normalize() * (controller.amount_forward - controller.amount_backward) * controller.speed * delta;
        // let right = Vec3::new(-yaw_sin, 0.0, yaw_cos).normalize() * (controller.amount_right - controller.amount_left) * controller.speed * delta;
        // camera.append_translation(forward);
        // camera.append_translation(right)
        // let (yaw_sin, yaw_cos) = camera.yaw.sin_cos();
        // let forward = Vec3::new(yaw_cos, 0.0, yaw_sin).normalize();
        // let right = Vec3::new(-yaw_sin, 0.0, yaw_cos).normalize();
        // camera.position += forward
        //     * (controller.amount_forward - controller.amount_backward)
        //     * controller.speed
        //     * deltay;
        // camera.position +=
        //     right * (controller.amount_right - controller.amount_left) * controller.speed * delta;

        // let (pitch_sin, pitch_cos) = camera.pitch.sin_cos();
        // let scrollward = Vec3::new(pitch_cos * yaw_cos, pitch_sin, pitch_cos * yaw_sin ).normalize() * controller.scroll * controller.speed * controller.sensitivity * delta;
        // controller.scroll = 0.0;
        // camera.append_translation(scrollward);
        // Move in/out (aka. "zoom")
        // Note: this isn't an actual zoom. The camera's position
        // changes when zooming. I've added this to make it easier
        // to get closer to an object you want to focus on.
        // let (pitch_sin, pitch_cos) = camera.pitch.sin_cos();
        // let scrollward = Vec3::new(pitch_cos * yaw_cos, pitch_sin, pitch_cos * yaw_sin).normalize();
        // camera.position +=
        //     scrollward * controller.scroll * controller.speed * controller.sensitivity * delta;
        // controller.scroll = 0.0;

        // Move up/down. Since we don't use roll, we can just
        // modify the y coordinate directly.
        // camera.position.y +=
        //     (controller.amount_up - controller.amount_down) * controller.speed * delta;

        if controller.cursor_grab {
            // Rotate
            let yaw = controller.rotate_horizontal * controller.sensitivity * delta;
            let pitch = {
                let pitch = -controller.rotate_vertical * controller.sensitivity * delta;
                if pitch < -FRAC_PI_2 {
                    -FRAC_PI_2
                } else if pitch > FRAC_PI_2 {
                    FRAC_PI_2
                } else {
                    pitch
                }
            };

            // let movement = Isometry3::new(
            //     Vec3::new(right, up, forward),
            //     Rotor3::from_euler_angles(0.0, pitch, yaw),
            // );
            // camera.prepend_isometry(movement);

            camera.append_rotation(Rotor3::from_euler_angles(0.0, pitch, yaw));

            // If process_mouse isn't called every frame, these values
            // will not get set to zero, and the camera will rotate
            // when moving in a non cardinal direction.
            controller.rotate_horizontal = 0.0;
            controller.rotate_vertical = 0.0;
        }
        // Keep the camera's angle from going too high/low.
        // if camera.pitch < -FRAC_PI_2 {
        //     camera.pitch = -FRAC_PI_2;
        // } else if camera.pitch > FRAC_PI_2 {
        //     camera.pitch = FRAC_PI_2;
        // }
    }
}
