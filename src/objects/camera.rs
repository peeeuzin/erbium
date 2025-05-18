use std::f32::consts::PI;

use glium::winit::{
    dpi::LogicalPosition,
    event::{ElementState, KeyEvent, MouseButton, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};

pub struct Camera {
    aspect_ratio: f32,
    position: (f32, f32, f32),
    direction: (f32, f32, f32),

    sensitivity: f32,

    pitch: f32,
    yaw: f32,

    moving_up: bool,
    moving_left: bool,
    moving_down: bool,
    moving_right: bool,
    moving_forward: bool,
    moving_backward: bool,

    moving_direction: bool,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            aspect_ratio: 1024.0 / 768.0,
            position: (0.1, 0.1, 1.0),
            direction: (0.0, 0.0, -1.0),
            moving_up: false,
            moving_left: false,
            moving_down: false,
            moving_right: false,
            moving_forward: false,
            moving_backward: false,
            moving_direction: false,
            sensitivity: 1.0,
            pitch: 0.0,
            yaw: 0.0,
        }
    }

    pub fn get_perspective(&self) -> [[f32; 4]; 4] {
        let fov: f32 = PI / 2.0;
        let zfar = 1024.0;
        let znear = 0.1;

        let f = 1.0 / (fov / 2.0).tan();

        // note: remember that this is column-major, so the lines of code are actually columns
        [
            [f / self.aspect_ratio, 0.0, 0.0, 0.0],
            [0.0, f, 0.0, 0.0],
            [0.0, 0.0, (zfar + znear) / (zfar - znear), 1.0],
            [0.0, 0.0, -(2.0 * zfar * znear) / (zfar - znear), 0.0],
        ]
    }

    pub fn get_view(&self) -> [[f32; 4]; 4] {
        let f = {
            let f = self.direction;
            let len = f.0 * f.0 + f.1 * f.1 + f.2 * f.2;
            let len = len.sqrt();
            (f.0 / len, f.1 / len, f.2 / len)
        };

        let up = (0.0, 1.0, 0.0);

        let s = (
            f.1 * up.2 - f.2 * up.1,
            f.2 * up.0 - f.0 * up.2,
            f.0 * up.1 - f.1 * up.0,
        );

        let s_norm = {
            let len = s.0 * s.0 + s.1 * s.1 + s.2 * s.2;
            let len = len.sqrt();
            (s.0 / len, s.1 / len, s.2 / len)
        };

        let u = (
            s_norm.1 * f.2 - s_norm.2 * f.1,
            s_norm.2 * f.0 - s_norm.0 * f.2,
            s_norm.0 * f.1 - s_norm.1 * f.0,
        );

        let p = (
            -self.position.0 * s.0 - self.position.1 * s.1 - self.position.2 * s.2,
            -self.position.0 * u.0 - self.position.1 * u.1 - self.position.2 * u.2,
            -self.position.0 * f.0 - self.position.1 * f.1 - self.position.2 * f.2,
        );

        // note: remember that this is column-major, so the lines of code are actually columns
        [
            [s_norm.0, u.0, f.0, 0.0],
            [s_norm.1, u.1, f.1, 0.0],
            [s_norm.2, u.2, f.2, 0.0],
            [p.0, p.1, p.2, 1.0],
        ]
    }

    pub fn update(&mut self) {
        let f = {
            let f = self.direction;
            let len = f.0 * f.0 + f.1 * f.1 + f.2 * f.2;
            let len = len.sqrt();
            (f.0 / len, f.1 / len, f.2 / len)
        };

        let up = (0.0, 1.0, 0.0);

        let s = (
            f.1 * up.2 - f.2 * up.1,
            f.2 * up.0 - f.0 * up.2,
            f.0 * up.1 - f.1 * up.0,
        );

        let s = {
            let len = s.0 * s.0 + s.1 * s.1 + s.2 * s.2;
            let len = len.sqrt();
            (s.0 / len, s.1 / len, s.2 / len)
        };

        let u = (
            s.1 * f.2 - s.2 * f.1,
            s.2 * f.0 - s.0 * f.2,
            s.0 * f.1 - s.1 * f.0,
        );

        if self.moving_up {
            self.position.0 += u.0 * 0.01;
            self.position.1 += u.1 * 0.01;
            self.position.2 += u.2 * 0.01;
        }

        if self.moving_left {
            self.position.0 -= s.0 * 0.01;
            self.position.1 -= s.1 * 0.01;
            self.position.2 -= s.2 * 0.01;
        }

        if self.moving_down {
            self.position.0 -= u.0 * 0.01;
            self.position.1 -= u.1 * 0.01;
            self.position.2 -= u.2 * 0.01;
        }

        if self.moving_right {
            self.position.0 += s.0 * 0.01;
            self.position.1 += s.1 * 0.01;
            self.position.2 += s.2 * 0.01;
        }

        if self.moving_forward {
            self.position.0 += f.0 * 0.01;
            self.position.1 += f.1 * 0.01;
            self.position.2 += f.2 * 0.01;
        }

        if self.moving_backward {
            self.position.0 -= f.0 * 0.01;
            self.position.1 -= f.1 * 0.01;
            self.position.2 -= f.2 * 0.01;
        }
    }

    pub fn process_input(&mut self, event: &WindowEvent, window: &Window) {
        match event {
            WindowEvent::KeyboardInput { event, .. } => {
                self.handle_key(event);
            }
            WindowEvent::MouseInput { state, button, .. } => {
                self.handle_mouse(state, button);
            }

            WindowEvent::CursorMoved { position, .. } => {
                if self.moving_direction {
                    let size = window.inner_size();

                    let (w, h) = (size.width as f32, size.height as f32);

                    let center = (w / 2.0, h / 2.0);
                    let delta_x = (position.x as f32) - center.0;
                    let delta_y = (position.y as f32) - center.1;

                    self.yaw += self.sensitivity * delta_x / w;
                    self.pitch -= self.sensitivity * delta_y / h;
                    self.pitch = self.pitch.clamp(-PI / 2.0, PI / 2.0);

                    let x = self.pitch.cos() * self.yaw.cos();
                    let y = self.pitch.sin();
                    let z = self.pitch.cos() * self.yaw.sin();
                    let len = (x * x + y * y + z * z).sqrt();
                    self.direction = (x / len, y / len, z / len);

                    let center_pos = LogicalPosition::new(center.0 as f64, center.1 as f64);
                    window.set_cursor_position(center_pos).ok();
                }
            }

            _ => (),
        }
    }

    fn handle_key(&mut self, event: &KeyEvent) {
        let pressed = event.state == glium::winit::event::ElementState::Pressed;

        match &event.physical_key {
            PhysicalKey::Code(KeyCode::ControlLeft) => self.moving_up = pressed,
            PhysicalKey::Code(KeyCode::ShiftLeft) => self.moving_down = pressed,
            PhysicalKey::Code(KeyCode::KeyA) => self.moving_left = pressed,
            PhysicalKey::Code(KeyCode::KeyD) => self.moving_right = pressed,
            PhysicalKey::Code(KeyCode::KeyW) => self.moving_forward = pressed,
            PhysicalKey::Code(KeyCode::KeyS) => self.moving_backward = pressed,

            // vista frontal
            PhysicalKey::Code(KeyCode::F1) => {
                self.direction = (0.0, 0.0, -1.0);
                self.position = (0.0, 0.0, 1.0);
            }

            _ => (),
        };
    }

    fn handle_mouse(&mut self, state: &ElementState, button: &MouseButton) {
        if button == &MouseButton::Right {
            self.moving_direction = state == &ElementState::Pressed;
        }
    }
}
