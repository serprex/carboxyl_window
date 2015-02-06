#![feature(std_misc, core, io)]

#[macro_use(lift)]
extern crate carboxyl;
extern crate nalgebra;
extern crate clock_ticks;
extern crate glutin;
extern crate glium;

pub use traits::ApplicationLoop;

pub mod glium_loop;
pub mod button;
mod traits;