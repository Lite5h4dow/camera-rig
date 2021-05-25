extern crate sysfs_gpio;

use sysfs_gpio::{Direction, Pin};
use std::thread::{sleep, spawn};
use std::time::Duration;

use pasts::prelude::*;

use stick::{Controller, Event};

fn micros_pulse(pin: Pin, duration: u64){
    pin.set_value(1).unwrap();
    sleep(Duration::from_micros(duration));
    pin.set_value(0).unwrap();
}

async fn controller_thread(){
    let mut listener = Controller::listener();
    let mut ctlrs = Vec::<Controller>::new();

    println!("Hello from the controller thread");

    'e: loop{
        match poll![listener, poll!(ctlrs)].await.1{
            (_, Event::Connect(new)) =>{
                println!(
                    "Connected p{}, id: {:04X}_{:04X}_{:04X}_{:04X}, name: {}",
                    ctlrs.len()+1,
                    new.id()[0],
                    new.id()[1],
                    new.id()[2],
                    new.id()[3],
                    new.name(),
                );
                ctlrs.push(*new);
            }

            (id, Event::Disconnect) => {
                println!("Disconnected p{}",id + 1);
                ctlrs.swap_remove(id);
            }

            (id, Event::Home(true)) => {
                println!("p{} ended the session", id+1);
                break 'e;
            }

            (id, event)=>{
                println!("p{}: {}", id+1, event);
                match event{
                    Event::ActionA(pressed) => {
                        ctlrs[id].rumble(if pressed {1.0} else {0.0});
                    }
                    Event::ActionB(pressed) => {
                        ctlrs[id].rumble(if pressed {0.3} else {0.0});
                    }
                    _ => {}
                }
            }
        }
    }
}

fn main(){
    spawn(|| {exec!(controller_thread())});
    let step = Pin::new(23);
    step.with_exported(||{
        step.set_direction(Direction::Out).unwrap();
        loop{
            micros_pulse(step, 1);
            sleep(Duration::from_micros(650));
        }
    }).unwrap()

}