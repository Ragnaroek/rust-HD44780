
use super::hosts::HD44780Host;

pub struct HD44780 {
    host: Box<HD44780Host>
}

impl HD44780 {
    pub fn new(h: Box<HD44780Host>) -> HD44780 {
        return HD44780 {host: h};
    }

    pub fn write_string(&self, txt: &str) {
        for chr in txt.as_bytes() {
            println!("char={:?}", chr);
        }
    }
}
