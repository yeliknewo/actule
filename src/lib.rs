pub extern crate id_alloc;
pub extern crate piston_window;
pub extern crate ncollide;
pub extern crate nalgebra;

pub mod actule {
    mod game;
    mod world;
    mod entity;
    mod components;
    mod utils;
    mod keyboard;

    pub use self::game::*;
    pub use self::world::*;
    pub use self::entity::*;
    pub use self::keyboard::*;
    pub use self::components::*;
    pub use self::utils::*;
}
