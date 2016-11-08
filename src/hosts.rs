
use std::fs::OpenOptions;
use std::fs::File;
use std::io::{Write, Error};

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
}

fn io(file_opt: &mut Option<File>, b: bool) {
    let out = if b { b"1"} else {b"0"};
    match *file_opt {
        Some(ref mut file) => {
            file.write_all(out).unwrap();
            file.flush().unwrap();
            println!("wrote {:?} to {:?}", out, file)},
        None => println!("some io not writable")
    }
    return;
}

impl HD44780Host for RaspberryPiBPlus {

    fn init(&mut self) -> Result<(), Error> {
        let mut o = OpenOptions::new();
        let fso = o.write(true);
        self.rs = Some(try!(fso.open("/sys/class/gpio/gpio7/value")));
        self.enable = Some(try!(fso.open("/sys/class/gpio/gpio8/value")));
        self.data4 = Some(try!(fso.open("/sys/class/gpio/gpio25/value")));
        self.data5 = Some(try!(fso.open("/sys/class/gpio/gpio24/value")));
        self.data6 = Some(try!(fso.open("/sys/class/gpio/gpio23/value")));
        self.data7 = Some(try!(fso.open("/sys/class/gpio/gpio18/value")));
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
