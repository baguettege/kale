pub mod io;

use kale_api::interpreter::{Init, Setup};

pub const INIT: Init = init;

fn init(setup: &mut Setup) {
    io::INIT(setup);
}
