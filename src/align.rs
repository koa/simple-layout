use std::marker::PhantomData;
use std::num::Saturating;

use embedded_graphics::{
    draw_target::DrawTarget, geometry::Point, pixelcolor::PixelColor, prelude::Size,
    primitives::Rectangle,
};

use crate::layoutable::Layoutable;
use crate::{ComponentSize, ValueRange};

///
/// Arrange a layoutable into the center of its available space
///
/// # Arguments
///
/// * `l`: element to place
///
/// returns: impl Layoutable<C>+Sized
///
/// # Examples
///
/// ```
/// use embedded_graphics::mono_font::iso_8859_1::FONT_6X12;
/// use embedded_graphics::mono_font::MonoTextStyle;
/// use embedded_graphics::pixelcolor::BinaryColor;
/// use simple_layout::prelude::{center, owned_text};
/// let temperature=20.3;
/// let element = center(owned_text(format!("{temperature:.1}°C"), MonoTextStyle::new(&FONT_6X12, BinaryColor::On)));
/// ```
pub fn center<L: Layoutable<C>, C: PixelColor>(l: L) -> impl Layoutable<C> {
    AlignLayout::<_, _, CenteredAlignment, CenteredAlignment>::new(l)
}

///
/// Arrange a layoutable into the left middle of its available space
///
pub fn west<L: Layoutable<C>, C: PixelColor>(l: L) -> impl Layoutable<C> {
    AlignLayout::<_, _, StartAlignment, CenteredAlignment>::new(l)
}
///
/// Arrange a layoutable into the right middle of its available space
///
pub fn east<L: Layoutable<C>, C: PixelColor>(l: L) -> impl Layoutable<C> {
    AlignLayout::<_, _, EndAlignment, CenteredAlignment>::new(l)
}
///
/// Arrange a layoutable into the top center of its available space
///
pub fn north<L: Layoutable<C>, C: PixelColor>(l: L) -> impl Layoutable<C> {
    AlignLayout::<_, _, CenteredAlignment, StartAlignment>::new(l)
}
///
/// Arrange a layoutable into the bottom center of its available space
///
pub fn south<L: Layoutable<C>, C: PixelColor>(l: L) -> impl Layoutable<C> {
    AlignLayout::<_, _, CenteredAlignment, EndAlignment>::new(l)
}

trait Alignment {
    fn place(
        available_range: Saturating<u32>,
        target_range: ValueRange<Saturating<u32>>,
    ) -> (Saturating<i32>, Saturating<u32>);
}

struct AlignLayout<L: Layoutable<C>, C: PixelColor, HA: Alignment, VA: Alignment> {
    layoutable: L,
    p1: PhantomData<C>,
    p2: PhantomData<VA>,
    p3: PhantomData<HA>,
}

impl<L: Layoutable<C>, C: PixelColor, HA: Alignment, VA: Alignment> AlignLayout<L, C, HA, VA> {
    fn new(layoutable: L) -> Self {
        Self {
            layoutable,
            p1: PhantomData,
            p2: PhantomData,
            p3: PhantomData,
        }
    }
    fn place(component_size: ComponentSize, available_area: Rectangle) -> Rectangle {
        let Size {
            width: available_width,
            height: available_height,
        } = available_area.size;
        let ComponentSize { width, height } = component_size;
        let origin = available_area.top_left;
        let (Saturating(x), Saturating(width)) = HA::place(Saturating(available_width), width);
        let (Saturating(y), Saturating(height)) = VA::place(Saturating(available_height), height);
        Rectangle {
            top_left: origin + Point { x, y },
            size: Size { width, height },
        }
    }
}

impl<L: Layoutable<C>, C: PixelColor, HA: Alignment, VA: Alignment> Layoutable<C>
    for AlignLayout<L, C, HA, VA>
{
    #[inline]
    fn size(&self) -> ComponentSize {
        self.layoutable.size()
    }

    #[inline]
    fn draw_placed<DrawError>(
        &self,
        target: &mut impl DrawTarget<Color = C, Error = DrawError>,
        position: Rectangle,
    ) -> Result<(), DrawError> {
        self.layoutable
            .draw_placed(target, Self::place(self.layoutable.size(), position))
    }
}

pub struct CenteredAlignment;

impl Alignment for CenteredAlignment {
    #[inline]
    fn place(
        available_range: Saturating<u32>,
        target_range: ValueRange<Saturating<u32>>,
    ) -> (Saturating<i32>, Saturating<u32>) {
        if target_range.max_value < available_range {
            (
                Saturating((available_range - target_range.max_value).0 as i32 / 2),
                target_range.max_value,
            )
        } else if target_range.min_value > available_range {
            (
                (Saturating(available_range.0 as i32)
                    - Saturating(target_range.min_value.0 as i32))
                    / Saturating(2),
                target_range.min_value,
            )
        } else {
            (Saturating(0), available_range)
        }
    }
}

pub struct StartAlignment;

impl Alignment for StartAlignment {
    #[inline]
    fn place(
        available_range: Saturating<u32>,
        target_range: ValueRange<Saturating<u32>>,
    ) -> (Saturating<i32>, Saturating<u32>) {
        if target_range.max_value < available_range {
            (Saturating(0), target_range.max_value)
        } else if target_range.min_value > available_range {
            (Saturating(0), target_range.min_value)
        } else {
            (Saturating(0), available_range)
        }
    }
}

pub struct EndAlignment;

impl Alignment for EndAlignment {
    #[inline]
    fn place(
        available_range: Saturating<u32>,
        target_range: ValueRange<Saturating<u32>>,
    ) -> (Saturating<i32>, Saturating<u32>) {
        if target_range.max_value < available_range {
            (
                Saturating(available_range.0 as i32) - Saturating(target_range.max_value.0 as i32),
                target_range.max_value,
            )
        } else if target_range.min_value > available_range {
            (
                Saturating(available_range.0 as i32) - Saturating(target_range.min_value.0 as i32),
                target_range.min_value,
            )
        } else {
            (Saturating(0), available_range)
        }
    }
}
