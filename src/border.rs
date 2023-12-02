use embedded_graphics::{
    draw_target::DrawTarget, geometry::Point, pixelcolor::PixelColor, prelude::Size,
    primitives::Rectangle, Pixel,
};

use crate::{layoutable::Layoutable, ComponentSize};

pub struct Bordered<L: Layoutable<C>, C: PixelColor> {
    layoutable: L,
    outer_margin: u32,
    inner_padding: u32,
    color: C,
    dot_count: u32,
    gap_count: u32,
}

impl<L: Layoutable<C>, C: PixelColor> Bordered<L, C> {
    pub fn new(
        layoutable: L,
        outer_margin: u32,
        inner_padding: u32,
        color: C,
        dot_count: u32,
        gap_count: u32,
    ) -> Self {
        Self {
            layoutable,
            outer_margin,
            inner_padding,
            color,
            dot_count,
            gap_count,
        }
    }
}

impl<L: Layoutable<C>, C: PixelColor> Layoutable<C> for Bordered<L, C> {
    fn size(&self) -> ComponentSize {
        let inner_size = self.layoutable.size();
        let offset = (self.outer_margin + self.inner_padding + 1) * 2;
        ComponentSize {
            width: inner_size.width + offset,
            height: inner_size.height + offset,
        }
    }

    fn draw_placed<DrawError>(
        &self,
        target: &mut impl DrawTarget<Color = C, Error = DrawError>,
        position: Rectangle,
    ) -> Result<Point, DrawError> {
        let offset = self.outer_margin + self.inner_padding + 1;
        let Rectangle {
            top_left: Point { x, y },
            size: Size { width, height },
        } = position;
        let inner_position = Rectangle {
            top_left: Point {
                x: x + offset as i32,
                y: y + offset as i32,
            },
            size: Size {
                width: width - 2 * offset,
                height: height - 2 * offset,
            },
        };
        let sx = x + self.outer_margin as i32;
        let ex = x + width as i32 - self.outer_margin as i32 - 1;
        let sy = y + self.outer_margin as i32;
        let ey = y + height as i32 - self.outer_margin as i32 - 1;
        let sequence_length = self.dot_count + self.gap_count;

        target.draw_iter(
            (sx..ex)
                .map(|x| Pixel(Point { x, y: sy }, self.color))
                .chain((sy..ey).map(|y| Pixel(Point { x: ex, y }, self.color)))
                .chain(
                    (sx..ex)
                        .rev()
                        .map(|x| Pixel(Point { x, y: ey }, self.color)),
                )
                .chain(
                    (sy..ey)
                        .rev()
                        .map(|y| Pixel(Point { x: sx, y }, self.color)),
                )
                .enumerate()
                .filter(|(i, _)| *i as u32 % sequence_length < self.dot_count)
                .map(|(_, p)| p),
        )?;
        self.layoutable.draw_placed(target, inner_position)
    }
}
