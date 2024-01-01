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
        ToBytes, FromBytes
    },
    // cxx::UniquePtr,
    // math::Vec3,
    // sim::{Arena, ArenaMemWeightMode, BallState, CarConfig, CarControls, GameMode, Team},
    GameState as GameState_sim,
};

use crate::make::RenderConfig;

#[repr(u8)]
enum UdpPacketTypes {
    Quit,
    GameState,
}

pub struct Renderer {
    socket: UdpSocket,
    interval: Duration,
    // tick_skip_interval: Duration,
    min_buf: [u8; GameState_sim::MIN_NUM_BYTES],
    next_time: Instant,
    // next_time_tick_skip: Instant,
    // ctrlc_recv: Receiver<()>,
    sock_addr: SocketAddr,
}

const RLVISER_PATH: &str = if cfg!(windows) { "./rlviser.exe" } else { "./rlviser" };

impl Renderer {
    pub fn new(
        render_config: RenderConfig, 
        // tick_skip: usize
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

        // println!("Listening on {}", socket.local_addr()?);
    
        // Load rocketsim
        // rocketsim_rs::init(None);
    
        // let mut args = std::env::args();
        // let _ = args.next();
        // let arena_type = match args.next().as_deref() {
        //     Some("hoops") => GameMode::HOOPS,
        //     _ => GameMode::SOCCAR,
        // };
        // let arena_type = GameMode::SOCCAR;
    
        // run_socket(socket, arena_type)

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

        if buf[0] == 1 {
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

        // set the update rate for the rendering
        let interval = Duration::from_secs_f32(1. / render_config.update_rate);
        // let tick_skip_interval = Duration::from_secs_f32(1. / (render_config.update_rate / tick_skip as f32));
        let next_time = Instant::now() + interval;
        // let next_time_tick_skip = Instant::now() + tick_skip_interval;
        let min_state_set_buf = [0; GameState_sim::MIN_NUM_BYTES];

        Ok(
            Self {
                socket,
                interval,
                // tick_skip_interval,
                min_buf: min_state_set_buf,
                next_time,
                // next_time_tick_skip,
                // ctrlc_recv: receiver,
                sock_addr: src,
            }
        )
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

        // let states_len = states.len();
        let mut state_set_bool = false;
        for state in states.into_iter() {
            // this is more just to handle if anything gets sent back
            let res = Renderer::handle_state_set(&mut self.min_buf, &self.socket);
            match res {
                Ok(val) => {
                    // if we received a state
                    if val {
                        state_set_bool = true;
                        break
                    }
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
    
            // ensure correct time to wait with interval
            // if i == states_len-1 {
            //     let wait_time = self.next_time - Instant::now();
            //     if wait_time > Duration::default() {
            //         sleep(wait_time);
            //     }
            //     self.next_time += self.interval + self.tick_skip_interval;
            // } else {
                let wait_time = self.next_time - Instant::now();
                if wait_time > Duration::default() {
                    sleep(wait_time);
                }
                self.next_time += self.interval;
            // }
        }

        if state_set_bool {
            return Ok(Some(GameState_sim::from_bytes(&self.min_buf)))
        }

        // let wait_time = self.next_time_tick_skip - Instant::now();
        // if wait_time > Duration::default() {
        //     sleep(wait_time);
        // }
        // self.next_time_tick_skip += self.tick_skip_interval;

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

    fn handle_state_set(
        min_state_set_buf: &mut [u8; GameState_sim::MIN_NUM_BYTES],
        socket: &UdpSocket,
        // arena: &mut UniquePtr<Arena>,
    ) -> io::Result<bool> {
        let mut state_set_buf = Vec::new();
    
        while socket.peek_from(min_state_set_buf).is_ok() {
            // the socket sent data back
            // this is the other side telling us to update the game state
            let num_bytes = GameState_sim::get_num_bytes(min_state_set_buf);
            state_set_buf = vec![0; num_bytes];
            socket.recv_from(&mut state_set_buf)?;
        }
    
        // the socket didn't send data back
        if state_set_buf.is_empty() {
            return Ok(false);
        }
    
        // set the game state
        // let game_state = GameState_sim::from_bytes(&state_set_buf);
        // if let Err(e) = arena.pin_mut().set_game_state(&game_state) {
        //     println!("Error setting game state: {e}");
        // };
    
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