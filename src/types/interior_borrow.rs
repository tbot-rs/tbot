use std::borrow::Cow;

/// Creates a `T` out of a `&T`, but the new value contains references
/// to the original one.
pub trait InteriorBorrow<'a>: 'a {
    /// Creates a new owned `Self`, but its content are references
    /// to the original value.
    #[must_use]
    fn borrow_inside(&'a self) -> Self;
}

impl<'a, T: InteriorBorrow<'a>> InteriorBorrow<'a> for Vec<T> {
    fn borrow_inside(&'a self) -> Self {
        self.iter().map(InteriorBorrow::borrow_inside).collect()
    }
}

impl<'a, T: ToOwned + ?Sized + 'a> InteriorBorrow<'a> for Cow<'a, T> {
    fn borrow_inside(&'a self) -> Self {
        Cow::Borrowed(&**self)
    }
}

impl<'a, T: InteriorBorrow<'a>> InteriorBorrow<'a> for Option<T> {
    fn borrow_inside(&'a self) -> Self {
        self.as_ref().map(InteriorBorrow::borrow_inside)
    }
}
