/// Wraps an element for allocation.
pub type ElementWrap<T>=Box<T>;

/// An element that can be rendered, i.e. produce other elements.
pub trait Element {
    fn render(self: ElementWrap<Self>) -> Elements;
}

/// An array of elements, i.e. an element fragment.
pub type Elements = Vec<ElementWrap<dyn Element>>;
