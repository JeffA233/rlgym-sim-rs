use std::f32::consts::PI;
use std::ops;

use serde::{Serialize, Deserialize};

// use ndarray::*;

// start of helper structs

#[derive(Clone, Copy, Default, Debug, Serialize, Deserialize)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Position {
    pub fn set_vals(&mut self, x: Option<f32>, y: Option<f32>, z: Option<f32>) {
        if let Some(val) = x { self.x = val }
        if let Some(val) = y { self.y = val }
        if let Some(val) = z { self.z = val }
    }

    pub fn into_array(&self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }

    pub fn to_vec(&self) -> Vec<f32> {
        vec![self.x, self.y, self.z]
    }

    // pub fn multiply_by_vel(&self, vel: &Velocity) -> Position {
    //     let x = self.x * vel.x;
    //     let y = self.y * vel.y;
    //     let z = self.z * vel.z;
    //     Position { x, y, z }
    // }

    pub fn divide_by_var(&self, var: f32) -> Position {
        let x = self.x / var;
        let y = self.y / var;
        let z = self.z / var;
        Position { x, y, z }
    }

    pub fn norm(&self) -> f32 {
        let mut running_val = 0.;
        running_val += self.x.powi(2);
        running_val += self.y.powi(2);
        running_val += self.z.powi(2);
        running_val.sqrt()
    }

    pub fn invert(&self) -> Self {
        Self { x: -self.x, y: -self.y, z: self.z }
    }
}

impl ops::Add<Position> for Position {
    type Output = Self;

    fn add(self, other_pos: Position) -> Self::Output {
        Self {
            x: self.x + other_pos.x,
            y: self.y + other_pos.y,
            z: self.z + other_pos.z,
        }
    }
}

impl ops::Add<Velocity> for Position {
    type Output = Self;

    fn add(self, other_pos: Velocity) -> Self::Output {
        Self {
            x: self.x + other_pos.x,
            y: self.y + other_pos.y,
            z: self.z + other_pos.z,
        }
    }
}

impl ops::Sub<Position> for Position {
    type Output = Self;

    fn sub(self, rhs: Position) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Sub<Velocity> for Position {
    type Output = Self;

    fn sub(self, rhs: Velocity) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Mul<Position> for Position {
    type Output = Self;

    fn mul(self, rhs: Position) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl ops::Mul<Velocity> for Position {
    type Output = Self;

    fn mul(self, rhs: Velocity) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl ops::Div<Position> for Position {
    type Output = Self;

    fn div(self, rhs: Position) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl ops::Div<Velocity> for Position {
    type Output = Self;

    fn div(self, rhs: Velocity) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl ops::Div<f32> for Position {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl ops::Mul<f32> for Position {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Add<f32> for Position {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

impl ops::Sub<f32> for Position {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}

pub struct IterCounterPos {
    count: usize,
    position: Position,
}

impl IterCounterPos {
    fn new(pos: Position) -> Self {
        IterCounterPos { count: 0, position: pos }
    }
}

impl Iterator for IterCounterPos {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        match self.count {
            1 => Some(self.position.x),
            2 => Some(self.position.y),
            3 => Some(self.position.z),
            _ => None
        }
    }
}

impl IntoIterator for Position {
    type Item = f32;
    type IntoIter = IterCounterPos;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}

impl From<Velocity> for Position {
    fn from(value: Velocity) -> Self {
        Self { x: value.x, y: value.y, z: value.z }
    }
}

#[derive(Clone, Copy, Default, Debug, Serialize, Deserialize)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Velocity {
    pub fn set_vals(&mut self, x: Option<f32>, y: Option<f32>, z: Option<f32>) {
        if let Some(val) = x { self.x = val }
        if let Some(val) = y { self.y = val }
        if let Some(val) = z { self.z = val }
    }

    pub fn into_array(&self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }

    pub fn to_vec(&self) -> Vec<f32> {
        vec![self.x, self.y, self.z]
    }

    // pub fn multiply_by_pos(&self, other_pos: &Position) -> Velocity {
    //     let x = self.x * other_pos.x;
    //     let y = self.y * other_pos.y;
    //     let z = self.z * other_pos.z;
    //     Velocity { x, y, z }
    // }

    pub fn divide_by_var(&self, var: f32) -> Velocity {
        let x = self.x / var;
        let y = self.y / var;
        let z = self.z / var;
        Velocity { x, y, z }
    }

    pub fn norm(&self) -> f32 {
        let mut running_val = 0.;
        running_val += self.x.powi(2);
        running_val += self.y.powi(2);
        running_val += self.z.powi(2);
        running_val.sqrt()
    }

    pub fn scalar_projection(self, dest_vec: Position) -> f32 {
        let norm = dest_vec.norm();
        if norm == 0. {
            return 0.;
        }
        ((self * dest_vec).into_array().iter().sum::<f32>()) / norm
    }

    pub fn invert(&self) -> Self {
        Self { x: -self.x, y: -self.y, z: self.z }
    }
}

impl ops::Add<Velocity> for Velocity {
    type Output = Self;

    fn add(self, other_pos: Velocity) -> Self::Output {
        Self {
            x: self.x + other_pos.x,
            y: self.y + other_pos.y,
            z: self.z + other_pos.z,
        }
    }
}

impl ops::Add<Position> for Velocity {
    type Output = Self;

    fn add(self, other_pos: Position) -> Self::Output {
        Self {
            x: self.x + other_pos.x,
            y: self.y + other_pos.y,
            z: self.z + other_pos.z,
        }
    }
}

impl ops::Sub<Velocity> for Velocity {
    type Output = Self;

    fn sub(self, rhs: Velocity) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Sub<Position> for Velocity {
    type Output = Self;

    fn sub(self, rhs: Position) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Mul<Velocity> for Velocity {
    type Output = Self;

    fn mul(self, rhs: Velocity) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl ops::Mul<Position> for Velocity {
    type Output = Self;

    fn mul(self, rhs: Position) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl ops::Div<Velocity> for Velocity {
    type Output = Self;

    fn div(self, rhs: Velocity) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl ops::Div<Position> for Velocity {
    type Output = Self;

    fn div(self, rhs: Position) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl ops::Div<f32> for Velocity {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl ops::Mul<f32> for Velocity {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Add<f32> for Velocity {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

impl ops::Sub<f32> for Velocity {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}

pub struct IterCounterVel {
    count: usize,
    velocity: Velocity,
}

impl IterCounterVel {
    fn new(vel: Velocity) -> Self {
        IterCounterVel { count: 0, velocity: vel }
    }
}

impl Iterator for IterCounterVel {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        match self.count {
            1 => Some(self.velocity.x),
            2 => Some(self.velocity.y),
            3 => Some(self.velocity.z),
            _ => None
        }
    }
}

impl IntoIterator for Velocity {
    type Item = f32;
    type IntoIter = IterCounterVel;
    
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}

impl From<Position> for Velocity {
    fn from(value: Position) -> Self {
        Velocity { x: value.x, y: value.y, z: value.z }
    }
}

#[derive(Clone, Copy, Default, Debug, Serialize, Deserialize)]
pub struct Quaternion {
    pub w: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Quaternion {
    pub fn set_vals(&mut self, w: Option<f32>, x: Option<f32>, y: Option<f32>, z: Option<f32>) {
        if let Some(val) = w { self.w = val }
        if let Some(val) = x { self.x = val }
        if let Some(val) = y { self.y = val }
        if let Some(val) = z { self.z = val }
    }

    pub fn norm(&self) -> f32 {
        let mut running_val = 0.;
        running_val += self.w.powi(2);
        running_val += self.x.powi(2);
        running_val += self.y.powi(2);
        running_val += self.z.powi(2);
        running_val.sqrt()
    }

    pub fn dot(&self, quat: Quaternion) -> f32 {
        self.w * quat.w + self.x * quat.x + self.y * quat.y + self.z * quat.z
    }

    /// quat Vec to rotation matrix Array2
    pub fn quat_to_rot_mtx(&self) -> RotationMatrix {
        let mut theta = RotationMatrix::zeros();

        let norm = self.dot(*self);

        let w = -&self.w;
        let x = -&self.x;
        let y = -&self.y;
        let z = -&self.z;

        // let s: f64 = 1.0 / norm;

        if norm != 0. {
            let s: f32 = 1.0 / norm;

            // front direction
            theta.array[0][0] = 1. - 2. * s * (y * y + z * z);
            theta.array[1][0] = 2. * s * (x * y + z * w);
            theta.array[2][0] = 2. * s * (x * z - y * w);

            // left direction
            theta.array[0][1] = 2. * s * (x * y - z * w);
            theta.array[1][1] = 1. - 2. * s * (x * x + z * z);
            theta.array[2][1] = 2. * s * (y * z + x * w);

            // up direction
            theta.array[0][2] = 2. * s * (x * z + y * w);
            theta.array[1][2] = 2. * s * (y * z - x * w);
            theta.array[2][2] = 1. - 2. * s * (x * x + y * y);
        }

        theta
    }

    pub fn quat_to_euler(&self) -> EulerAngle {
        // let w = self.w;
        // let x = self.x;
        // let y = self.y;
        // let z = self.z;

        let sinr_cosp: f32 = 2. * (self.w * self.x + self.y * self.z);
        let cosr_cosp: f32 = 1. - 2. * (self.x * self.x + self.y * self.y);
        let sinp: f32 = 2. * (self.w * self.y - self.z * self.x);
        let siny_cosp: f32 = 2. * (self.w * self.z + self.x * self.y);
        let cosy_cosp: f32 = 1. - 2. * (self.y * self.y + self.z * self.z);
        let roll: f32 = sinr_cosp.atan2(cosr_cosp);

        let pitch: f32 = if sinp.abs() > 1. {
            PI / 2.
        } else {
            sinp.asin()
        };

        let yaw: f32 = siny_cosp.atan2(cosy_cosp);

        EulerAngle { pitch: -pitch, yaw, roll: -roll }
    }

    pub fn into_array(&self) -> [f32; 4] {
        [self.w, self.x, self.y, self.z]
    }

    pub fn invert(&self) -> Quaternion {
        Quaternion {
            w: self.z,
            x: self.y,
            y: -self.x,
            z: -self.w,
        }
    }
}

#[derive(Clone, Copy, Default, Debug, Serialize, Deserialize)]
pub struct EulerAngle {
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
}

impl EulerAngle {
    pub fn set_vals(&mut self, pitch: Option<f32>, yaw: Option<f32>, roll: Option<f32>) {
        if let Some(val) = pitch { self.pitch = val }
        if let Some(val) = yaw { self.yaw = val }
        if let Some(val) = roll { self.roll = val }
    }

    pub fn euler_to_rotation(&self) -> RotationMatrix {
        let cp = &self.pitch.cos();
        let cy = &self.yaw.cos();
        let cr = &self.roll.cos();

        let sp = &self.pitch.sin();
        let sy = &self.yaw.sin();
        let sr = &self.roll.sin();

        let mut theta = RotationMatrix::zeros();

        // front
        theta.array[0][0] = cp * cy;
        theta.array[1][0] = cp * sy;
        theta.array[2][0] = *sp;

        // left
        theta.array[0][1] = cy * sp * sr - cr * sy;
        theta.array[1][1] = sy * sp * sr + cr * cy;
        theta.array[2][1] = -cp * sr;

        // up
        theta.array[0][2] = -cr * cy * sp - sr * sy;
        theta.array[1][2] = -cr * sy * sp + sr * cy;
        theta.array[2][2] = cp * cr;

        theta
    }

    pub fn into_array(&self) -> [f32; 3] {
        [self.pitch, self.yaw, self.roll]
    }
}

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct RotationMatrix {
    pub array: [[f32; 3]; 3],
}

impl RotationMatrix {
    // pub fn get_val(&self, row: usize, col: usize) -> f64 {
    //     self.array[row][col]
    // }

    pub fn column(&self, col: usize) -> [f32; 3] {
        // let val1 = self.array[0][col];
        // let val2 = self.array[1][col];
        // let val3 = self.array[2][col];
        // [val1, val2, val3]
        [self.array[0][col], self.array[1][col], self.array[2][col]]
    }

    pub fn row(&self, row: usize) -> [f32; 3] {
        let val1 = self.array[row][0];
        let val2 = self.array[row][1];
        let val3 = self.array[row][2];
        [val1, val2, val3]
    }

    pub fn zeros() -> RotationMatrix {
        RotationMatrix { array: [[0.; 3]; 3] }
    }

    pub fn into_array(&self) -> [[f32; 3]; 3] {
        self.array
    }

    pub fn into_flat_array(&self) -> [f32; 9] {
        let mut row_vec = [0.; 9];
        let mut i = 0;
        for col in self.array {
            for row_val in col {
                row_vec[i] = row_val;
                i += 1;
            }
        }
        // for idx in 0..3 {
        //     let mut x = 0;
        //     for col in self.array {
        //         row_vec[i] = col[x];
        //         i += 1;
        //         x += 1;
        //     }
        // }
        row_vec
    }

    pub fn rotation_to_quaternion(&self) -> Quaternion {
        let trace = self.trace();
        let mut q = Quaternion::default();

        if trace > 0. {
            let mut s = (trace + 1.).powf(0.5);
            q.w = s * 0.5;
            s = 0.5 / s;
            q.x = (self.array[2][1] - self.array[1][2]) * s;
            q.y = (self.array[0][2] - self.array[2][0]) * s;
            q.z = (self.array[1][0] - self.array[0][1]) * s;
        } else if self.array[0][0] >= self.array[1][1] && self.array[0][0] >= self.array[2][2] {
            let s = (1. + self.array[0][0] - self.array[1][1] - self.array[2][2]).powf(0.5);
            let inv_s = 0.5 / s;
            q.x = 0.5 * s;
            q.y = (self.array[1][0] + self.array[0][1]) * inv_s;
            q.z = (self.array[2][0] + self.array[0][2]) * inv_s;
            q.w = (self.array[2][1] - self.array[1][2]) * inv_s;
        } else if self.array[1][1] > self.array[2][2] {
            let s = (1. + self.array[1][1] - self.array[0][0] - self.array[2][2]).powf(0.5);
            let inv_s = 0.5 / s;
            q.x = (self.array[0][1] + self.array[1][0]) * inv_s;
            q.y = 0.5 * s;
            q.z = (self.array[1][2] + self.array[2][1]) * inv_s;
            q.w = (self.array[0][2] - self.array[2][0]) * inv_s;
        } else {
            let s = (1. + self.array[2][2] - self.array[0][0] - self.array[1][1]).powf(0.5);
            let inv_s = 0.5 / s;
            q.x = (self.array[0][2] + self.array[2][0]) * inv_s;
            q.y = (self.array[1][2] + self.array[2][1]) * inv_s;
            q.z = 0.5 * s;
            q.w = (self.array[1][0] - self.array[0][1]) * inv_s;
        }
        q.w = -q.w;
        q.x = -q.x;
        q.y = -q.y;
        q.z = -q.z;
        q
    }

    /// Numpy-like trace function
    pub fn trace(&self) -> f32 {
        self.array[0][0] + self.array[1][1] + self.array[2][2]
    }

    pub fn invert(&self) -> RotationMatrix {
        let mut new_rot_mtx = RotationMatrix::default();
        // new_rot_mtx.array[0] = [self.column(0)[0], self.column(0)[1], self.column(0)[2]];
        // new_rot_mtx.array[1] = [self.column(1)[0], self.column(1)[1], self.column(1)[2]];
        // new_rot_mtx.array[2] = [self.column(2)[0], self.column(2)[1], self.column(2)[2]];
        new_rot_mtx.array[0] = [-self.column(0)[0], -self.column(1)[0], -self.column(2)[0]];
        new_rot_mtx.array[1] = [-self.column(0)[1], -self.column(1)[1], -self.column(2)[1]];
        new_rot_mtx.array[2] = [self.column(0)[2], self.column(1)[2], self.column(2)[2]];
        new_rot_mtx
    }
}

// end of helper structs
// -------------------------------------------------------------------------------------------
// start of PhysicsObject struct

/// Struct that holds any kind of physics data for car/ball
#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct PhysicsObject {
    pub position: Position,
    pub quaternion: Quaternion,
    pub linear_velocity: Velocity,
    pub angular_velocity: Velocity,
    pub euler_angles: EulerAngle,
    pub rotation_mtx: RotationMatrix,
    pub has_computed_rot_mtx: bool,
    pub has_computed_euler_angles: bool,
}

impl PhysicsObject {
    pub fn new() -> Self {
        PhysicsObject {
            position: Position::default(),
            quaternion: Quaternion::default(),
            linear_velocity: Velocity::default(),
            angular_velocity: Velocity::default(),
            euler_angles: EulerAngle::default(),
            rotation_mtx: RotationMatrix::zeros(),
            has_computed_euler_angles: false,
            has_computed_rot_mtx: false,
        }
    }

    pub fn decode_car_data(&mut self, car_data: &[f32]) {
        self.position.set_vals(Some(car_data[0]), Some(car_data[1]), Some(car_data[2]));
        self.quaternion.set_vals(Some(car_data[3]), Some(car_data[4]), Some(car_data[5]), Some(car_data[6]));
        self.linear_velocity.set_vals(Some(car_data[7]), Some(car_data[8]), Some(car_data[9]));
        self.angular_velocity.set_vals(Some(car_data[10]), Some(car_data[11]), Some(car_data[12]));
    }

    pub fn decode_ball_data(&mut self, ball_data: &[f32]) {
        self.position.set_vals(Some(ball_data[0]), Some(ball_data[1]), Some(ball_data[2]));
        self.linear_velocity.set_vals(Some(ball_data[3]), Some(ball_data[4]), Some(ball_data[5]));
        self.angular_velocity.set_vals(Some(ball_data[6]), Some(ball_data[7]), Some(ball_data[8]));
    }

    pub fn forward(&self) -> [f32; 3] {
        // let arr = &self.rotation_mtx();
        // arr.column(0)
        self.rotation_mtx.column(0)
    }

    pub fn right(&self) -> [f32; 3] {
        // let arr = self.rotation_mtx();
        // arr.column(1)
        self.rotation_mtx.column(1)
    }

    pub fn left(&self) -> [f32; 3] {
        let arr = self.rotation_mtx;
        let mut partial_arr = arr.column(1);
        for val in partial_arr.iter_mut() {
            *val *= -1.;
        }
        partial_arr
    }

    pub fn up(&self) -> [f32; 3] {
        // let arr = self.rotation_mtx();
        // arr.column(2)
        self.rotation_mtx.column(2)
    }

    pub fn pitch(&self) -> f32 {
        self.euler_angles.pitch
    }

    pub fn yaw(&self) -> f32 {
        self.euler_angles.yaw
    }

    pub fn roll(&self) -> f32 {
        self.euler_angles.roll
    }

    pub fn euler_angles(&self) -> EulerAngle {
        // if !self.has_computed_euler_angles {
        //     self.euler_angles = self.quaternion.quat_to_euler();
        //     self.has_computed_euler_angles = true;
        // }
        self.euler_angles
    }

    pub fn rotation_mtx(&self) -> RotationMatrix {
        // if !self.has_computed_rot_mtx {
        //     self.rotation_mtx = self.quaternion.quat_to_rot_mtx();
        //     self.has_computed_rot_mtx = true;
        // }
        self.rotation_mtx
    }

    pub fn serialize_to_vec(&mut self) -> Vec<f32> {
        let mut repr = Vec::<f32>::with_capacity(25);

        repr.extend(self.position.into_array().iter());
        repr.extend(self.quaternion.into_array().iter());
        repr.extend(self.linear_velocity.into_array().iter());
        repr.extend(self.angular_velocity.into_array().iter());
        repr.extend(self.euler_angles.into_array().iter());

        // let mut row_vec = Vec::<f64>::with_capacity(9);
        let row_vec = self.rotation_mtx().into_flat_array();
        repr.extend(row_vec.iter());

        repr
    }
}
