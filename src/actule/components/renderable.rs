use super::renderables::*;
use actule::utils::Layer;

pub struct Renderable {
    layer: Layer,
    polygon: Option<Shape>,
    texture: Option<MyTexture>
}
