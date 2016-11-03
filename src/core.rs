use std;
use std::time::Duration;
use super::hosts::{HD44780Host, Mode};

pub struct HD44780 {
    host: Box<HD44780Host>
}

impl HD44780 {
    pub fn new(h: Box<HD44780Host>) -> HD44780 {
        return HD44780 {host: h};
    }

    pub fn init(&mut self) {
        self.host.init();
    }

    fn delay(&self) {
        std::thread::sleep(Duration::from_millis(3));
    }

    fn reset_data(&mut self) {
        self.host.data4(false);
        self.host.data5(false);
        self.host.data6(false);
        self.host.data7(false);
    }

    fn toggle_enable(&mut self) {
        self.delay();
        self.host.enable(false);
        self.delay();
        self.host.enable(true);
        self.delay();
    }

    fn write_byte(&mut self, b: u8) {
        self.host.rs(Mode::Data);
        self.reset_data();

        if b&0x10==0x10 {
            self.host.data4(true);
        }
        if b&0x20==0x20 {
            self.host.data5(true);
        }
        if b&0x40==0x40 {
            self.host.data6(true);
        }
        if b&0x80==0x80 {
            self.host.data7(true);
        }

        self.toggle_enable();
        self.reset_data();

        if b&0x01==0x01 {
            self.host.data4(true);
        }
        if b&0x02==0x02 {
            self.host.data5(true);
        }
        if b&0x04==0x04 {
            self.host.data6(true);
        }
        if b&0x08==0x08 {
            self.host.data7(true);
        }

        self.toggle_enable();
    }

    pub fn write_string(&mut self, txt: &str) {
        let bytes = txt.as_bytes();
        let len = if txt.len() > 16 {
            16
        } else {
            txt.len()
        };
        for i in 0..len {
            self.write_byte(bytes[i]);
        }
    }
}
