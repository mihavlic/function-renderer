use std::f32::consts::FRAC_PI_2;

use glam::f32::*;

/// The Shoemake Arcball camera.
pub struct ArcballCamera {
    translation: Mat4,
    center_translation: Mat4,
    rotation: Quat,
    camera: Mat4,
    inv_camera: Mat4,
    zoom_speed: f32,
    inv_screen: Vec2,
}

impl ArcballCamera {
    /// Create a new Arcball camera focused at the `center` point, which will zoom at `zoom_speed`
    /// `screen` should be `[screen_width, screen_height]`.
    pub fn new(
        center: Vec3,
        zoom_speed: f32,
        initial_distance: f32,
        screen: Vec2,
    ) -> ArcballCamera {
        let mut cam = ArcballCamera {
            translation: Mat4::from_translation(Vec3::new(0.0, 0.0, -initial_distance)),
            center_translation: Mat4::from_translation(center).inverse(),
            rotation: Quat::from_euler(glam::EulerRot::XYZ, 0.0, FRAC_PI_2, 0.0),
            camera: Mat4::IDENTITY,
            inv_camera: Mat4::IDENTITY,
            zoom_speed,
            inv_screen: Vec2::new(1.0 / screen.x, 1.0 / screen.y),
        };
        cam.update_camera();
        cam
    }
    /// Get the view matrix computed by the camera.
    pub fn get_mat4(&self) -> Mat4 {
        self.camera
    }
    /// Get the inverse view matrix
    pub fn get_inv_camera(&self) -> Mat4 {
        self.inv_camera
    }
    /// Get the camera eye position
    pub fn eye_pos(&self) -> Vec3 {
        Vec3::new(
            self.inv_camera.w_axis.x,
            self.inv_camera.w_axis.y,
            self.inv_camera.w_axis.z,
        )
    }
    /// Get the camera view direction
    pub fn eye_dir(&self) -> Vec3 {
        let dir = self.inv_camera * Vec4::NEG_Z;
        Vec3::new(dir.x, dir.y, dir.z).normalize()
    }
    /// Get the camera view direction
    pub fn up_dir(&self) -> Vec3 {
        let dir = self.inv_camera * Vec4::Y;
        Vec3::new(dir.x, dir.y, dir.z).normalize()
    }
    /// Rotate the camera, mouse positions should be in pixel coordinates.
    ///
    /// Rotates from the orientation at the previous mouse position specified by `mouse_prev`
    /// to the orientation at the current mouse position, `mouse_cur`.
    pub fn rotate(&mut self, mouse_prev: Vec2, mouse_cur: Vec2) {
        let m_cur = Vec2::new(
            mouse_cur.x * 2.0 * self.inv_screen.x - 1.0,
            1.0 - 2.0 * mouse_cur.y * self.inv_screen.y,
        )
        .clamp(Vec2::NEG_ONE, Vec2::ONE);

        let m_prev = Vec2::new(
            mouse_prev.x * 2.0 * self.inv_screen.x - 1.0,
            1.0 - 2.0 * mouse_prev.y * self.inv_screen.y,
        )
        .clamp(Vec2::NEG_ONE, Vec2::ONE);

        let mouse_cur_ball = ArcballCamera::screen_to_arcball(m_cur);
        let mouse_prev_ball = ArcballCamera::screen_to_arcball(m_prev);
        self.rotation = mouse_cur_ball * mouse_prev_ball * self.rotation;

        self.update_camera();
    }
    /// Zoom the camera in by some amount. Positive values zoom in, negative zoom out.
    pub fn zoom(&mut self, amount: f32, elapsed: f32) {
        let motion = Vec3::new(0.0, 0.0, amount);
        self.translation =
            Mat4::from_translation(motion * self.zoom_speed * elapsed) * self.translation;
        self.update_camera();
    }
    /// Pan the camera following the motion of the mouse. The mouse delta should be in pixels.
    pub fn pan(&mut self, mouse_delta: Vec2) {
        let zoom_dist = self.translation.w_axis.w.abs();
        let delta = Vec4::new(
            mouse_delta.x * self.inv_screen.x,
            -mouse_delta.y * self.inv_screen.y,
            0.0,
            0.0,
        ) * zoom_dist;
        let motion = self.inv_camera * delta;
        self.center_translation = Mat4::from_translation(Vec3::new(motion.x, motion.y, motion.z))
            * self.center_translation;
        self.update_camera();
    }
    /// Update the screen dimensions, e.g. if the window has resized.
    pub fn update_screen(&mut self, width: f32, height: f32) {
        self.inv_screen.x = 1.0 / width;
        self.inv_screen.y = 1.0 / height;
    }
    fn update_camera(&mut self) {
        self.camera = self.translation * Mat4::from_quat(self.rotation) * self.center_translation;
        self.inv_camera = self.camera.inverse();
    }
    fn screen_to_arcball(p: Vec2) -> Quat {
        let dist = p.length_squared();
        // If we're on/in the sphere return the point on it
        if dist <= 1.0 {
            Quat::from_xyzw(p.x, p.y, (1.0 - dist).sqrt(), 0.0)
        } else {
            let unit_p = p.normalize();
            Quat::from_xyzw(unit_p.x, unit_p.y, 0.0, 0.0)
        }
    }
}
