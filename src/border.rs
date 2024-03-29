use std::marker::PhantomData;
use std::num::Saturating;

use embedded_graphics::{
    draw_target::DrawTarget, geometry::Point, pixelcolor::PixelColor, prelude::Size,
    primitives::Rectangle, Pixel,
};

use crate::{layoutable::Layoutable, ComponentSize};

pub trait Decorator<C: PixelColor> {
    fn width(&self) -> u32;
    fn draw_placed<DrawError>(
        &self,
        target: &mut impl DrawTarget<Color = C, Error = DrawError>,
        position: Rectangle,
    ) -> Result<(), DrawError>;
}

struct Bordered<L: Layoutable<C>, C: PixelColor, D: Decorator<C>> {
    layoutable: L,
    decorator: D,
    p: PhantomData<C>,
}

///
/// Generate a border around a layouted element
///
/// # Arguments
///
/// * `layoutable`: element within the border
/// * `decorator`: definition of the decorator around that element
///
/// returns: impl Layoutable<C>+Sized
///
/// # Examples
///
/// ```
/// use embedded_graphics::mono_font::iso_8859_1::FONT_6X12;
/// use embedded_graphics::mono_font::MonoTextStyle;
/// use embedded_graphics::pixelcolor::BinaryColor;
/// use simple_layout::prelude::{bordered, center, DashedLine, owned_text};
/// let temperature = 20.7;
/// let element = bordered(
///                 center(owned_text(format!("{temperature:.1}°C"), MonoTextStyle::new(&FONT_6X12, BinaryColor::On))),
///                 DashedLine::new(2, 2, BinaryColor::On),
///             );
/// ```
pub fn bordered<L: Layoutable<C>, C: PixelColor, D: Decorator<C>>(
    layoutable: L,
    decorator: D,
) -> impl Layoutable<C> {
    Bordered {
        layoutable,
        decorator,
        p: PhantomData,
    }
}

impl<L: Layoutable<C>, C: PixelColor, D: Decorator<C>> Layoutable<C> for Bordered<L, C, D> {
    fn size(&self) -> ComponentSize {
        let ComponentSize { width, height } = self.layoutable.size();
        let offset = Saturating(self.decorator.width() * 2);
        ComponentSize {
            width: width + offset,
            height: height + offset,
        }
    }

    fn draw_placed<DrawError>(
        &self,
        target: &mut impl DrawTarget<Color = C, Error = DrawError>,
        position: Rectangle,
    ) -> Result<(), DrawError> {
        let border = self.decorator.width();
        let Rectangle {
            top_left: Point { x, y },
            size: Size { width, height },
        } = position;
        let inner_position = Rectangle {
            top_left: Point {
                x: x + border as i32,
                y: y + border as i32,
            },
            size: Size {
                width: width - 2 * border,
                height: height - 2 * border,
            },
        };
        self.decorator.draw_placed(target, position)?;
        self.layoutable.draw_placed(target, inner_position)
    }
}
pub struct DashedLine<C: PixelColor> {
    dot_count: u32,
    gap_count: u32,
    color: C,
}

impl<C: PixelColor> DashedLine<C> {
    /// Create a new dashed line
    ///
    /// # Arguments
    ///
    /// * `dot_count`: count of dots on the pattern to draw
    /// * `gap_count`: count of pixels to miss between the strokes
    /// * `color`: color to paint the dots
    ///
    /// returns: DashedLine<C>
    ///
    /// # Examples
    ///
    /// Draw 2 dots, then skip 2 dots
    /// ```
    /// use embedded_graphics::pixelcolor::BinaryColor;
    /// use simple_layout::prelude::DashedLine;
    /// DashedLine::new(2, 2, BinaryColor::On);
    /// ```
    pub fn new(dot_count: u32, gap_count: u32, color: C) -> Self {
        Self {
            dot_count,
            gap_count,
            color,
        }
    }
}

impl<C: PixelColor> Decorator<C> for DashedLine<C> {
    fn width(&self) -> u32 {
        1
    }

    fn draw_placed<DrawError>(
        &self,
        target: &mut impl DrawTarget<Color = C, Error = DrawError>,
        position: Rectangle,
    ) -> Result<(), DrawError> {
        let sequence_length = self.dot_count + self.gap_count;
        let Point { x: sx, y: sy } = position.top_left;
        let Size { width, height } = position.size;
        let ex = sx + width as i32 - 1;
        let ey = sy + height as i32 - 1;
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
        )
    }
}

pub struct RoundedLine<C: PixelColor> {
    color: C,
}

impl<C: PixelColor> RoundedLine<C> {
    ///
    /// Create a pattern of a rounded line around a element
    ///
    /// # Arguments
    ///
    /// * `color`: Color of the line
    ///
    /// returns: RoundedLine<C>
    ///
    /// # Examples
    ///
    /// defines a plus button by rendering a '+' and draw a rounded line around
    /// ```
    /// use embedded_graphics::mono_font::iso_8859_1::FONT_6X12;
    /// use embedded_graphics::mono_font::MonoTextStyle;
    /// use embedded_graphics::pixelcolor::BinaryColor;
    /// use embedded_graphics::prelude::Point;
    /// use embedded_graphics::text::Text;
    /// use simple_layout::prelude::{bordered, padding, RoundedLine};
    /// let plus_button = bordered(
    ///                     padding(Text::new("+", Point::zero(), MonoTextStyle::new(&FONT_6X12, BinaryColor::On)), -2, 1, -1, 1),
    ///                     RoundedLine::new(BinaryColor::On),
    ///                 );
    /// ```
    pub fn new(color: C) -> Self {
        Self { color }
    }
}

impl<C: PixelColor> Decorator<C> for RoundedLine<C> {
    fn width(&self) -> u32 {
        2
    }

    fn draw_placed<DrawError>(
        &self,
        target: &mut impl DrawTarget<Color = C, Error = DrawError>,
        position: Rectangle,
    ) -> Result<(), DrawError> {
        let Point { x: sx, y: sy } = position.top_left;
        let Size { width, height } = position.size;
        let ex = sx + width as i32 - 1;
        let ey = sy + height as i32 - 1;
        target.draw_iter(
            (sx + 2..ex - 1)
                .map(|x| Pixel(Point { x, y: sy }, self.color))
                .chain(Some(Pixel(
                    Point {
                        x: ex - 1,
                        y: sy + 1,
                    },
                    self.color,
                )))
                .chain((sy + 2..ey - 1).map(|y| Pixel(Point { x: ex, y }, self.color)))
                .chain(Some(Pixel(
                    Point {
                        x: ex - 1,
                        y: ey - 1,
                    },
                    self.color,
                )))
                .chain(
                    (sx + 2..ex - 1)
                        .rev()
                        .map(|x| Pixel(Point { x, y: ey }, self.color)),
                )
                .chain(Some(Pixel(
                    Point {
                        x: sx + 1,
                        y: ey - 1,
                    },
                    self.color,
                )))
                .chain(
                    (sy + 2..ey - 1)
                        .rev()
                        .map(|y| Pixel(Point { x: sx, y }, self.color)),
                )
                .chain(Some(Pixel(
                    Point {
                        x: sx + 1,
                        y: sy + 1,
                    },
                    self.color,
                ))),
        )
    }
}
