use std::hash::Hash;

use id_alloc::*;

use actule::*;

pub trait Entity<I: Num + Bounded + Ord + CheckedAdd + CheckedSub + One + Copy + Hash, T: Entity<I, T>> {
    fn get_id(&self) -> I;
    fn get_renderable(&self) -> Option<&Box<Renderable>>;
    fn get_transform(&self) -> Option<&Box<Transform>>;
    fn get_mut_renderable(&mut self) -> Option<&mut Box<Renderable>>;
    fn get_mut_transform(&mut self) -> Option<&mut Box<Transform>>;
    fn tick(&mut self, dt: f64, manager: &mut Node<I>, world: &mut World<I, T>, keyboard: &mut Keyboard);
    fn is_tick(&self) -> bool;
}

#[macro_export]
macro_rules! impl_component_for_entity {
    ($t: ty, $p: ident, $c: ty, $so: ident, $s: ident, $w: ident, $g: ident, $m: ident, $ta: ident, $gi: ident) => (
        impl $t {
            #[inline]
            pub fn $so (&mut self, $p: Option<Box<$c>>) {
                self.$p = $p;
            }

            #[inline]
            pub fn $s (&mut self, $p: $c) {
                self.$so(Some(Box::new($p)));
            }

            #[inline]
            pub fn $w (mut self, $p: $c) -> $t {
                self.$s($p);
                self
            }

            #[inline]
            pub fn $g (&self) -> Option<&Box<$c>> {
                self.$p.as_ref()
            }

            #[inline]
            pub fn $m (&mut self) -> Option<&mut Box<$c>> {
                self.$p.as_mut()
            }

            #[inline]
            pub fn $ta (&mut self) -> Option<Box<$c>> {
                self.$p.take()
            }

            #[inline]
            pub fn $gi (&mut self, $p: Box<$c>) {
                self.$p = Some($p);
            }
        }
    )
}
