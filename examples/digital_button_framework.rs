//! Example using pin GPIO9 as digital in to count the amount of times a button
//! is pressed. The signal is configured with a debounce time of 200msec.

use esp32framework::{gpio::digital::InterruptType, Microcontroller};

fn main() {
    let mut micro = Microcontroller::take();
    let mut button = micro.set_pin_as_digital_in(9).unwrap();
    button.set_debounce(200 * 1000);

    let mut count: u32 = 0;
    let callback = move |_| {
        count += 1;
        println!("Press Count {}", count);
    };
    button
        .trigger_on_interrupt(callback, InterruptType::NegEdge)
        .unwrap();
    micro.wait_for_updates(None);
}
