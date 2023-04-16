//! A YawPitch camera driver.

use std::{f32::consts::FRAC_PI_2, marker::PhantomData};

use glam::{EulerRot, Quat};

use dolly::{
    driver::RigDriver, handedness::Handedness, rig::RigUpdateParams, transform::Transform,
};

/// A reimplementation of [`dolly::drivers::YawPitch`] with a right handed coordinate system
#[derive(Debug)]
pub struct YawPitchZUp {
    /// [0..720)
    ///
    /// Note: Quaternions can encode 720 degrees of rotation, causing a slerp from 350 to 0 degrees
    /// to happen counter-intuitively in the negative direction; the positive direction would go through 720,
    /// thus being farther. By encoding rotation here in the 0..720 range, we reduce the risk of this happening.
    pub yaw_degrees: f32,

    /// [-90..90]
    pub pitch_degrees: f32,
}

impl Default for YawPitchZUp {
    fn default() -> Self {
        Self::new()
    }
}

impl YawPitchZUp {
    /// Creates camera looking forward along Z axis (negative or positive depends on system handedness)
    pub fn new() -> Self {
        Self {
            yaw_degrees: 0.0,
            pitch_degrees: 0.0,
        }
    }

    /// Initialize the yaw and pitch angles from a quaternion.
    /// Any roll rotation will be ignored.
    pub fn rotation_quat(mut self, rotation: Quat) -> Self {
        self.set_rotation_quat(rotation);
        self
    }

    /// Set the yaw angle in degrees.
    pub fn yaw_degrees(mut self, yaw_degrees: f32) -> Self {
        self.yaw_degrees = yaw_degrees;
        self
    }

    /// Set the pitch angle in degrees.
    pub fn pitch_degrees(mut self, pitch_degrees: f32) -> Self {
        self.pitch_degrees = pitch_degrees;
        self
    }

    /// Additively rotate by the specified angles.
    pub fn rotate_yaw_pitch(&mut self, yaw_degrees: f32, pitch_degrees: f32) {
        self.yaw_degrees = (self.yaw_degrees + yaw_degrees) % 720_f32;
        self.pitch_degrees = (self.pitch_degrees + pitch_degrees).clamp(-90.0, 90.0);
    }

    /// Set the yaw and pitch angles from a quaternion.
    /// Any roll rotation will be ignored.
    pub fn set_rotation_quat(&mut self, rotation: Quat) {
        let (yaw, pitch, _) = rotation.to_euler(EulerRot::YXZ);
        self.yaw_degrees = yaw.to_degrees();
        self.pitch_degrees = pitch.to_degrees();
    }
}

impl<H: Handedness> RigDriver<H> for YawPitchZUp {
    fn update(&mut self, params: RigUpdateParams<H>) -> Transform<H> {
        Transform {
            position: params.parent.position,
            rotation: Quat::from_euler(
                EulerRot::ZXY,
                self.yaw_degrees.to_radians(),
                self.pitch_degrees.to_radians() - FRAC_PI_2,
                0.0,
            ),
            phantom: PhantomData,
        }
    }
}
