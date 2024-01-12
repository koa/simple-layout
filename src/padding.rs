use std::marker::PhantomData;
use std::num::Saturating;

use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::geometry::Point;
use embedded_graphics::prelude::{PixelColor, Size};
use embedded_graphics::primitives::Rectangle;

use crate::prelude::Layoutable;
use crate::ComponentSize;

///
/// Adjust placement of a component by adding/removing additional offsets
///
/// # Arguments
///
/// * `layoutable`: element to place
/// * `top`: adjustment to the top
/// * `right`: adjustment on the right
/// * `bottom`: adjustment to the bottom
/// * `left`: adjustment on the left
///
/// returns: impl Layoutable<C>+Sized
///
/// # Examples
///
/// ```
/// use embedded_graphics::geometry::Point;
/// use embedded_graphics::mono_font::iso_8859_1::FONT_6X12;
/// use embedded_graphics::mono_font::MonoTextStyle;
/// use embedded_graphics::pixelcolor::BinaryColor;
/// use embedded_graphics::text::Text;
/// use simple_layout::prelude::padding;
/// let adjusted_text = padding(Text::new("-", Point::zero(), MonoTextStyle::new(&FONT_6X12, BinaryColor::On)), -2, 1, -1, 1);
/// ```
pub fn padding<C: PixelColor, L: Layoutable<C>>(
    layoutable: L,
    top: i32,
    right: i32,
    bottom: i32,
    left: i32,
) -> impl Layoutable<C> {
    Padding {
        layoutable,
        top,
        right,
        bottom,
        left,
        p: Default::default(),
    }
}

struct Padding<C: PixelColor, L: Layoutable<C>> {
    layoutable: L,
    top: i32,
    right: i32,
    bottom: i32,
    left: i32,
    p: PhantomData<C>,
}

impl<C: PixelColor, L: Layoutable<C>> Layoutable<C> for Padding<C, L> {
    fn size(&self) -> ComponentSize {
        let ComponentSize { width, height } = self.layoutable.size();
        ComponentSize {
            width: width + (self.left + self.right),
            height: height + (self.top + self.bottom),
        }
    }

    fn draw_placed<DrawError>(
        &self,
        target: &mut impl DrawTarget<Color = C, Error = DrawError>,
        position: Rectangle,
    ) -> Result<(), DrawError> {
        let Rectangle {
            top_left: Point { x, y },
            size: Size { width, height },
        } = position;
        let position = Rectangle {
            top_left: Point {
                x: x + self.left,
                y: y + self.top,
            },
            size: Size {
                width: (Saturating(width as i32) - Saturating(self.left + self.right)).0 as u32,
                height: (Saturating(height as i32) - Saturating(self.top + self.bottom)).0 as u32,
            },
        };
        self.layoutable.draw_placed(target, position)
    }
}
