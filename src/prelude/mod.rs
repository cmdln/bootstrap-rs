mod border;
mod color;
mod edge;
mod margin;
mod padding;

pub use self::{border::Border, color::Color, edge::Edge, margin::Margin, padding::Padding};
use yew::prelude::*;

pub fn render_if_ne<T: PartialEq>(assign_to: &mut T, argument: T) -> ShouldRender {
    render_if(assign_to, argument, identity, is_ne)
}

pub fn render_if_opt_str<S: AsRef<str>>(
    assign_to: &mut Option<String>,
    argument: S,
) -> ShouldRender {
    render_if(assign_to, argument, opt_from_str, is_ne)
}

pub fn render_if<T, A>(
    assign_to: &mut T,
    argument: A,
    map: impl Fn(A) -> T,
    assign_render: impl Fn(&mut T, &T) -> bool,
) -> ShouldRender {
    let argument = map(argument);
    if assign_render(assign_to, &argument) {
        *assign_to = argument;
        true
    } else {
        false
    }
}

pub fn valid_as_class(v: &Option<bool>) -> &'static str {
    match v {
        None => "",
        Some(true) => " is-valid",
        Some(false) => " is-invalid",
    }
}

fn identity<T>(t: T) -> T {
    t
}

fn opt_from_str<S: AsRef<str>>(a: S) -> Option<String> {
    if a.as_ref().is_empty() {
        None
    } else {
        Some(a.as_ref().to_owned())
    }
}

fn is_ne<T: PartialEq>(t: &mut T, a: &T) -> bool {
    t != a
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
        let should = render_if_opt_str(&mut t, String::default());
        assert!(should, "Should have determined to render");
        assert_eq!(t, None);
    }
}
