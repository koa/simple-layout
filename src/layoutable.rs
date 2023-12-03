use embedded_graphics::{
    geometry::Size,
    image::Image,
    prelude::{Dimensions, DrawTarget, ImageDrawable, PixelColor, Point},
    primitives::Rectangle,
    text::{renderer::TextRenderer, Text},
    Drawable,
};

use crate::{draw::OffsetDrawable, ComponentSize};

pub trait Layoutable<Color: PixelColor> {
    fn size(&self) -> ComponentSize;
    fn draw_placed<DrawError>(
        &self,
        target: &mut impl DrawTarget<Color = Color, Error = DrawError>,
        position: Rectangle,
    ) -> Result<(), DrawError>;
}
impl<'a, S: TextRenderer<Color = Color>, Color: PixelColor> Layoutable<Color> for Text<'a, S> {
    fn size(&self) -> ComponentSize {
        let mut total_height = 0;
        let mut max_line_length = 0;
        for line in self.text.split('\n') {
            let metrics = self.character_style.measure_string(
                line,
                Point::default(),
                self.text_style.baseline,
            );
            let bbox = metrics.bounding_box;
            if bbox.size.width > max_line_length {
                max_line_length = bbox.size.width;
            }
            total_height += bbox.size.height;
        }
        ComponentSize::fixed_size(max_line_length - 1, total_height)
    }

    fn draw_placed<DrawError>(
        &self,
        target: &mut impl DrawTarget<Color = Color, Error = DrawError>,
        position: Rectangle,
    ) -> Result<(), DrawError> {
        let offset = if let Some(first_line) = self.text.split('\n').next() {
            self.character_style
                .measure_string(first_line, Point::default(), self.text_style.baseline)
                .bounding_box
                .top_left
        } else {
            Point::zero()
        };
        let offset = position.top_left - self.position - offset;
        self.draw(&mut OffsetDrawable::new(target, offset))?;
        //Drawable::draw(self, &mut OffsetDrawable::new(target, offset))?;
        Ok(())
    }
}

impl<'a, C: PixelColor, T: ImageDrawable<Color = C>> Layoutable<C> for Image<'a, T> {
    fn size(&self) -> ComponentSize {
        let Rectangle {
            size: Size { width, height },
            ..
        } = self.bounding_box();
        ComponentSize::fixed_size(width, height)
    }

    fn draw_placed<DrawError>(
        &self,
        target: &mut impl DrawTarget<Color = C, Error = DrawError>,
        position: Rectangle,
    ) -> Result<(), DrawError> {
        let Rectangle { top_left, .. } = self.bounding_box();
        let offset = position.top_left - top_left;
        self.draw(&mut OffsetDrawable::new(target, offset))?;
        Ok(())
    }
}
