extern crate hd44780;

use std::boxed::Box;
use hd44780::core::HD44780;
use hd44780::hosts::RaspberryPiBPlus;

fn main() {
    let host = RaspberryPiBPlus::new();
    let display = HD44780::new(Box::new(host));

    display.write_string("hello!");
}
