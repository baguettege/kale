pub mod globals;

use kale_api::interpreter::{Loader, Registry};

pub const LOADER: Loader = loader;

pub fn loader(registry: &mut Registry) {
    globals::LOADER(registry);
}
