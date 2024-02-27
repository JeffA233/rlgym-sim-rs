// use numpy::*;
// use ndarray::*;

// pub mod math{
// use std::f32::consts::PI;
// use std::f32::consts::PI;

// use numpy::*;
// use ndarray::*;
use rand::{rngs::SmallRng, thread_rng, Rng};

use crate::gamestates::physics_object::RotationMatrix;

// use crate::gamestates::physics_object::Quaternion;

/// clips all of the values in place to the range between high and low
pub fn clip(vec: &mut [f32], high: f32, low: f32) {
    for val in vec.iter_mut() {
        *val = if *val > high {
            high
        } else if *val < low {
            low
        } else {
            *val
        }
    }
}

/// Numpy-like trace function
pub fn trace(arr: &RotationMatrix) -> f32 {
    arr.array[0][0] + arr.array[1][1] + arr.array[2][2]
}

/// divide a vec by a given variable
pub fn vec_div_variable(a: &[f32], b: &f32) -> Vec<f32> {
    a.iter().map(|x| *x / *b).collect()
}

/// multiply elementwise vec a * vec b
pub fn element_mult_vec(a: &[f32], b: &[f32]) -> Vec<f32> {
    assert!(a.len() == b.len(), "length of a did not match length of b");

    std::iter::zip(a, b).map(|(x, y)| x * y).collect()
}

/// divide elementwise vec a / vec b
pub fn element_div_vec(a: &[f32], b: &[f32]) -> Vec<f32> {
    assert!(a.len() == b.len(), "length of a did not match length of b");

    std::iter::zip(a, b).map(|(x, y)| x / y).collect()
}

/// subtract elementwise vec b from vec a
pub fn element_sub_vec(a: &[f32], b: &[f32]) -> Vec<f32> {
    assert!(a.len() == b.len(), "length of a did not match length of b");

    std::iter::zip(a, b).map(|(x, y)| x - y).collect()
}

/// add elementwise vec a + vec b
pub fn element_add_vec(a: &[f32], b: &[f32]) -> Vec<f32> {
    assert!(a.len() == b.len(), "length of a did not match length of b");

    std::iter::zip(a, b).map(|(x, y)| x + y).collect()
}

/// subtract elements of two vecs to get dist
pub fn get_dist(a: &[f32], b: &[f32]) -> Vec<f32> {
    element_sub_vec(a, b)
}

/// Vector projection of two vecs and an optional mag_squared.
/// 
/// Does not use &[f32] as it would require an extra copy for some conditions.
pub fn vector_projection(vec: Vec<f32>, dest_vec: Vec<f32>, mag_squared: Option<f32>) -> Vec<f32> {
    assert!(vec.len() == dest_vec.len(), "length of a did not match length of b");
    // let mut _mag_squared: f32;

    let mut _mag_squared = match mag_squared {
        Some(mag_squared) => {
            if mag_squared == 0. {
                return dest_vec;
            } else {
                mag_squared
            }
        }
        None => {
            let norm = norm_func(&vec);
            if norm == 0. {
                return dest_vec;
            } else {
                norm * norm
            }
        }
    };

    let dot_prod = element_mult_vec(&vec, &dest_vec).iter().sum::<f32>();

    let part = dot_prod / _mag_squared;
    dest_vec.into_iter().map(|x| x * part).collect()
}

/// get norm of vec
pub fn norm_func(nums: &[f32]) -> f32 {
    nums.iter().map(|x| x.powi(2)).sum::<f32>().sqrt()
}

pub fn scalar_projection(vec: &[f32], dest_vec: &[f32]) -> f32 {
    let norm = norm_func(dest_vec);
    if norm == 0. {
        return 0.;
    }
    return (element_mult_vec(vec, dest_vec).iter().sum::<f32>()) / norm;
}

/// norm squared
pub fn squared_vecmag(vec: &[f32]) -> f32 {
    norm_func(vec).powi(2)
}

/// equal to the norm
pub fn vecmag(vec: &[f32]) -> f32 {
    norm_func(vec)
}

/// vec / norm
pub fn unitvec(vec: &[f32]) -> Vec<f32> {
    let vecm: f32 = norm_func(vec);
    vec_div_variable(vec, &vecm)
}

/// returns 0. if either the norm of a or b is equal to 0.
pub fn cosine_similarity(a: Vec<f32>, b: Vec<f32>) -> f32 {
    let a_norm = norm_func(&a);
    let b_norm = norm_func(&b);

    if a_norm == 0. || b_norm == 0. {
        return 0.
    };

    let a_vec = vec_div_variable(&a, &a_norm);
    let b_vec = vec_div_variable(&b, &b_norm);

    std::iter::zip(a_vec, b_vec).map(|(a, b)| a * b).sum()
}

// pub fn quat_to_euler(quat: &[f32]) -> Vec<f32> {
//     assert!(quat.len() == 4, "quat is not the correct shape");

//     let w = quat[0];
//     let x = quat[1];
//     let y = quat[2];
//     let z = quat[3];

//     let sinr_cosp = 2. * (w * x + y * z);
//     let cosr_cosp = 1. - 2. * (x * x + y * y);
//     let sinp = 2. * (w * y - z * x);
//     let siny_cosp = 2. * (w * z + x * y);
//     let cosy_cosp = 1. - 2. * (y * y + z * z);
//     let roll = sinr_cosp.atan2(cosr_cosp);

//     let yaw = siny_cosp.atan2(cosy_cosp);

//     let pitch = if sinp.abs() > 1. {
//         PI / 2.
//     } else {
//         sinp.asin()
//     };

//     vec![-pitch, yaw, -roll]
// }

// /// quat Vec to rotation matrix Array2
// pub fn quat_to_rot_mtx(nums: &[f32]) -> Array2<f32> {
//     let mut theta = Array2::<f32>::zeros((3, 3));

//     assert!(nums.len() == 4, "nums is not the correct shape");

//     let norm: f32 = nums
//         .clone()
//         .into_iter()
//         .map(|x: f32| x.powf(2.))
//         // .collect::<Vec<f64>>()
//         // .iter()
//         .sum();

//     let w = -&nums[0];
//     let x = -&nums[1];
//     let y = -&nums[2];
//     let z = -&nums[3];

//     // let s: f64 = 1.0 / norm;

//     if norm != 0. {
//         let s: f32 = 1.0 / norm;

//         theta[[0, 0]] = 1. - 2. * s * (y * y + z * z);
//         theta[[1, 0]] = 2. * s * (x * y + z * w);
//         theta[[2, 0]] = 2. * s * (x * z - y * w);

//         // left direction
//         theta[[0, 1]] = 2. * s * (x * y - z * w);
//         theta[[1, 1]] = 1. - 2. * s * (x * x + z * z);
//         theta[[2, 1]] = 2. * s * (y * z + x * w);

//         // up direction
//         theta[[0, 2]] = 2. * s * (x * z + y * w);
//         theta[[1, 2]] = 2. * s * (y * z - x * w);
//         theta[[2, 2]] = 1. - 2. * s * (x * x + y * y);
//     }

//     theta
// }

// pub fn rotation_to_quaternion(m: RotationMatrix) -> Quaternion {
//     let trace = trace(&m);
//     let mut q = Quaternion::default();

//     if trace > 0. {
//         let mut s = (trace + 1.).powf(0.5);
//         q.w = s * 0.5;
//         s = 0.5 / s;
//         q.x = (m.array[2][1] - m.array[1][2]) * s;
//         q.y = (m.array[0][2] - m.array[2][0]) * s;
//         q.z = (m.array[1][0] - m.array[0][1]) * s;
//     } else if m.array[0][0] >= m.array[1][1] && m.array[0][0] >= m.array[2][2] {
//         let s = (1. + m.array[0][0] - m.array[1][1] - m.array[2][2]).powf(0.5);
//         let inv_s = 0.5 / s;
//         q.x = 0.5 * s;
//         q.y = (m.array[1][0] + m.array[0][1]) * inv_s;
//         q.z = (m.array[2][0] + m.array[0][2]) * inv_s;
//         q.w = (m.array[2][1] - m.array[1][2]) * inv_s;
//     } else if m.array[1][1] > m.array[2][2] {
//         let s = (1. + m.array[1][1] - m.array[0][0] - m.array[2][2]).powf(0.5);
//         let inv_s = 0.5 / s;
//         q.x = (m.array[0][1] + m.array[1][0]) * inv_s;
//         q.y = 0.5 * s;
//         q.z = (m.array[1][2] + m.array[2][1]) * inv_s;
//         q.w = (m.array[0][2] - m.array[2][0]) * inv_s;
//     } else {
//         let s = (1. + m.array[2][2] - m.array[0][0] - m.array[1][1]).powf(0.5);
//         let inv_s = 0.5 / s;
//         q.x = (m.array[0][2] + m.array[2][0]) * inv_s;
//         q.y = (m.array[1][2] + m.array[2][1]) * inv_s;
//         q.z = 0.5 * s;
//         q.w = (m.array[1][0] - m.array[0][1]) * inv_s;
//     }
//     q.w = -q.w;
//     q.x = -q.x;
//     q.y = -q.y;
//     q.z = -q.z;
//     q
// }

// pub fn euler_to_rotation(pyr: EulerAngle) -> RotationMatrix {
//     let cp = pyr.pitch.cos();
//     let cy = pyr.yaw.cos();
//     let cr = pyr.roll.cos();

//     let sp = pyr.pitch.sin();
//     let sy = pyr.yaw.sin();
//     let sr = pyr.roll.sin();

//     let mut theta = RotationMatrix::zeros();

//     // front
//     theta.array[0][0] = cp * cy;
//     theta.array[1][0] = cp * sy;
//     theta.array[2][0] = sp;

//     // left
//     theta.array[0][1] = cy * sp * sr - cr * sy;
//     theta.array[1][1] = sy * sp * sr + cr * cy;
//     theta.array[2][1] = -cp * sr;

//     // up
//     theta.array[0][2] = -cr * cy * sp - sr * sy;
//     theta.array[1][2] = -cr * sy * sp + sr * cy;
//     theta.array[2][2] = cp * cr;

//     theta
// }

/// initializes a randomized Vec<f32> of length 3
pub fn rand_uvec3() -> Vec<f32> {
    let mut vec: Vec<f32> = Vec::new();
    let mut rng = thread_rng();
    let rand_num = rng.gen_range((0.)..1.);
    for _ in 0..3 {
        vec.push(rand_num - 0.5);
    }
    let norm_vec = norm_func(&vec);
    for i in vec.iter_mut() {
        *i /= norm_vec;
    }
    vec
}

/// with respect to a max_norm and the rng, randomly creates a new Vec<f32> of length 3
pub fn rand_vec3(max_norm: f32, rng: &mut SmallRng) -> Vec<f32> {
    let mut res: Vec<f32> = vec![0., 0., 0.];
    for i in res.iter_mut() {
        let rand_num = rng.gen::<f32>();
        let partial = rand_num * max_norm;
        *i = partial;
    }
    // get norm
    let norm: f32 = res.iter().map(|&x| x * x).sum::<f32>().sqrt();
    // Normalize to the max_norm
    for i in &mut res {
        *i *= max_norm / norm;
    }
    res
}
