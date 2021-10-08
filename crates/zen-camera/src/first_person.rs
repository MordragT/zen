use hecs::{PreparedQuery, World};
use std::f32::consts::FRAC_PI_2;
use ultraviolet::{Isometry3, Mat4, Rotor3, Vec3};

use crate::Camera;
use winit::{dpi::PhysicalPosition, event::*};
use zen_core::{EventQueue, TimeDelta};
use zen_input::{KeyboardInput, MouseMotion, MouseWheel};

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
        }
    }
}

// fn on_cursor_entered(&mut self, world: &mut World, query: PreparedQuery<>) {}

// fn on_cursor_left(&mut self, _world: &mut World) {}

// fn on_button(&mut self, _button: MouseButton, _state: ElementState, _world: &mut World) {}

fn on_scroll(
    world: &mut World,
    query: &mut PreparedQuery<(&mut FirstPersonController, &mut EventQueue<MouseWheel>)>,
) {
    for (_id, (controller, mouse_wheel)) in query.query_mut(world) {
        while let Some(event) = mouse_wheel.pop() {
            controller.scroll = -match event.delta {
                // I'm assuming a line is about 100 pixels
                MouseScrollDelta::LineDelta(_, scroll) => scroll * 100.0,
                MouseScrollDelta::PixelDelta(PhysicalPosition { y: scroll, .. }) => scroll as f32,
            };
        }
    }
}

fn on_motion(
    world: &mut World,
    query: &mut PreparedQuery<(&mut FirstPersonController, &mut EventQueue<MouseMotion>)>,
) {
    for (_id, (controller, mouse_motion)) in query.query_mut(world) {
        while let Some(MouseMotion { delta: (dx, dy) }) = mouse_motion.pop() {
            controller.rotate_horizontal = dx as f32;
            controller.rotate_vertical = dy as f32;
        }
    }
}
fn on_key(
    world: &mut World,
    query: &mut PreparedQuery<(&mut FirstPersonController, &mut EventQueue<KeyboardInput>)>,
) {
    for (_id, (controller, keyboard_input)) in query.query_mut(world) {
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

pub fn update_camera(
    world: &mut World,
    query: &mut PreparedQuery<(&mut Camera, &mut FirstPersonController, &mut TimeDelta)>,
) {
    for (_id, (camera, controller, delta)) in query.query_mut(world) {
        let delta = delta.as_secs_f32();

        // Move forward/backward and left/right
        let up = (controller.amount_up - controller.amount_down) * controller.speed * delta;
        let right = (controller.amount_right - controller.amount_left) * controller.speed * delta;
        let forward =
            (controller.amount_forward - controller.amount_backward) * controller.speed * delta;
        camera.prepend_translation(Vec3::new(right, up, forward));
        // let (yaw_sin, yaw_cos) = camera.yaw.sin_cos();
        // let forward = Vec3::new(yaw_cos, 0.0, yaw_sin).normalize();
        // let right = Vec3::new(-yaw_sin, 0.0, yaw_cos).normalize();
        // camera.position += forward
        //     * (controller.amount_forward - controller.amount_backward)
        //     * controller.speed
        //     * deltay;
        // camera.position +=
        //     right * (controller.amount_right - controller.amount_left) * controller.speed * delta;

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

        camera.append_rotation(Rotor3::from_euler_angles(0.0, pitch, yaw));

        // If process_mouse isn't called every frame, these values
        // will not get set to zero, and the camera will rotate
        // when moving in a non cardinal direction.
        controller.rotate_horizontal = 0.0;
        controller.rotate_vertical = 0.0;

        // Keep the camera's angle from going too high/low.
        // if camera.pitch < -FRAC_PI_2 {
        //     camera.pitch = -FRAC_PI_2;
        // } else if camera.pitch > FRAC_PI_2 {
        //     camera.pitch = FRAC_PI_2;
        // }
    }
}
