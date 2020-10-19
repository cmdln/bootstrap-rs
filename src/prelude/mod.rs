mod border;
mod color;
mod edge;
mod margin;
mod padding;

pub use self::{border::Border, color::Color, edge::Edge, margin::Margin, padding::Padding};
use yew::prelude::*;

/// A newtype that can be helpful to work with `String` values the represent direct user input.
///
/// This type is convertible into `Option<String>` which makes it helpful for wrapping user input
/// for optional fields when sent to the server. Leaving the wrapped type as a `String` value while
/// handling input and validation helps preserve the user's text as entered so they can continue to
/// edit or correct as needed.
pub struct InputString<S: AsRef<str>>(pub S);

impl<S: AsRef<str>> Into<Option<String>> for InputString<S> {
    fn into(self) -> Option<String> {
        let InputString(input) = self;
        if input.as_ref().is_empty() {
            None
        } else {
            Some(input.as_ref().to_owned())
        }
    }
}

/// A convenience method for determining `ShouldRender` based on whether a value has changed, i.e.
/// the existing and new values are not equal, and its side effect of assigning `argument` to `assign_to` for the same condition.
///
/// Be careful if you choose to logically `or` calls of this function together for multiple
/// properties or state fields since short circuiting of any compound conditional may yield
/// surprising results, specifically failure to assign some fields. For this use case, collect the
/// results of each call into a collection then use reduce that collection to produce the result of
/// applying `or` to all its members.
pub fn render_if_ne<T: PartialEq>(assign_to: &mut T, argument: T) -> ShouldRender {
    if assign_to != &argument {
        *assign_to = argument;
        true
    } else {
        false
    }
}

/// A convenience method for determining which validation class to use for some input field
/// including one that has not yet been validating, represented by the argument having a `None`
/// value.
pub fn valid_as_class(v: &Option<bool>) -> &'static str {
    match v {
        None => "",
        Some(true) => " is-valid",
        Some(false) => " is-invalid",
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn render_true() {
        let mut t = String::from("foo");
        let should = render_if_ne(&mut t, String::from("bar"));
        assert!(should, "Should have determined to render");
        assert_eq!(t, String::from("bar"));
    }

    #[test]
    fn render_false() {
        let mut t = String::from("foo");
        let should = render_if_ne(&mut t, String::from("foo"));
        assert!(!should, "Should have determined to render");
        assert_eq!(t, String::from("foo"));
    }

    #[test]
    fn render_if_opt() {
        let mut t = Some(String::from("test"));
        let should = render_if_ne(&mut t, InputString(String::default()).into());
        assert!(should, "Should have determined to render");
        assert_eq!(t, None);
    }
}
