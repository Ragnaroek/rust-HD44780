extern crate hd44780;

use std::boxed::Box;
use hd44780::core::HD44780;
use hd44780::hosts::RaspberryPiBPlus;

fn main() {
    let host = RaspberryPiBPlus::new();
    let mut display = HD44780::new(Box::new(host));

    display.init().unwrap();
    display.write_string("override_me!");
    display.write_string("hello!");
}
