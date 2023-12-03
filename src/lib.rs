use std::num::Saturating;
use std::ops::{Add, AddAssign, Range, Sub};

mod draw;

mod align;
mod border;
mod expand;
mod layoutable;
mod linear;
mod padding;
mod placement;
mod scale;

pub mod prelude {
    pub use crate::{
        align::{center, east, north, south, west},
        border::{bordered, DashedLine, RoundedLine},
        expand::{expand, expand_horizontal, expand_vertical},
        layoutable::Layoutable,
        linear::{horizontal_layout, vertical_layout},
        padding::padding,
        placement::{callback_placement, optional_placement},
        scale::scale,
    };
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ComponentSize {
    width: ValueRange<Saturating<u32>>,
    height: ValueRange<Saturating<u32>>,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ValueRange<V> {
    preferred_value: V,
    min_value: V,
    max_value: V,
}

impl<V> Add<V> for ValueRange<V>
where
    V: Add<V, Output = V> + Clone,
{
    type Output = ValueRange<V>;

    fn add(self, rhs: V) -> Self::Output {
        ValueRange {
            preferred_value: self.preferred_value + rhs.clone(),
            min_value: self.min_value + rhs.clone(),
            max_value: self.max_value + rhs,
        }
    }
}
impl Add<i32> for ValueRange<Saturating<u32>> {
    type Output = ValueRange<Saturating<u32>>;

    fn add(self, rhs: i32) -> Self::Output {
        if rhs.is_negative() {
            let offset = Saturating(-rhs as u32);
            ValueRange {
                preferred_value: self.preferred_value - offset,
                min_value: self.min_value - offset,
                max_value: self.max_value - offset,
            }
        } else {
            let offset = Saturating(rhs as u32);
            ValueRange {
                preferred_value: self.preferred_value + offset,
                min_value: self.min_value + offset,
                max_value: self.max_value + offset,
            }
        }
    }
}
impl<V> Sub<V> for ValueRange<V>
where
    V: Sub<V, Output = V> + Clone,
{
    type Output = ValueRange<V>;

    fn sub(self, rhs: V) -> Self::Output {
        ValueRange {
            preferred_value: self.preferred_value - rhs.clone(),
            min_value: self.min_value - rhs.clone(),
            max_value: self.max_value - rhs,
        }
    }
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

impl<V: Clone> ValueRange<Saturating<V>> {
    fn fixed(value: V) -> Self {
        Self {
            preferred_value: Saturating(value.clone()),
            min_value: Saturating(value.clone()),
            max_value: Saturating(value),
        }
    }
}

impl ValueRange<Saturating<u32>> {
    fn expand_max(&self) -> Self {
        Self {
            preferred_value: self.preferred_value,
            min_value: self.min_value,
            max_value: Saturating(u32::MAX),
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
                preferred_value: Saturating(preferred_width),
                min_value: Saturating(width_range.start),
                max_value: Saturating(width_range.end),
            },
            height: ValueRange {
                preferred_value: Saturating(preferred_height),
                min_value: Saturating(height_range.start),
                max_value: Saturating(height_range.end),
            },
        }
    }
}
