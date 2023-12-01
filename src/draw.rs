use embedded_graphics::{
    prelude::{Dimensions, DrawTarget, PixelColor, Point},
    primitives::Rectangle,
    Pixel,
};

pub(crate) struct OffsetDrawable<'a, Color, Error, Target>
where
    Target: DrawTarget<Color = Color, Error = Error>,
    Color: PixelColor,
{
    target: &'a mut Target,
    offset: Point,
}

impl<'a, Color, Error, Target> OffsetDrawable<'a, Color, Error, Target>
where
    Target: DrawTarget<Color = Color, Error = Error>,
    Color: PixelColor,
{
    pub fn new(target: &'a mut Target, offset: Point) -> Self {
        Self { target, offset }
    }
}

impl<'a, Color, Error, Target> Dimensions for OffsetDrawable<'a, Color, Error, Target>
where
    Target: DrawTarget<Color = Color, Error = Error>,
    Color: PixelColor,
{
    fn bounding_box(&self) -> Rectangle {
        let bbox = self.target.bounding_box();
        Rectangle {
            top_left: bbox.top_left - self.offset,
            size: bbox.size,
        }
    }
}

impl<'a, Color, Error, Target> DrawTarget for OffsetDrawable<'a, Color, Error, Target>
where
    Target: DrawTarget<Color = Color, Error = Error>,
    Color: PixelColor,
{
    type Color = Color;
    type Error = Error;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        let offset = self.offset;
        self.target.draw_iter(
            pixels
                .into_iter()
                .map(|Pixel::<Self::Color>(p, c)| Pixel(p + offset, c)),
        )
    }
}
