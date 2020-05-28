mod border;
mod border_color;
mod margin;

pub use self::{border::Border, border_color::BorderColor, margin::Margin};
use std::fmt::{Display, Formatter, Result};
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Edge {
    Top,
    Right,
    Bottom,
    Left,
}

impl Display for Edge {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        match self {
            Self::Top => write!(fmt, "t"),
            Self::Bottom => write!(fmt, "b"),
            Self::Right => write!(fmt, "r"),
            Self::Left => write!(fmt, "l"),
        }
    }
}

trait BootstrapClass {
    fn as_classname(&self) -> String;
}

pub struct BootstrapProps<'a> {
    pub borders: Vec<&'a Border>,
    pub border_colors: Vec<&'a BorderColor>,
    pub margins: Vec<&'a Margin>,
}

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

pub fn collect_bs<'a, T>(t: &'a Option<T>, ts: &'a [T]) -> Vec<&'a T> {
    if let Some(t) = t.as_ref() {
        let mut r = vec![t];
        r.append(&mut ts.iter().collect());
        r
    } else {
        ts.iter().collect()
    }
}

pub fn calculate_classes(props: BootstrapProps) -> String {
    let BootstrapProps {
        borders,
        border_colors,
        margins,
    } = props;
    let mut classes = Vec::new();
    classes.append(&mut into_classnames(borders));
    classes.append(&mut into_classnames(border_colors));
    classes.append(&mut into_classnames(margins));

    classes.join(" ")
}

fn into_classnames<C: BootstrapClass>(c: Vec<&C>) -> Vec<String> {
    c.into_iter().map(|c| c.as_classname()).collect()
}
