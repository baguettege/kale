mod core;
mod io;
mod math;

pub use core::CORE;
pub use io::IO;
pub use math::MATH;
use kale_api::runtime::builtin::Lib;

pub const STDLIB: &[&Lib] = &[
    CORE,
    IO,
    MATH,
];
