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
    mod input;

    pub use self::game::*;
    pub use self::world::*;
    pub use self::entity::*;
    pub use self::input::*;
    pub use self::components::*;
    pub use self::utils::*;
}
