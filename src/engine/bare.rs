use super::*;

pub struct BareEngine<'a> {
    pub world: crate::world::World,
    w: usize,
    h: usize,
    set_pixel: &'a mut dyn FnMut(usize, usize, u8, u8, u8) -> (),
}

impl<'a> BareEngine<'a> {
    pub fn new(
        world: crate::world::World,
        w: usize,
        h: usize,
        set_pixel: &'a mut dyn FnMut(usize, usize, u8, u8, u8) -> ()
    ) -> Self {
        Self {
            world,
            w,
            h,
            set_pixel
        }
    }

    pub fn tick(&mut self) {
        self.world.tick();
    }
    pub fn render(&mut self) {
        self.clear();
        self.render_world(&self.world.clone());
    }
}

impl Engine for BareEngine<'_> {
    fn width(&self) -> usize {
        self.w
    }
    fn height(&self) -> usize {
        self.h
    }
    fn clear(&mut self) {
        for y in 0..self.h {
            for x in 0..self.w {
                (self.set_pixel)(x, y, 255, 0, 0);
            }
        }
    }
    fn set_at(&mut self, _idx: usize, _pixel: Pixel) {
        //  No-op funciton in this case
    }
    fn set(&mut self, x: usize, y: usize, pixel: Pixel) {
        (self.set_pixel)(x, y, pixel.0, pixel.1, pixel.2);
    }
    fn set_at_with_opacity(&mut self, _idx: usize, _pixel: Pixel, _opacity: f64) {
        // No-op here, we don't use opacity in this implementation
    }
}