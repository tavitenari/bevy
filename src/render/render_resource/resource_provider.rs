use crate::render::renderer::Renderer;
use legion::prelude::*;

pub trait ResourceProvider {
    fn initialize(
        &mut self,
        _renderer: &mut dyn Renderer,
        _world: &mut World,
        _resources: &Resources,
    ) {
    }
    fn update(&mut self, _renderer: &mut dyn Renderer, _world: &mut World, _resources: &Resources) {
    }
    fn resize(
        &mut self,
        _renderer: &mut dyn Renderer,
        _world: &mut World,
        _resources: &Resources,
        _width: u32,
        _height: u32,
    ) {
    }
}