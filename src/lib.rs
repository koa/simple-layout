use std::ops::{AddAssign, Range};

use embedded_graphics::{
    geometry::Dimensions, pixelcolor::PixelColor, prelude::DrawTarget, text::renderer::TextRenderer,
};

mod draw;

mod expand;
mod layoutable;
mod linear;

pub mod prelude {
    pub use crate::{
        expand::expand,
        layoutable::Layoutable,
        linear::{horizontal_layout, vertical_layout},
    };
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ComponentSize {
    width: ValueRange<u32>,
    height: ValueRange<u32>,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ValueRange<V> {
    preferred_value: V,
    min_value: V,
    max_value: V,
}

impl<V: PartialOrd + Clone> ValueRange<V> {
    fn expand(&mut self, rhs: &Self) {
        if self.preferred_value < rhs.preferred_value {
            self.preferred_value = rhs.preferred_value.clone();
        }
        if self.min_value < rhs.min_value {
            self.min_value = rhs.min_value.clone()
        }
        if self.max_value < rhs.max_value {
            self.max_value = rhs.max_value.clone()
        }
    }
}

impl<V: AddAssign> AddAssign for ValueRange<V> {
    fn add_assign(&mut self, rhs: Self) {
        self.preferred_value += rhs.preferred_value;
        self.min_value += rhs.min_value;
        self.max_value += rhs.max_value;
    }
}

impl<V: Clone> ValueRange<V> {
    fn fixed(value: V) -> Self {
        Self {
            preferred_value: value.clone(),
            min_value: value.clone(),
            max_value: value,
        }
    }
}

impl ValueRange<u32> {
    fn expand_max(&self) -> Self {
        Self {
            preferred_value: self.preferred_value,
            min_value: self.min_value,
            max_value: u32::MAX,
        }
    }
}

impl ComponentSize {
    pub fn fixed_size(width: u32, height: u32) -> ComponentSize {
        ComponentSize {
            width: ValueRange::fixed(width),
            height: ValueRange::fixed(height),
        }
    }
    pub fn new(
        preferred_width: u32,
        preferred_height: u32,
        width_range: Range<u32>,
        height_range: Range<u32>,
    ) -> Self {
        Self {
            width: ValueRange {
                preferred_value: preferred_width,
                min_value: width_range.start,
                max_value: width_range.end,
            },
            height: ValueRange {
                preferred_value: preferred_height,
                min_value: height_range.start,
                max_value: height_range.end,
            },
        }
    }
}
