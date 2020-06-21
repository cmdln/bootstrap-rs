mod border;
mod color;
mod edge;
mod margin;
mod padding;

pub use self::{border::Border, color::Color, edge::Edge, margin::Margin, padding::Padding};
use yew::prelude::*;

pub fn render_on_change<P: Properties + PartialEq>(
    props_on_comp: &mut P,
    props: P,
) -> ShouldRender {
    if props_on_comp == &props {
        false
    } else {
        *props_on_comp = props;
        true
    }
}

pub fn valid_as_class(v: &Option<bool>) -> &'static str {
    match v {
        None => "",
        Some(true) => " is-valid",
        Some(false) => " is-invalid",
    }
}
