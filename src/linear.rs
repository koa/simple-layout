use std::num::Saturating;
use std::{cmp::Ordering, marker::PhantomData, ops::Deref};

use embedded_graphics::{
    pixelcolor::PixelColor,
    prelude::{DrawTarget, Point, Size},
    primitives::Rectangle,
};

use crate::{layoutable::Layoutable, ComponentSize, ValueRange};

pub trait Orientation {
    fn split_component_size(
        size: ComponentSize,
    ) -> (ValueRange<Saturating<u32>>, ValueRange<Saturating<u32>>);
    fn split_size(size: Size) -> (Saturating<u32>, Saturating<u32>);
    fn split_point(p: Point) -> (Saturating<i32>, Saturating<i32>);
    fn create_component_size(
        along: ValueRange<Saturating<u32>>,
        cross: ValueRange<Saturating<u32>>,
    ) -> ComponentSize;
    fn create_size(along: Saturating<u32>, across: Saturating<u32>) -> Size;
    fn create_point(along: Saturating<i32>, cross: Saturating<i32>) -> Point;
}

pub struct Horizontal {}

impl Orientation for Horizontal {
    #[inline]
    fn split_component_size(
        size: ComponentSize,
    ) -> (ValueRange<Saturating<u32>>, ValueRange<Saturating<u32>>) {
        (size.width, size.height)
    }

    #[inline]
    fn split_size(size: Size) -> (Saturating<u32>, Saturating<u32>) {
        (Saturating(size.width), Saturating(size.height))
    }

    #[inline]
    fn split_point(p: Point) -> (Saturating<i32>, Saturating<i32>) {
        let Point { x, y } = p;
        (Saturating(x), Saturating(y))
    }

    #[inline]
    fn create_component_size(
        along: ValueRange<Saturating<u32>>,
        cross: ValueRange<Saturating<u32>>,
    ) -> ComponentSize {
        ComponentSize {
            width: along,
            height: cross,
        }
    }

    #[inline]
    fn create_size(along: Saturating<u32>, across: Saturating<u32>) -> Size {
        Size {
            width: along.0,
            height: across.0,
        }
    }

    #[inline]
    fn create_point(along: Saturating<i32>, cross: Saturating<i32>) -> Point {
        Point {
            x: along.0,
            y: cross.0,
        }
    }
}

pub struct Vertical {}

impl Orientation for Vertical {
    fn split_component_size(
        size: ComponentSize,
    ) -> (ValueRange<Saturating<u32>>, ValueRange<Saturating<u32>>) {
        (size.height, size.width)
    }

    fn split_size(size: Size) -> (Saturating<u32>, Saturating<u32>) {
        (Saturating(size.height), Saturating(size.width))
    }

    fn split_point(p: Point) -> (Saturating<i32>, Saturating<i32>) {
        (Saturating(p.y), Saturating(p.x))
    }

    fn create_component_size(
        along: ValueRange<Saturating<u32>>,
        cross: ValueRange<Saturating<u32>>,
    ) -> ComponentSize {
        ComponentSize {
            width: cross,
            height: along,
        }
    }

    fn create_size(along: Saturating<u32>, across: Saturating<u32>) -> Size {
        Size {
            width: across.0,
            height: along.0,
        }
    }

    fn create_point(along: Saturating<i32>, cross: Saturating<i32>) -> Point {
        Point {
            x: cross.0,
            y: along.0,
        }
    }
}

pub trait LinearLayout<C: PixelColor, O: Orientation>: Sized {
    fn len() -> usize;
    fn fill_sizes(&self, sizes: &mut [ComponentSize]);
    fn fill_weights(&self, weights: &mut [u32]);
    fn draw_placed_components<DrawError>(
        &self,
        target: &mut impl DrawTarget<Color = C, Error = DrawError>,
        places: &[Rectangle],
    ) -> Result<(), DrawError>;
}

#[derive(Default, Debug)]
pub struct SingleLinearLayout<L: Layoutable<C>, C: PixelColor, O: Orientation> {
    layout: L,
    weight: u32,
    p1: PhantomData<C>,
    p2: PhantomData<O>,
}

impl<L: Layoutable<C>, C: PixelColor, O: Orientation> Layoutable<C>
    for SingleLinearLayout<L, C, O>
{
    fn size(&self) -> ComponentSize {
        self.layout.size()
    }

    fn draw_placed<DrawError>(
        &self,
        target: &mut impl DrawTarget<Color = C, Error = DrawError>,
        position: Rectangle,
    ) -> Result<(), DrawError> {
        self.layout.draw_placed(target, position)
    }
}

impl<L: Layoutable<C>, C: PixelColor, O: Orientation> LinearLayout<C, O>
    for SingleLinearLayout<L, C, O>
{
    #[inline]
    fn len() -> usize {
        1
    }

    #[inline]
    fn fill_sizes(&self, sizes: &mut [ComponentSize]) {
        sizes[0] = self.size();
    }

    #[inline]
    fn fill_weights(&self, weights: &mut [u32]) {
        weights[0] = self.weight;
    }

    #[inline]
    fn draw_placed_components<DrawError>(
        &self,
        target: &mut impl DrawTarget<Color = C, Error = DrawError>,
        places: &[Rectangle],
    ) -> Result<(), DrawError> {
        self.layout.draw_placed(target, places[0])
    }
}

pub struct LayoutableLinearLayout<C: PixelColor, O: Orientation, LL: LinearLayout<C, O>>(
    LL,
    PhantomData<C>,
    PhantomData<O>,
);

impl<C: PixelColor, O: Orientation, LL: LinearLayout<C, O>> LayoutableLinearLayout<C, O, LL> {
    ///
    /// append an additional element to the current linear stack
    ///
    /// # Arguments
    ///
    /// * `element`: new element
    /// * `weight`: weight of the element
    ///
    /// returns: LayoutableLinearLayout<C, O, ChainingLinearLayout<LL, L, C, O>>
    ///
    pub fn append<L>(
        self,
        element: L,
        weight: u32,
    ) -> LayoutableLinearLayout<C, O, ChainingLinearLayout<LL, L, C, O>>
    where
        L: Layoutable<C>,
    {
        LayoutableLinearLayout(
            ChainingLinearLayout {
                base_layout: self.0,
                layoutable: element,
                weight,
                p: Default::default(),
                o: Default::default(),
            },
            PhantomData,
            PhantomData,
        )
    }
}

impl<C: PixelColor, O: Orientation, LL: LinearLayout<C, O>> From<LL>
    for LayoutableLinearLayout<C, O, LL>
{
    fn from(value: LL) -> Self {
        LayoutableLinearLayout(value, PhantomData, PhantomData)
    }
}

impl<C: PixelColor, O: Orientation, LL: LinearLayout<C, O>> Layoutable<C>
    for LayoutableLinearLayout<C, O, LL>
{
    fn size(&self) -> ComponentSize {
        let mut sizes = vec![ComponentSize::default(); LL::len()].into_boxed_slice();
        self.0.fill_sizes(&mut sizes);
        let mut total_along = ValueRange::default();
        let mut total_cross = ValueRange::default();
        for size in sizes.iter() {
            let (along, cross) = O::split_component_size(*size);
            total_along += along;
            total_cross.expand(&cross);
        }
        O::create_component_size(total_along, total_cross)
    }
    fn draw_placed<DrawError>(
        &self,
        target: &mut impl DrawTarget<Color = C, Error = DrawError>,
        position: Rectangle,
    ) -> Result<(), DrawError> {
        let (along_target, cross_target) = O::split_size(position.size);
        let (mut along_offset, cross_offset) = O::split_point(position.top_left);

        let mut sizes = vec![ComponentSize::default(); LL::len()].into_boxed_slice();
        self.0.fill_sizes(&mut sizes);
        let sizes = sizes
            .iter()
            .map(|s| O::split_component_size(*s).0)
            .collect::<Box<_>>();
        let preferred_sizes = sizes.iter().map(|s| s.preferred_value).collect::<Box<_>>();
        let total_preferred: Saturating<u32> =
            preferred_sizes.iter().fold(Saturating(0), |s, v| s + v);
        let places = match along_target.cmp(&total_preferred) {
            Ordering::Less => {
                let min_sizes = sizes.iter().map(|s| s.min_value).collect::<Box<_>>();
                let total_min = min_sizes.iter().fold(Saturating(0), |s, v| s + v);
                if total_min >= along_target {
                    min_sizes
                } else {
                    let mut remaining_budget = total_preferred - along_target;
                    let mut result_sizes = preferred_sizes;
                    let mut weights = vec![0; LL::len()].into_boxed_slice();
                    self.0.fill_weights(&mut weights);
                    while remaining_budget > Saturating(0) {
                        let remaining_budget_before = remaining_budget;
                        let mut entries_with_headroom = weights
                            .iter()
                            .zip(result_sizes.iter_mut())
                            .zip(sizes.iter())
                            .filter(|((weight, result_size), size)| {
                                **weight > 0 && **result_size > size.min_value
                            })
                            .collect::<Box<_>>();
                        let mut remaining_weights: u32 = entries_with_headroom
                            .iter()
                            .map(|((weight, _), _)| **weight)
                            .sum();
                        if remaining_weights == 0 {
                            break;
                        }
                        for ((weight, result_size), size) in entries_with_headroom.iter_mut() {
                            let theoretical_decrease = remaining_budget * Saturating(**weight)
                                / Saturating(remaining_weights);
                            let selected_decrease =
                                (theoretical_decrease).min(**result_size - size.min_value);
                            **result_size -= selected_decrease;
                            remaining_budget -= theoretical_decrease;
                            remaining_weights -= *weight;
                        }
                        if remaining_budget_before == remaining_budget {
                            // nothing more to distribute -> break
                            break;
                        }
                    }
                    result_sizes
                }
            }
            Ordering::Equal => preferred_sizes,
            Ordering::Greater => {
                let max_sizes = sizes.iter().map(|s| s.max_value).collect::<Box<_>>();
                let total_max = max_sizes.iter().fold(Saturating(0), |s, v| s + v);
                if total_max <= along_target {
                    max_sizes
                } else {
                    let mut remaining_budget = along_target - total_preferred;
                    let mut result_sizes = preferred_sizes;
                    let mut weights = vec![0; LL::len()].into_boxed_slice();
                    self.0.fill_weights(&mut weights);
                    while remaining_budget > Saturating(0) {
                        let remaining_budget_before = remaining_budget;
                        let mut entries_with_headroom = weights
                            .iter()
                            .zip(result_sizes.iter_mut())
                            .zip(sizes.iter())
                            .filter(|((weight, result_size), size)| {
                                **weight > 0 && **result_size < size.max_value
                            })
                            .collect::<Box<_>>();
                        let mut remaining_weights: u32 = entries_with_headroom
                            .iter()
                            .map(|((weight, _), _)| **weight)
                            .sum();
                        if remaining_weights == 0 {
                            break;
                        }

                        for ((weight, result_size), size) in entries_with_headroom.iter_mut() {
                            let theoretical_increase = remaining_budget * Saturating(**weight)
                                / Saturating(remaining_weights);
                            let selected_increase =
                                (theoretical_increase).min(size.max_value - **result_size);
                            **result_size += selected_increase;
                            remaining_budget -= theoretical_increase;
                            remaining_weights -= *weight;
                        }
                        if remaining_budget_before == remaining_budget {
                            // nothing more to distribute -> break
                            break;
                        }
                    }
                    result_sizes
                }
            }
        }
        .iter()
        .map(|l| {
            let place = Rectangle {
                top_left: O::create_point(along_offset, cross_offset),
                size: O::create_size(*l, cross_target),
            };
            along_offset += Saturating(l.0 as i32);
            place
        })
        .collect::<Box<_>>();
        self.0.draw_placed_components(target, &places)
    }
}

impl<C: PixelColor, O: Orientation, LL: LinearLayout<C, O>> Deref
    for LayoutableLinearLayout<C, O, LL>
{
    type Target = LL;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/*
impl<L1: Layoutable<C>, L2: Layoutable<C>, C: PixelColor, O: Orientation> From<(L1, L2)>
for LinearPair<L1, L2, C, O>
{
fn from((l1, l2): (L1, L2)) -> Self {
    Self {
        l1,
        l2,
        weights: [1, 1],
        p1: PhantomData,
        p2: PhantomData,
    }
}
}
*/
pub struct ChainingLinearLayout<
    LL: LinearLayout<C, O>,
    L: Layoutable<C>,
    C: PixelColor,
    O: Orientation,
> {
    base_layout: LL,
    layoutable: L,
    weight: u32,
    p: PhantomData<C>,
    o: PhantomData<O>,
}

impl<LL: LinearLayout<C, O>, L: Layoutable<C>, C: PixelColor, O: Orientation> LinearLayout<C, O>
    for ChainingLinearLayout<LL, L, C, O>
{
    #[inline]
    fn len() -> usize {
        LL::len() + 1
    }

    #[inline]
    fn fill_sizes(&self, sizes: &mut [ComponentSize]) {
        let idx = Self::len() - 1;
        self.base_layout.fill_sizes(&mut sizes[0..idx]);
        sizes[idx] = self.layoutable.size();
    }

    #[inline]
    fn fill_weights(&self, weights: &mut [u32]) {
        let idx = Self::len() - 1;
        self.base_layout.fill_weights(&mut weights[0..idx]);
        weights[idx] = self.weight;
    }

    #[inline]
    fn draw_placed_components<DrawError>(
        &self,
        target: &mut impl DrawTarget<Color = C, Error = DrawError>,
        places: &[Rectangle],
    ) -> Result<(), DrawError> {
        let idx = Self::len() - 1;
        self.base_layout
            .draw_placed_components(target, &places[0..idx])?;
        self.layoutable.draw_placed(target, places[idx])
    }
}

///
/// Stack multiple layout elements vertically
///
/// # Arguments
///
/// * `first_child`: First layout element to stack
/// * `first_child_weight`: Weight of this element when expansion or shrinking is needed to fit elements vertically
///
/// returns: LayoutableLinearLayout<C, Vertical, SingleLinearLayout<L, C, Vertical>>
///
/// # Examples
///
/// ```
/// use embedded_graphics::mono_font::iso_8859_1::FONT_6X12;
/// use embedded_graphics::mono_font::MonoTextStyle;
/// use embedded_graphics::pixelcolor::BinaryColor;
/// use embedded_graphics::prelude::Point;
/// use embedded_graphics::text::Text;
/// use simple_layout::prelude::{bordered, center, expand, horizontal_layout, optional_placement, owned_text, padding, RoundedLine, scale, vertical_layout};
/// const TEXT_STYLE: MonoTextStyle<BinaryColor> = MonoTextStyle::new(&FONT_6X12, BinaryColor::On);
/// let mut minus_button_placement=None;
/// let mut plus_button_placement=None;
/// let value = 0.7;
/// let data_visualization = vertical_layout(expand(center(owned_text(format!("{value:.1}"), TEXT_STYLE))),1,)
///                 .append(scale(value, BinaryColor::On), 0);
/// let numbered_scale = expand(center(
///         horizontal_layout(
///             center(optional_placement(
///                 &mut minus_button_placement,
///                 bordered(
///                     padding(Text::new("-", Point::zero(), TEXT_STYLE), -2, 1, -1, 1),
///                     RoundedLine::new(BinaryColor::On),
///                 ),
///             )),
///             0,
///         )
///         .append(data_visualization, 1)
///         .append(
///             center(optional_placement(
///                 &mut plus_button_placement,
///                 bordered(
///                     padding(Text::new("+", Point::zero(), TEXT_STYLE), -2, 1, -1, 1),
///                     RoundedLine::new(BinaryColor::On),
///                 ),
///             )),
///             0,
///         ),
///     ));
/// ```
pub fn vertical_layout<L: Layoutable<C>, C: PixelColor>(
    first_child: L,
    first_child_weight: u32,
) -> LayoutableLinearLayout<C, Vertical, SingleLinearLayout<L, C, Vertical>> {
    LayoutableLinearLayout(
        SingleLinearLayout {
            layout: first_child,
            weight: first_child_weight,
            p1: PhantomData,
            p2: PhantomData,
        },
        PhantomData,
        PhantomData,
    )
}

///
///
/// # Arguments
///
/// * `first_child`: First layout element to stack
/// * `first_child_weight`: Weight of this element when expansion or shrinking is needed to fit elements vertically
///
/// returns: LayoutableLinearLayout<C, Horizontal, SingleLinearLayout<L, C, Horizontal>>
///
pub fn horizontal_layout<L: Layoutable<C>, C: PixelColor>(
    first_child: L,
    first_child_weight: u32,
) -> LayoutableLinearLayout<C, Horizontal, SingleLinearLayout<L, C, Horizontal>> {
    LayoutableLinearLayout(
        SingleLinearLayout {
            layout: first_child,
            weight: first_child_weight,
            p1: PhantomData,
            p2: PhantomData,
        },
        PhantomData,
        PhantomData,
    )
}
