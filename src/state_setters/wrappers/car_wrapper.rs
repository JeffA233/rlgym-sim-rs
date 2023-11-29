use crate::gamestates::{
    physics_object::{EulerAngle, Position, Velocity},
    player_data::PlayerData,
};

/// Car wrapper that allows for easy modification of all of the units in a car (PlayerData, PhysicsObject)
pub struct CarWrapper {
    pub rotation: EulerAngle,
    pub team_num: i32,
    pub id: i32,
    pub boost: f32,
    pub position: Position,
    pub linear_velocity: Velocity,
    pub angular_velocity: Velocity,
}

impl CarWrapper {
    pub fn new(team_num: Option<i32>, id: Option<i32>, player_data: Option<&PlayerData>) -> Self {
        let team_num = team_num.unwrap_or(-1);
        // let id = id.unwrap_or(-1);
        match player_data {
            Some(player_data) => CarWrapper::_read_from_player_data(player_data),
            None => CarWrapper {
                rotation: EulerAngle { pitch: 0., yaw: 0., roll: 0. },
                team_num,
                id: id.unwrap_or(-1),
                boost: 0.,
                position: Position { x: 0., y: 0., z: 0. },
                linear_velocity: Velocity { x: 0., y: 0., z: 0. },
                angular_velocity: Velocity { x: 0., y: 0., z: 0. },
            },
        }
    }

    fn _read_from_player_data(player_data: &PlayerData) -> CarWrapper {
        CarWrapper {
            rotation: player_data.car_data.euler_angles(),
            team_num: player_data.team_num,
            id: player_data.car_id,
            boost: player_data.boost_amount,
            position: player_data.car_data.position,
            linear_velocity: player_data.car_data.linear_velocity,
            angular_velocity: player_data.car_data.angular_velocity,
        }
    }

    pub fn set_rot(&mut self, pitch: Option<f32>, yaw: Option<f32>, roll: Option<f32>) {
        if let Some(pitch) = pitch { self.rotation.pitch = pitch }
        if let Some(yaw) = yaw { self.rotation.yaw = yaw }
        if let Some(roll) = roll { self.rotation.roll = roll }
    }

    pub fn set_pos(&mut self, x: Option<f32>, y: Option<f32>, z: Option<f32>) {
        if let Some(x) = x { self.position.x = x }
        if let Some(y) = y { self.position.y = y }
        if let Some(z) = z { self.position.z = z }
    }

    pub fn set_lin_vel(&mut self, x: Option<f32>, y: Option<f32>, z: Option<f32>) {
        if let Some(x) = x { self.linear_velocity.x = x }
        if let Some(y) = y { self.linear_velocity.y = y }
        if let Some(z) = z { self.linear_velocity.z = z }

    }

    pub fn set_ang_vel(&mut self, x: Option<f32>, y: Option<f32>, z: Option<f32>) {
        if let Some(x) = x { self.angular_velocity.x = x }
        if let Some(y) = y { self.angular_velocity.y = y }
        if let Some(z) = z { self.angular_velocity.z = z }
    }

    // pub fn encode(&self) -> Vec<f64> {
    //     let mut vec = Vec::<f64>::new();

    //     vec.push(self.id as f64);
    //     vec.extend(self.position.into_array().iter());
    //     vec.extend(self.linear_velocity.into_array().iter());
    //     vec.extend(self.angular_velocity.into_array().iter());
    //     vec.extend(self.rotation.into_array().iter());
    //     vec.push(self.boost);

    //     // let vec_str: Vec<String>;

    //     // vec_str = vec.iter().map(|x| x.to_string()).collect();
    //     // let str = vec_str.join(" ");
    //     // format!("{id} {str} {boost}")
    //     return vec
    // }
}
