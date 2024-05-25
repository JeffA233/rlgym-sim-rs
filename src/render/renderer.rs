use std::{
    io,
    net::{UdpSocket, SocketAddr},
    // sync::mpsc::{channel, Receiver},
    thread::sleep,
    time::{Duration, Instant},
    process::Command,
};

// use rocketsim_rs::autocxx::WithinUniquePtr;
use rocketsim_rs::{
    bytes::{
        // FromBytes, 
        ToBytes, FromBytes, FromBytesExact
    },
    // cxx::UniquePtr,
    // math::Vec3,
    // sim::{Arena, ArenaMemWeightMode, BallState, CarConfig, CarControls, GameMode, Team},
    GameState as GameState_sim,
};

use crate::make::RenderConfig;

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
enum UdpPacketTypes {
    Quit,
    GameState,
    Connection,
    Paused,
    Speed,
    Render,
}

impl From<u8> for UdpPacketTypes {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::Quit,
            1 => Self::GameState,
            2 => Self::Connection,
            3 => Self::Paused,
            4 => Self::Speed,
            5 => Self::Render,
            _ => panic!("Invalid packet type"),
        }
    }
}

pub struct Renderer {
    socket: UdpSocket,
    interval: Duration,
    // NOTE: 2911 for 3v3, 1316 for 1v1, not sure if it matters really
    min_buf: [u8; 65536],
    next_time: Instant,
    // ctrlc_recv: Receiver<()>,
    sock_addr: SocketAddr,
    pause: bool,
}

const RLVISER_PATH: &str = if cfg!(windows) { "./rlviser.exe" } else { "./rlviser" };

impl Renderer {
    pub fn new(
        render_config: RenderConfig, 
    ) -> Result<Self, io::Error> {
        let socket_op = UdpSocket::bind("0.0.0.0:34254");
        // print the socket address
        let socket = match socket_op {
            Ok(val) => {
                match val.local_addr() {
                    Ok(val) => {
                        println!("Renderer in gym listening on {val}");
                    }
                    Err(e) => {
                        println!("Couldn't get local address due to err: {e}, continuing rendering anyways");
                    }
                };

                val
            },
            Err(e) => {
                println!("Unable to bind to socket, err: {e}, continuing but not rendering");
                return Err(e)
            }
        };

        if let Err(e) = Command::new(RLVISER_PATH).spawn() {
            println!("Failed to launch RLViser ({RLVISER_PATH}): {e}");
        }

        let mut buf = [0; 1];

        let res = socket.set_read_timeout(Some(Duration::from_secs(30)));
        match res {
            Ok(val) => val,
            Err(e) => {
                println!("Could not set timeout, err: {e}, stopping rendering");
                return Err(e)
            },
        };

        let sock_res = socket.recv_from(&mut buf);
        let (_, src) = match sock_res {
            Ok(val) => val,
            Err(e) => {
                println!("Couldn't receive from buffer, err: {e}, stopping rendering");
                return Err(e)
            }
        };

        if buf[0] == 2 {
            println!("Renderer connection established to {src}");
        }

        let res = socket.set_read_timeout(None);
        match res {
            Ok(val) => val,
            Err(e) => println!("Could not set timeout, err: {e}, continuing"),
        };

        let res = socket.set_nonblocking(true);
        match res {
            Ok(val) => val,
            Err(e) => println!("Could not set to nonblocking, err: {e}, continuing"),
        };

        let res = socket.send_to(&[UdpPacketTypes::Connection as u8], src);
        match res {
            Ok(_) => (),
            Err(e) => {
                println!("Could not send connection packet, err: {e}, stopping rendering"); 
                return Err(e)
            },
        };

        // let (sender, receiver) = channel();
    
        // // Setup Ctrl+C handler
        // let ctrlc_res = ctrlc::set_handler(move || {
        //     // Send a signal to the main thread to break the loop
        //     // If we can't send the signal for some reason,
        //     // then panic the process to shut down
        //     sender.send(()).unwrap();
        // });

        // match ctrlc_res {
        //     Ok(val) => val,
        //     Err(e) => {
        //         println!("Could not create ctrl-c handler, err: {e}, stopping rendering");
        //         return Err(io::Error::new(io::ErrorKind::Other, e))
        //     }
        // };

        let step_speed = render_config.update_rate / 120.;
        let step_speed_f_bytes = step_speed.to_le_bytes();
        let res = socket.send_to(&[UdpPacketTypes::Speed as u8], src);
        match res {
            Ok(_) => (),
            Err(e) => {
                println!("Could not send speed packet, err: {e}, continuing"); 
            },
        };
        let res = socket.send_to(&step_speed_f_bytes, src);
        match res {
            Ok(_) => (),
            Err(e) => {
                println!("Could not send speed packet float, err: {e}, continuing"); 
            },
        };

        let paused = [UdpPacketTypes::Paused as u8];
        let res = socket.send_to(&paused, src);
        match res {
            Ok(_) => (),
            Err(e) => {
                println!("Could not send pause packet, err: {e}, continuing"); 
            },
        };
        let res = socket.send_to(&[0], src);
        match res {
            Ok(_) => (),
            Err(e) => {
                println!("Could not send pause u8, err: {e}, continuing"); 
            },
        };

        // set the update rate for the rendering
        let interval = Duration::from_secs_f32(1. / render_config.update_rate);
        let next_time = Instant::now() + interval;
        let min_state_set_buf = [0; 65536];

        let mut inst =             
        Self {
            socket,
            interval,
            min_buf: min_state_set_buf,
            next_time,
            // ctrlc_recv: receiver,
            sock_addr: src,
            pause: false,
        };

        let res = inst.handle_ret_msg_init();
        match res {
            Ok(_) => (),
            Err(e) => {
                println!("Could not receive message from rlviser, err: {e}");
                return Err(e)
            }
        };

        Ok(inst)
    }

    pub fn step(&mut self, states: Vec<GameState_sim>) -> io::Result<Option<GameState_sim>> {
        // if self.ctrlc_recv.try_recv().is_ok() {
        //     let res = self.socket.send_to(&[UdpPacketTypes::Quit as u8], self.sock_addr);
        //     match res {
        //         Ok(val) => val,
        //         Err(e) => {
        //             println!("Could not send quit signal to rlviser, err: {e}");
        //             return Err(e)
        //         }
        //     };

        //     return Ok(())
        // }

        let mut state_set_data = None;
        for state in states.into_iter() {
            // this is more just to handle if anything gets sent back
            let res = self.handle_ret_msg();
            match res {
                Ok(val) => {
                    state_set_data = val
                },
                Err(e) => {
                    println!("Could not receive state signal from rlviser, err: {e}");
                    return Err(e)
                }
            };
    
            // send packet type
            let res = self.socket.send_to(&[UdpPacketTypes::GameState as u8], self.sock_addr);
            match res {
                Ok(val) => val,
                Err(e) => {
                    println!("Could not send state signal to rlviser, err: {e}");
                    return Err(e)
                }
            };
    
            // then send the data
            let res = self.socket.send_to(&state.to_bytes(), self.sock_addr);
            match res {
                Ok(val) => val,
                Err(e) => {
                    println!("Could not send state data to rlviser, err: {e}");
                    return Err(e)
                }
            };

            // sleep timer for tps
            let wait_time = self.next_time - Instant::now();
            if wait_time > Duration::default() {
                sleep(wait_time);
            }
            self.next_time += self.interval;
        }

        while self.pause {
            // reuse sleep timer to handle waiting for pause/unpause
            let wait_time = self.next_time - Instant::now();
            if wait_time > Duration::default() {
                sleep(wait_time);
            }
            // slightly more robust potentially in case of delays (probably unnecessary)
            self.next_time = Instant::now() + Duration::from_secs_f32(0.05);
            
            let res = self.handle_ret_msg();
            match res {
                Ok(val) => {
                    state_set_data = val
                },
                Err(e) => {
                    println!("Could not receive state signal from rlviser, err: {e}");
                    return Err(e)
                }
            };
        }
        if let Some(data) = state_set_data {
            return Ok(Some(GameState_sim::from_bytes(&data)))
        }

        Ok(None)
    }

    pub fn close(&mut self) -> io::Result<()> {
        let res = self.socket.send_to(&[UdpPacketTypes::Quit as u8], self.sock_addr);
        match res {
            Ok(val) => val,
            Err(e) => {
                println!("Could not send quit signal to rlviser when closing, err: {e}");
                return Err(e)
            }
        };

        Ok(())
    }

    // fn handle_state_set(
    //     min_state_set_buf: &mut [u8; GameState_sim::MIN_NUM_BYTES],
    //     socket: &UdpSocket,
    //     // arena: &mut UniquePtr<Arena>,
    // ) -> io::Result<bool> {
    //     let mut state_set_buf = Vec::new();
    
    //     while socket.peek_from(min_state_set_buf).is_ok() {
    //         // the socket sent data back
    //         // this is the other side telling us to update the game state
    //         let num_bytes = GameState_sim::get_num_bytes(min_state_set_buf);
    //         state_set_buf = vec![0; num_bytes];
    //         socket.recv_from(&mut state_set_buf)?;
    //     }
    
    //     // the socket didn't send data back
    //     if state_set_buf.is_empty() {
    //         return Ok(false);
    //     }
    
    //     // set the game state
    //     // let game_state = GameState_sim::from_bytes(&state_set_buf);
    //     // if let Err(e) = arena.pin_mut().set_game_state(&game_state) {
    //     //     println!("Error setting game state: {e}");
    //     // };
    
    //     Ok(true)
    // }

    fn handle_ret_msg(
        &mut self,
    ) -> io::Result<Option<Vec<u8>>> {
        let mut state_set_buf = Vec::new();
    
        let mut byte_buffer = [0];

        while let Ok((_, src)) = self.socket.recv_from(&mut byte_buffer) {
            let packet_type = UdpPacketTypes::from(byte_buffer[0]);

            match packet_type {
                UdpPacketTypes::GameState => {
                    self.socket.peek_from(&mut self.min_buf)?;

                    let num_bytes = GameState_sim::get_num_bytes(&self.min_buf);
                    state_set_buf.resize(num_bytes, 0);
                    self.socket.recv_from(&mut state_set_buf)?;
                }
                UdpPacketTypes::Connection => {
                    println!("Connection established to {src}");
                }
                UdpPacketTypes::Speed => {
                    let mut speed_buffer = [0; f32::NUM_BYTES];
                    self.socket.recv_from(&mut speed_buffer)?;
                    let speed = f32::from_bytes(&speed_buffer);
                    self.interval = Duration::from_secs_f32(1. / (120. * speed));
                }
                UdpPacketTypes::Paused => {
                    self.socket.recv_from(&mut byte_buffer)?;
                    self.pause = byte_buffer[0] == 1;
                }
                UdpPacketTypes::Quit | UdpPacketTypes::Render => {
                    panic!("We shouldn't be receiving packets of type {packet_type:?}")
                }
            }
        }
    
        // the socket didn't send a state back
        if state_set_buf.is_empty() {
            return Ok(None);
        }
    
        Ok(Some(state_set_buf))
    }

    // just to make sure the speed value doesn't get updated from the initial speed sent back
    fn handle_ret_msg_init(
        &mut self,
    ) -> io::Result<bool> {
        let mut state_set_buf = Vec::new();
    
        let mut byte_buffer = [0];

        while let Ok((_, src)) = self.socket.recv_from(&mut byte_buffer) {
            let packet_type = UdpPacketTypes::from(byte_buffer[0]);

            match packet_type {
                UdpPacketTypes::GameState => {
                    self.socket.peek_from(&mut self.min_buf)?;

                    let num_bytes = GameState_sim::get_num_bytes(&self.min_buf);
                    state_set_buf.resize(num_bytes, 0);
                    self.socket.recv_from(&mut state_set_buf)?;
                }
                UdpPacketTypes::Connection => {
                    println!("Connection established to {src}");
                }
                UdpPacketTypes::Speed => {
                    let mut speed_buffer = [0; f32::NUM_BYTES];
                    self.socket.recv_from(&mut speed_buffer)?;
                    // let speed = f32::from_bytes(&speed_buffer);
                    // self.interval = Duration::from_secs_f32(1. / (120. * speed));
                }
                UdpPacketTypes::Paused => {
                    self.socket.recv_from(&mut byte_buffer)?;
                    // self.paused = byte_buffer[0] == 1;
                }
                UdpPacketTypes::Quit | UdpPacketTypes::Render => {
                    panic!("We shouldn't be receiving packets of type {packet_type:?}")
                }
            }
        }
    
        // the socket didn't send a state back
        if state_set_buf.is_empty() {
            return Ok(false);
        }
    
        Ok(true)
    }

    // fn ctrl_channel() -> Result<Receiver<()>, ctrlc::Error> {
    //     let (sender, receiver) = channel();
    
    //     // Setup Ctrl+C handler
    //     ctrlc::set_handler(move || {
    //         // Send a signal to the main thread to break the loop
    //         // If we can't send the signal for some reason,
    //         // then panic the process to shut down
    //         sender.send(()).unwrap();
    //     })?;
    
    //     Ok(receiver)
    // }
}