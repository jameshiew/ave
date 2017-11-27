use cgmath::{Rad, Angle, PerspectiveFov};
use cgmath::Matrix4;
use collision::Frustum;
use std;
use collision;
use glutin;
use space::{Position, Direction};
use block::cube_at;
use default;

const DEFAULT_ASPECT_RATIO: f32 = default::VIEWPORT_WIDTH as f32 / default::VIEWPORT_HEIGHT as f32;
const DEFAULT_FIELD_OF_VIEW: Rad<f32> = Rad(std::f32::consts::PI / 2.0 * (7.0 / 9.0));
const DEFAULT_Z_NEAR_CUTOFF: f32 = 0.1;
const DEFAULT_Z_FAR_CUTOFF: f32 = 1024.0;

pub struct CameraState {
    perspective_fov: PerspectiveFov<f32>,
    pub perspective: Matrix4<f32>,
    pub position: Position,
    pub direction: Direction,

    move_speed: f32,
    rotation_speed: f32,

    moving_up: bool,
    moving_left: bool,
    moving_down: bool,
    moving_right: bool,
    moving_forward: bool,
    moving_backward: bool,
    rotating_up: bool,
    rotating_left: bool,
    rotating_down: bool,
    rotating_right: bool,
}

impl CameraState {
    pub fn new() -> CameraState {
        let mut camera = CameraState {
            perspective_fov: PerspectiveFov {
                fovy: DEFAULT_FIELD_OF_VIEW,
                aspect: DEFAULT_ASPECT_RATIO,
                near: DEFAULT_Z_NEAR_CUTOFF,
                far: DEFAULT_Z_FAR_CUTOFF,
            },
            perspective: [[0.0; 4]; 4].into(),
            position: [0.0, 16.0, 0.0].into(),
            direction: [0.0, -1.0, -1.0].into(),
            move_speed: 0.3,
            rotation_speed: 0.08,
            moving_up: false,
            moving_left: false,
            moving_down: false,
            moving_right: false,
            moving_forward: false,
            moving_backward: false,
            rotating_up: false,
            rotating_left: false,
            rotating_down: false,
            rotating_right: false,
        };
        camera.update_perspective();
        camera
    }

    fn update_perspective(&mut self) {
        let f = 1.0 / (self.perspective_fov.fovy / 2.0).tan();
        self.perspective = Matrix4::new(
            f / self.perspective_fov.aspect, 0.0, 0.0, 0.0,
            0.0, f, 0.0, 0.0,
            0.0, 0.0, (self.perspective_fov.far + self.perspective_fov.near) / (self.perspective_fov.far - self.perspective_fov.near), 1.0,
            0.0, 0.0, -(2.0 * self.perspective_fov.far * self.perspective_fov.near) / (self.perspective_fov.far - self.perspective_fov.near), 0.0,
        );
    }

    pub fn get_view(&self) -> Matrix4<f32> {
        let f = {
            let f = &self.direction;
            let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
            let len = len.sqrt();
            (f[0] / len, f[1] / len, f[2] / len)
        };

        let up = (0.0, 1.0, 0.0);

        let s = (f.1 * up.2 - f.2 * up.1,
                 f.2 * up.0 - f.0 * up.2,
                 f.0 * up.1 - f.1 * up.0);

        let s_norm = {
            let len = s.0 * s.0 + s.1 * s.1 + s.2 * s.2;
            let len = len.sqrt();
            (s.0 / len, s.1 / len, s.2 / len)
        };

        let u = (s_norm.1 * f.2 - s_norm.2 * f.1,
                 s_norm.2 * f.0 - s_norm.0 * f.2,
                 s_norm.0 * f.1 - s_norm.1 * f.0);

        let p = (-self.position[0] * s.0 - self.position[1] * s.1 - self.position[2] * s.2,
                 -self.position[0] * u.0 - self.position[1] * u.1 - self.position[2] * u.2,
                 -self.position[0] * f.0 - self.position[1] * f.1 - self.position[2] * f.2);

        Matrix4::new(
            s_norm.0, u.0, f.0, 0.0,
            s_norm.1, u.1, f.1, 0.0,
            s_norm.2, u.2, f.2, 0.0,
            p.0, p.1, p.2, 1.0,
        )
    }

    pub fn update(&mut self) {
        let f = {
            let f = &self.direction;
            let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
            let len = len.sqrt();
            (f[0] / len, f[1] / len, f[2] / len)
        };

        let up = (0.0, 1.0, 0.0);

        let s = (f.1 * up.2 - f.2 * up.1,
                 f.2 * up.0 - f.0 * up.2,
                 f.0 * up.1 - f.1 * up.0);

        let s = {
            let len = s.0 * s.0 + s.1 * s.1 + s.2 * s.2;
            let len = len.sqrt();
            (s.0 / len, s.1 / len, s.2 / len)
        };

        let u = (s.1 * f.2 - s.2 * f.1,
                 s.2 * f.0 - s.0 * f.2,
                 s.0 * f.1 - s.1 * f.0);

        if self.moving_up {
            self.position[0] += u.0 * self.move_speed;
            self.position[1] += u.1 * self.move_speed;
            self.position[2] += u.2 * self.move_speed;
        }

        if self.moving_left {
            self.position[0] -= s.0 * self.move_speed;
            self.position[1] -= s.1 * self.move_speed;
            self.position[2] -= s.2 * self.move_speed;
        }

        if self.moving_down {
            self.position[0] -= u.0 * self.move_speed;
            self.position[1] -= u.1 * self.move_speed;
            self.position[2] -= u.2 * self.move_speed;
        }

        if self.moving_right {
            self.position[0] += s.0 * self.move_speed;
            self.position[1] += s.1 * self.move_speed;
            self.position[2] += s.2 * self.move_speed;
        }

        if self.rotating_up {
            self.direction[0] += u.0 * self.rotation_speed;
            self.direction[1] += u.1 * self.rotation_speed;
            self.direction[2] += u.2 * self.rotation_speed;
        }

        if self.rotating_left {
            self.direction[0] -= s.0 * self.rotation_speed;
            self.direction[1] -= s.1 * self.rotation_speed;
            self.direction[2] -= s.2 * self.rotation_speed;
        }

        if self.rotating_down {
            self.direction[0] -= u.0 * self.rotation_speed;
            self.direction[1] -= u.1 * self.rotation_speed;
            self.direction[2] -= u.2 * self.rotation_speed;
        }

        if self.rotating_right {
            self.direction[0] += s.0 * self.rotation_speed;
            self.direction[1] += s.1 * self.rotation_speed;
            self.direction[2] += s.2 * self.rotation_speed;
        }

        if self.moving_forward {
            self.position[0] += f.0 * self.move_speed;
            self.position[1] += f.1 * self.move_speed;
            self.position[2] += f.2 * self.move_speed;
        }

        if self.moving_backward {
            self.position[0] -= f.0 * self.move_speed;
            self.position[1] -= f.1 * self.move_speed;
            self.position[2] -= f.2 * self.move_speed;
        }
    }

    pub fn can_see(&self, position: Position) -> bool {
        let frustum = Frustum::from_matrix4(self.perspective * self.get_view()).unwrap();
        // this is a naive approach for frustum culling cubes
        // we should check if all cube vertices lie on the wrong side of one plane, rather than
        // if all vertices are outside the frustum - see http://www.lighthouse3d.com/tutorials/view-frustum-culling/geometric-approach-testing-boxes/
        for vertex in cube_at(&position).iter() {
            match frustum.contains(vertex) {
                collision::Relation::Out => continue,
                _ => return true,
            }
        }
        false
    }

    pub fn process_input(&mut self, pressed: bool, key: glutin::VirtualKeyCode) {
        match key {
            glutin::VirtualKeyCode::Space => self.moving_up = pressed,
            glutin::VirtualKeyCode::LControl => self.moving_down = pressed,
            glutin::VirtualKeyCode::A => self.moving_left = pressed,
            glutin::VirtualKeyCode::D => self.moving_right = pressed,
            glutin::VirtualKeyCode::W => self.moving_forward = pressed,
            glutin::VirtualKeyCode::S => self.moving_backward = pressed,
            glutin::VirtualKeyCode::Left => self.rotating_left = pressed,
            glutin::VirtualKeyCode::Right => self.rotating_right = pressed,
            glutin::VirtualKeyCode::Up => self.rotating_up = pressed,
            glutin::VirtualKeyCode::Down => self.rotating_down = pressed,
            glutin::VirtualKeyCode::Q => {
                self.move_speed += self.move_speed + 0.1;
                if self.move_speed > 1.0 {
                    self.move_speed = 1.0;
                }
            },
            glutin::VirtualKeyCode::E => {
                self.move_speed = self.move_speed - 0.1;
                if self.move_speed < 0.1 {
                    self.move_speed = 0.1;
                }
            },
            _ => (),
        };
    }
}
