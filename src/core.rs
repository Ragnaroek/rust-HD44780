use std;
use std::time::Duration;
use std::io::Error;
use super::hosts::{HD44780Host, Mode};

pub enum DisplayRow {
    R0,
    R1
}

pub struct HD44780 {
    host: Box<HD44780Host>
}

impl HD44780 {
    pub fn new(h: Box<HD44780Host>) -> HD44780 {
        return HD44780 {host: h};
    }

    pub fn init(&mut self) -> Result<(), Error> {
        try!(self.host.init());

        self.write_cmd(0x33);
        self.write_cmd(0x32);
        self.write_cmd(0x28);
        self.write_cmd(0x0C);
        self.write_cmd(0x06);
        self.write_cmd(0x01);
        Ok(())
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

    fn write_cmd(&mut self, b: u8) {
        self.write_byte_in_mode(b, Mode::Command);
    }

    fn write_data(&mut self, b: u8) {
        self.write_byte_in_mode(b, Mode::Data)
    }

    fn write_byte_in_mode(&mut self, b: u8, mode: Mode) {
        self.host.rs(mode);
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

    pub fn row_select(&mut self, row: DisplayRow) {
        match row {
            DisplayRow::R0 => self.write_cmd(0x80),
            DisplayRow::R1 => self.write_cmd(0xC0)
        }
    }

    pub fn write_string(&mut self, txt: &str) {
        let bytes = txt.as_bytes();
        let len = if txt.len() > 16 {
            16
        } else {
            txt.len()
        };
        for i in 0..len {
            self.write_data(bytes[i]);
        }
    }
}
