pub extern crate id_alloc;
pub extern crate piston_window;
pub extern crate ncollide;
pub extern crate nalgebra;
pub extern crate gfx_device_gl;

pub mod actule {
    mod game;
    mod world;
    mod entity;
    mod components;
    mod utils;
    mod input;
    mod layer;

    pub use self::game::*;
    pub use self::world::*;
    pub use self::entity::*;
    pub use self::input::*;
    pub use self::components::*; //gfx_device_gl Factory type lives here
    pub use self::utils::*;
    pub use self::layer::*;

}
