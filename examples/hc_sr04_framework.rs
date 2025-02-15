//! Example using the HC-SR04 sensor. It asks every second
//! for the distance of the object in front.

use esp32framework::sensors::HCSR04;
use esp32framework::Microcontroller;

fn main() {
    let mut micro = Microcontroller::take();
    let echo = micro.set_pin_as_digital_in(6).unwrap();
    let trig = micro.set_pin_as_digital_out(5).unwrap();
    let mut sensor = HCSR04::new(trig, echo);

    loop {
        let distance = sensor.get_distance();
        println!("{:?} cm", distance);
        micro.wait_for_updates(Some(1000));
    }
}
