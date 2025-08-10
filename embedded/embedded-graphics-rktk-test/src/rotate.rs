use embedded_graphics::{
    Pixel,
    prelude::{Dimensions, DrawTarget, Point, Size},
    primitives::Rectangle,
};

#[derive(Debug)]
pub struct RotatedDrawTarget<'a, T>
where
    T: DrawTarget,
{
    parent: &'a mut T,
}

impl<'a, T> RotatedDrawTarget<'a, T>
where
    T: DrawTarget,
{
    pub fn new(parent: &'a mut T) -> Self {
        Self { parent }
    }
}

impl<T> DrawTarget for RotatedDrawTarget<'_, T>
where
    T: DrawTarget,
{
    type Color = T::Color;
    type Error = T::Error;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        let parent_height = self.parent.bounding_box().size.height as i32;

        self.parent.draw_iter(
            pixels
                .into_iter()
                .map(|Pixel(p, c)| Pixel(Point::new(p.y, parent_height - p.x), c)),
        )
    }
}

impl<T> Dimensions for RotatedDrawTarget<'_, T>
where
    T: DrawTarget,
{
    fn bounding_box(&self) -> Rectangle {
        let parent_bb = self.parent.bounding_box();
        Rectangle::new(
            parent_bb.top_left,
            Size::new(parent_bb.size.height, parent_bb.size.width),
        )
    }
}
