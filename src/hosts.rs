use std::fs::OpenOptions;
use std::fs::File;
use std::io::{Write, Error};
use std::path::Path;
use std::{thread, time};

pub enum Mode {
    Command,
    Data
}

/// A host device the HD44780 is connected to.
pub trait HD44780Host {
    fn init(&mut self) -> Result<(), Error>;
    fn rs(&mut self, mode: Mode) -> ();
    fn enable(&mut self, b: bool) -> ();
    fn data4(&mut self, b: bool) -> ();
    fn data5(&mut self, b: bool) -> ();
    fn data6(&mut self, b: bool) -> ();
    fn data7(&mut self, b: bool) -> ();
}

pub struct RaspberryPiBPlus {
    rs: Option<File>,
    enable: Option<File>,
    data4: Option<File>,
    data5: Option<File>,
    data6: Option<File>,
    data7: Option<File>
}

impl RaspberryPiBPlus {
    pub fn new() -> RaspberryPiBPlus {
        return RaspberryPiBPlus{rs: None, enable: None,
            data4: None, data5: None, data6: None, data7: None};
    }

    fn init_gpio(&self) -> Result<(), Error> {
        let mut o = OpenOptions::new();
        let fso = o.write(true);
        let mut export = try!(fso.open("/sys/class/gpio/export"));
        if self.rs.is_none() {
            try!(export.write_all(b"7"));
        }
        if self.enable.is_none() {
            try!(export.write_all(b"8"));
        }
        if self.data4.is_none() {
            try!(export.write_all(b"25"));
        }
        if self.data5.is_none() {
            try!(export.write_all(b"24"));
        }
        if self.data6.is_none() {
            try!(export.write_all(b"23"));
        }
        if self.data7.is_none() {
            try!(export.write_all(b"18"));
        }
        try!(export.flush());
        return Ok(());
    }

    fn try_open_io(&mut self) -> Result<(), Error> {
        let mut o = OpenOptions::new();
        let fso = o.write(true);
        let gpio7 = Path::new("/sys/class/gpio/gpio7/value");
        if gpio7.exists() {
            self.rs = Some(try!(fso.open(gpio7)));
        }
        let gpio8 = Path::new("/sys/class/gpio/gpio8/value");
        if gpio8.exists() {
            self.enable = Some(try!(fso.open(gpio8)));
        }
        let gpio25 = Path::new("/sys/class/gpio/gpio25/value");
        if gpio25.exists() {
            self.data4 = Some(try!(fso.open(gpio25)));
        }
        let gpio24 = Path::new("/sys/class/gpio/gpio24/value");
        if gpio24.exists() {
            self.data5 = Some(try!(fso.open(gpio24)));
        }
        let gpio23 = Path::new("/sys/class/gpio/gpio23/value");
        if gpio23.exists() {
            self.data6 = Some(try!(fso.open(gpio23)));
        }
        let gpio18 = Path::new("/sys/class/gpio/gpio18/value");
        if gpio18.exists() {
            self.data7 = Some(try!(fso.open(gpio18)));
        }
        return Ok(());
    }

    fn write_out(&self, mut file: File) -> Result<(), Error> {
        try!(file.write_all(b"out"));
        try!(file.flush());
        return Ok(());
    }

    fn init_out_dir(&mut self) -> Result<(), Error> {
        let mut o = OpenOptions::new();
        let fso = o.write(true);
        try!(self.write_out(try!(fso.open("/sys/class/gpio/gpio7/direction"))));
        try!(self.write_out(try!(fso.open("/sys/class/gpio/gpio8/direction"))));
        try!(self.write_out(try!(fso.open("/sys/class/gpio/gpio25/direction"))));
        try!(self.write_out(try!(fso.open("/sys/class/gpio/gpio24/direction"))));
        try!(self.write_out(try!(fso.open("/sys/class/gpio/gpio23/direction"))));
        try!(self.write_out(try!(fso.open("/sys/class/gpio/gpio18/direction"))));
        return Ok(());
    }
}

fn io(file_opt: &mut Option<File>, b: bool) {
    let out = if b { b"1"} else {b"0"};
    match *file_opt {
        Some(ref mut file) => {
            file.write_all(out).unwrap();
            file.flush().unwrap();
            ()
        },
        None => println!("some io not writable")
    }
    return;
}

impl HD44780Host for RaspberryPiBPlus {

    fn init(&mut self) -> Result<(), Error> {
        try!(self.try_open_io());
        try!(self.init_gpio());
        try!(self.try_open_io());
        try!(self.init_out_dir());

        return Ok(());
    }

    fn rs(&mut self, mode: Mode) -> () {
        match mode {
            Mode::Command => io(&mut self.rs, false),
            Mode::Data => io(&mut self.rs, true)
        }
    }
    fn enable(&mut self, b: bool) -> () {
        io(&mut self.enable, b);
    }
    fn data4(&mut self, b: bool) -> () {
        io(&mut self.data4, b);
    }
    fn data5(&mut self, b: bool) -> () {
        io(&mut self.data5, b);
    }
    fn data6(&mut self, b: bool) -> () {
        io(&mut self.data6, b);
    }
    fn data7(&mut self, b: bool) -> () {
        io(&mut self.data7, b);
    }
}
