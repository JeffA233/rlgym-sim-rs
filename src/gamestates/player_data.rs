use rocketsim_rs::sim::{BallHitInfo, CarControls};
use serde::{Deserialize, Serialize};

use crate::gamestates::physics_object::PhysicsObject;

/// Struct which holds extra data for agents/players aside from just the PhysicsObjects
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct PlayerData {
    pub car_id: i32,
    pub team_num: i32,
    pub match_goals: i64,
    pub match_saves: i64,
    pub match_shots: i64,
    pub match_demolishes: i64,
    pub boost_pickups: i64,
    pub is_demoed: bool,
    pub last_bumped_by: u32,
    pub last_bumpee: u32,
    pub bumps: u32,
    pub been_bumped: u32,
    pub on_ground: bool,
    pub ball_touched: bool,
    #[serde(skip)]
    pub ball_info: BallHitInfo,
    pub has_jump: bool,
    pub has_flip: bool,
    pub boost_amount: f32,
    pub car_data: PhysicsObject,
    pub inverted_car_data: PhysicsObject,
    pub last_ball_touch_tick: u64,
    #[serde(skip)]
    pub last_actions: CarControls,
}

impl PlayerData {
    pub fn new() -> Self {
        // Default::default()
        PlayerData {
            car_id: -1,
            team_num: -1,
            match_goals: -1,
            match_saves: -1,
            match_shots: -1,
            match_demolishes: -1,
            boost_pickups: -1,
            is_demoed: false,
            last_bumped_by: 0,
            last_bumpee: 0,
            bumps: 0,
            been_bumped: 0,
            on_ground: false,
            ball_touched: false,
            ball_info: BallHitInfo::default(),
            has_jump: false,
            has_flip: false,
            boost_amount: -1.,
            car_data: PhysicsObject::new(),
            inverted_car_data: PhysicsObject::new(),
            last_ball_touch_tick: 0,
            last_actions: CarControls::default(),
        }
    }
}

impl Default for PlayerData {
    fn default() -> Self {
        Self::new()
    }
}
