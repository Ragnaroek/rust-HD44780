
pub enum Mode {
    Command,
    Data
}

/// A host device the HD44780 is connected to.
pub trait HD44780Host {
    fn init(&self) -> ();
    fn rs(&self, mode: Mode) -> ();
    fn enable(&self, b: bool) -> ();
    fn data4(&self, b: bool) -> ();
    fn data5(&self, b: bool) -> ();
    fn data6(&self, b: bool) -> ();
    fn data7(&self, b: bool) -> ();
}


pub struct RaspberryPiBPlus {}

impl RaspberryPiBPlus {
    pub fn new() -> RaspberryPiBPlus {
        return RaspberryPiBPlus{};
    }
}

impl HD44780Host for RaspberryPiBPlus {
    fn init(&self) -> () {

    }
    fn rs(&self, mode: Mode) -> () {

    }
    fn enable(&self, b: bool) -> () {

    }
    fn data4(&self, b: bool) -> () {

    }
    fn data5(&self, b: bool) -> () {

    }
    fn data6(&self, b: bool) -> () {

    }
    fn data7(&self, b: bool) -> () {

    }
}
