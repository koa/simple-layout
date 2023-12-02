use std::marker::PhantomData;
use std::num::Saturating;

use embedded_graphics::{
    draw_target::DrawTarget, geometry::Point, pixelcolor::PixelColor, prelude::Size,
    primitives::Rectangle,
};

use crate::layoutable::Layoutable;
use crate::{ComponentSize, ValueRange};

pub fn center<L: Layoutable<C>, C: PixelColor>(
    l: L,
) -> AlignLayout<L, C, CenteredAlignment, CenteredAlignment> {
    AlignLayout::new(l)
}

pub fn west<L: Layoutable<C>, C: PixelColor>(
    l: L,
) -> AlignLayout<L, C, StartAlignment, CenteredAlignment> {
    AlignLayout::new(l)
}
pub fn east<L: Layoutable<C>, C: PixelColor>(
    l: L,
) -> AlignLayout<L, C, EndAlignment, CenteredAlignment> {
    AlignLayout::new(l)
}
pub fn north<L: Layoutable<C>, C: PixelColor>(
    l: L,
) -> AlignLayout<L, C, CenteredAlignment, StartAlignment> {
    AlignLayout::new(l)
}
pub fn south<L: Layoutable<C>, C: PixelColor>(
    l: L,
) -> AlignLayout<L, C, CenteredAlignment, EndAlignment> {
    AlignLayout::new(l)
}

trait Alignment {
    fn place(
        available_range: Saturating<u32>,
        target_range: ValueRange<Saturating<u32>>,
    ) -> (Saturating<i32>, Saturating<u32>);
}

pub struct AlignLayout<L: Layoutable<C>, C: PixelColor, VA: Alignment, HA: Alignment> {
    layoutable: L,
    p1: PhantomData<C>,
    p2: PhantomData<VA>,
    p3: PhantomData<HA>,
}

impl<L: Layoutable<C>, C: PixelColor, VA: Alignment, HA: Alignment> AlignLayout<L, C, VA, HA> {
    fn new(layoutable: L) -> Self {
        Self {
            layoutable,
            p1: PhantomData,
            p2: PhantomData,
            p3: PhantomData,
        }
    }
}

impl<L: Layoutable<C>, C: PixelColor, VA: Alignment, HA: Alignment> AlignLayout<L, C, VA, HA> {
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

impl<L: Layoutable<C>, C: PixelColor, VA: Alignment, HA: Alignment> Layoutable<C>
    for AlignLayout<L, C, VA, HA>
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
    ) -> Result<Point, DrawError> {
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
