use embedded_graphics::{
    draw_target::DrawTarget, geometry::Point, pixelcolor::PixelColor, primitives::Rectangle, Pixel,
};

use crate::layoutable::Layoutable;
use crate::{ComponentSize, ValueRange};

///
/// Draws a simple scale consisting of 2x2 Dots onto a given space.
///
/// # Arguments
///
/// * `value`: value to be represented on the scale (0.0-1.0)
/// * `color`: color of the dots
///
/// returns: impl Layoutable<C>+Sized
///
pub fn scale<C: PixelColor>(value: f32, color: C) -> impl Layoutable<C> {
    Scale { value, color }
}

struct Scale<C: PixelColor> {
    value: f32,
    color: C,
}

impl<C: PixelColor> Layoutable<C> for Scale<C> {
    fn size(&self) -> ComponentSize {
        ComponentSize {
            width: ValueRange::fixed(11).expand_max(),
            height: ValueRange::fixed(4),
        }
    }

    fn draw_placed<DrawError>(
        &self,
        target: &mut impl DrawTarget<Color = C, Error = DrawError>,
        position: Rectangle,
    ) -> Result<(), DrawError> {
        let width = position.size.width;
        let x = position.top_left.x;
        let y = position.top_left.y;
        let total_dot_count = (width - 5) / 3;
        let x_offset = (width - total_dot_count * 3 - 4) / 2;
        let enabled_dot_count =
            ((self.value * total_dot_count as f32).round() as u32).min(total_dot_count);

        let pixels = [1, total_dot_count * 3 + 3]
            .into_iter()
            .chain((0..enabled_dot_count).flat_map(|d| [d * 3 + 3, d * 3 + 4]))
            .map(|p| (p + x_offset) as i32)
            .flat_map(|p| [Point { x: x + p, y: y + 1 }, Point { x: x + p, y: y + 2 }])
            .map(|p| Pixel(p, self.color));
        target.draw_iter(pixels)
    }
}
