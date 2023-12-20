use crate::gamestates::game_state::GameState;
use crate::envs::game_match::{GameMatch, GameConfig};
use crate::make::RenderConfig;
use crate::obs_builders::obs_builder::ObsBuilder;
use crate::render::renderer::Renderer;

// use subprocess::Popen;

use std::collections::HashMap;
use std::io;

// use std::thread;
// use std::time::Duration;

/// Base Gym struct for RLGym-Rust.
/// 
/// See 
pub struct Gym {
    pub _game_match: GameMatch,
    pub observation_space: Vec<usize>,
    pub action_space: Vec<usize>,
    pub _prev_state: GameState,
    renderer: Option<Renderer>,
}

impl Gym {
    /// Creates a new instance of a gym and launches + connects to Rocket League instance
    pub fn new(game_match: GameMatch, render_config: RenderConfig) -> Self {
        let observation_space = game_match.observation_space.clone();
        let action_space = game_match.action_space.clone();
        let renderer = if render_config.render {
            let render_op = Renderer::new(render_config);
            match render_op {
                Ok(val) => {
                    Some(val)
                },
                Err(e) => {
                    println!("Unable to start rendering due to error: {e}");
                    None
                }
            }
        } else {
            None
        };

        let mut gym = Gym {
            _game_match: game_match,
            observation_space,
            action_space,
            _prev_state: GameState::new(None),
            renderer,
        };

        gym._prev_state = gym.receive_state();
        gym.reset(None, None);

        gym
    }

    pub fn reset(&mut self, _return_info: Option<bool>, seed: Option<u64>) -> Vec<Vec<f32>> {
        // let _return_info = match _return_info {
        //     Some(return_info) => return_info,
        //     None => false
        // };
        if let Some(seed) = seed { self._game_match.set_seeds(seed) };

        let state_wrapper = self._game_match.get_reset_state(&self._prev_state);

        // set the sim state and get the state from the sim
        let gym_state = if self.renderer.is_some() {
            let (gym_state, sim_state) = self._game_match.sim_wrapper.set_state(state_wrapper, true);

            let render_op = self.renderer.as_mut().unwrap().step(vec![sim_state.unwrap()]);
            match render_op {
                Ok(_) => {
                    // self.renderer = Some(val);
                    // return Ok(())
                },
                Err(e) => {
                    println!("Unable to do rendering in reset due to error: {e}, attempting to close renderer");
                    let close_op = self.renderer.as_mut().unwrap().close();
                    match close_op {
                        Ok(_) => {
                            // self.renderer = Some(val);
                            // return Ok(())
                        },
                        Err(e) => {
                            println!("Unable to close renderer in reset due to error: {e}");
                            // return Err(e)
                        }
                    }
                }
            }

            gym_state
        } else {
            let (gym_state, _) = self._game_match.sim_wrapper.set_state(state_wrapper, false);

            gym_state
        };
        // let state = self._game_match.sim_wrapper.set_state(state_wrapper);

        self._game_match.episode_reset(&gym_state);
        self._prev_state = gym_state.clone();

        self._game_match.build_observations(&gym_state)
        // TODO return Option except that state and get_result don't match
        // if _return_info {
        //     let mut h_m = HashMap::<&str,f64>::new();
        //     h_m.insert("result", self._game_match.get_result(state) as f64);
        // }
    }

    pub fn step(&mut self, actions: Vec<Vec<f32>>) -> (Vec<Vec<f32>>, Vec<f32>, bool, HashMap<String, f32>) {
        let actions = self._game_match.parse_actions(actions, &self._prev_state);
        // let act_res = self._send_actions(actions);

        // if !act_res {
        //     // self.close();
        //     panic!("closed gym because of action error")
        // }

        // let state = self._receive_state();
        // let state = self._game_match.sim_wrapper.step(actions);
        // set the sim state and get the state from the sim
        let gym_state = if self.renderer.is_some() {
            let (gym_state, sim_state) = self._game_match.sim_wrapper.step(actions, true);
            
            let render_op = self.renderer.as_mut().unwrap().step(sim_state.unwrap());
            match render_op {
                Ok(_) => {
                    // self.renderer = Some(val);
                    // return Ok(())
                },
                Err(e) => {
                    println!("Unable to do rendering in reset due to error: {e}, attempting to close renderer");
                    let close_op = self.renderer.as_mut().unwrap().close();
                    match close_op {
                        Ok(_) => {
                            // self.renderer = Some(val);
                            // return Ok(())
                        },
                        Err(e) => {
                            println!("Unable to close renderer in reset due to error: {e}");
                            // return Err(e)
                        }
                    }
                }
            }

            gym_state
        } else {
            let (gym_state, _) = self._game_match.sim_wrapper.step(actions, false);

            gym_state
        };

        let obs = self._game_match.build_observations(&gym_state);
        let done = self._game_match.is_done(&gym_state);
        self._prev_state = gym_state.clone();
        let reward = self._game_match.get_rewards(&gym_state, done);
        let mut info = HashMap::<String, f32>::new();
        info.insert("result".to_string(), self._game_match.get_result(&gym_state) as f32);
        (obs, reward, done, info)
    }

    pub fn close_renderer(&mut self) {
        if self.renderer.is_some() {
            let close_op = self.renderer.as_mut().unwrap().close();
            match close_op {
                Ok(_) => {
                    // self.renderer = Some(val);
                    // return Ok(())
                },
                Err(e) => {
                    println!("Unable to close renderer due to error: {e}");
                    // return Err(e)
                }
            }
            self.renderer = None;
        } else {
            println!("Close rendered was called but did nothing as there was no renderer")
        }
    }

    pub fn try_render(&mut self, render_config: RenderConfig) -> Result<(), io::Error> {
        let render_op = Renderer::new(render_config);
        match render_op {
            Ok(val) => {
                self.renderer = Some(val);
                Ok(())
            },
            Err(e) => {
                println!("Unable to do rendering due to error: {e}");
                Err(e)
            }
        }
    }

    pub fn update_config(&mut self, new_config: GameConfig, new_obs: Option<Vec<Box<dyn ObsBuilder>>>) {
        self._prev_state = self._game_match.update_settings(new_config, new_obs);
    }

    // pub fn close(&mut self) {
    //     self._game_process.terminate().unwrap();
    //     self._comm_handler.close_pipe();
    // }

    fn receive_state(&mut self) -> GameState {
        // let message = self._comm_handler.receive_message(Some(RLGYM_STATE_MESSAGE_HEADER.to_vec()));
        // if message.body[0] == -999999. {
        //     self.close();
        //     panic!("panicked gym because of comm error");
        // }
        self._game_match.get_state()
    }

    fn _send_actions(&mut self, actions: Vec<Vec<f32>>) -> bool {
        let mut result = true;
        for action in &actions {
            // assert!(action.len() == 8, "action was not of length 8");
            if action.len() != 8 {
                let act_str = format!("{:?}", action);
                println!("action was not of length 8, action was: {act_str}");
                result = false;
            }
        }

        // let actions_formatted = self._game_match.format_actions(actions);

        // self._comm_handler.send_message(None, Some(RLGYM_AGENT_ACTION_IMMEDIATE_RESPONSE_MESSAGE_HEADER.to_vec()), Some(actions_formatted));
        //

        result
    }
}
