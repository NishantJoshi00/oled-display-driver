use std::{sync::{self, atomic}, thread::{self, JoinHandle}, fmt};

use crate::loader::disp::{DisplayCon, get_display};

trait Scene {
    fn looper(&mut self, display: DisplayCon) -> Result<(), Box<dyn std::error::Error>>;
}


pub trait Executor {
    type Status;
    fn start_task(&mut self, scene: Box<dyn Scene + Send>) -> Result<(), Box<dyn std::error::Error>>;
    fn status(&self) -> Result<Self::Status, Box<dyn std::error::Error>>;
    fn stop_task(&mut self) -> Result<Box<dyn Scene + Send>, Box<dyn std::error::Error>>;
}

pub struct Engine {
    online: sync::Arc<atomic::AtomicBool>,
    thread_handle: Option<thread::JoinHandle<Box<dyn Scene + Send>>>
}

pub enum State {
    Online,
    Offline
}

#[derive(Debug)]
struct EngineError(String);

impl fmt::Display for EngineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EngineError: {}", self.0)
    }
}

impl std::error::Error for EngineError {}


impl Executor for Engine {
    type Status = State;

    fn start_task(&mut self, scene: Box<dyn Scene + Send>) -> Result<(), Box<dyn std::error::Error>> {
        match self.status()? {
            State::Online => Err(EngineError("A task is already in execution".to_string()))?,
            _ => {}
        }
        let istate = self.online.clone();
        let executor = scene;
        let handle = thread::spawn(move || {
                istate.store(true, atomic::Ordering::Relaxed);

                match get_display() {
                    Ok(display) => { 
                        while istate.load(atomic::Ordering::Relaxed) {
                            match executor.looper(display) {
                                Err(err) => {
                                    eprintln!("Error while in loop: {}", err);
                                    break;
                                },
                                _ => {}
                            }
                        }
                        istate.store(false, atomic::Ordering::Relaxed);

                        executor
                    },
                    Err(err) => {
                        eprintln!("Error in thread: {}", err);
                        executor
                    }
                }

            });
        self.thread_handle = Some(handle);
        Ok(())

    }

    fn status(&self) -> Result<Self::Status, Box<dyn std::error::Error>> {
        match self.online.load(atomic::Ordering::Relaxed) {
            true => Ok(State::Online),
            false => Ok(State::Offline)
        }
    }

    fn stop_task(&mut self) -> Result<Box<dyn Scene + Send>, Box<dyn std::error::Error>> {
        self.online.store(false, atomic::Ordering::Relaxed);
        let output = self.thread_handle.ok_or(EngineError("unable to find a thread_handle".to_string()))?.join().map_err(|_| EngineError("something went wrong while `.join()``".to_string()))?;
        self.thread_handle = None;
        Ok(output)
    }
}
