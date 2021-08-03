use sysfs_gpio::Pin;
use std::{thread::sleep, time:: Duration};

pub struct Axis{
  name: String,
  dir_pin: Pin,
  step_pin: Pin,
  step_delay: u16,
  pulse_delay: u8,
  axis_position: i64,
}

impl Axis{
  pub fn build_axis(step_pin:u64, dir_pin:u64, name: String ) -> Axis{
    Axis{
      name: name,
      dir_pin: Pin::new(dir_pin),
      step_pin: Pin::new(step_pin),
      step_delay: 650,
      pulse_delay: 1,
      axis_position: 0
    }
  }

  fn step(&self){
    self.step_pin.set_value(1).unwrap();
    sleep(Duration::from_micros(self.pulse_delay as u64));
    self.step_pin.set_value(0).unwrap();
  }

}