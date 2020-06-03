mod border;
mod color;
mod margin;
mod padding;

pub use self::{border::Border, color::Color, margin::Margin, padding::Padding};
use std::fmt::{Display, Formatter, Result};
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq, Default)]
pub struct Props {
    #[prop_or_default]
    pub aria_label: String,
    #[prop_or_default]
    pub role: String,
    #[prop_or_default]
    pub border: Option<Border>,
    #[prop_or_default]
    pub borders: Vec<Border>,
    #[prop_or_default]
    pub margin: Option<Margin>,
    #[prop_or_default]
    pub margins: Vec<Margin>,
    #[prop_or_default]
    pub padding: Option<Padding>,
    #[prop_or_default]
    pub paddings: Vec<Padding>,
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub style: String,
    #[prop_or_default]
    pub children: Children,
}

impl<'a> From<&'a Props> for BootstrapProps<'a> {
    fn from(props: &Props) -> BootstrapProps {
        let class = &props.class;
        let borders = collect_bs(&props.border, &props.borders);
        let margins = collect_bs(&props.margin, &props.margins);
        let paddings = collect_bs(&props.padding, &props.paddings);
        BootstrapProps {
            class,
            borders,
            margins,
            paddings,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Edge {
    All,
    Top,
    Right,
    Bottom,
    Left,
    LeftAndRight,
    TopAndBottom,
}

impl Display for Edge {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        match self {
            Self::All => write!(fmt, ""),
            Self::Top => write!(fmt, "t"),
            Self::Bottom => write!(fmt, "b"),
            Self::Right => write!(fmt, "r"),
            Self::Left => write!(fmt, "l"),
            Self::LeftAndRight => write!(fmt, "x"),
            Self::TopAndBottom => write!(fmt, "y"),
        }
    }
}

impl Edge {
    fn suffix(&self) -> &str {
        match self {
            Self::Top => "-top",
            _ => "",
        }
    }

    fn with_prefix<S: AsRef<str>>(&self, prefix: S) -> String {
        match self {
            Self::All => prefix.as_ref().to_owned(),
            Self::LeftAndRight => format!(
                "{0}{1} {0}{2}",
                prefix.as_ref(),
                Self::Left.suffix(),
                Self::Right.suffix()
            ),
            Self::TopAndBottom => format!(
                "{0}{1} {0}{2}",
                prefix.as_ref(),
                Self::Top.suffix(),
                Self::Bottom.suffix()
            ),
            _ => format!("{}{}", prefix.as_ref(), self.suffix()),
        }
    }
}

trait BootstrapClass {
    fn as_classname(&self) -> String;
}

pub struct BootstrapProps<'a> {
    pub class: &'a str,
    pub borders: Vec<&'a Border>,
    pub margins: Vec<&'a Margin>,
    pub paddings: Vec<&'a Padding>,
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

pub fn calculate_classes<S: AsRef<str>>(prefix: S, props: BootstrapProps) -> String {
    let BootstrapProps {
        class,
        borders,
        margins,
        paddings,
    } = props;
    let mut classes = Vec::new();
    if !prefix.as_ref().is_empty() {
        classes.push(prefix.as_ref().to_owned());
    }
    if !props.class.is_empty() {
        classes.push(class.to_owned());
    }
    classes.append(&mut into_classnames(borders));
    classes.append(&mut into_classnames(margins));
    classes.append(&mut into_classnames(paddings));

    classes.join(" ")
}

fn into_classnames<C: BootstrapClass>(c: Vec<&C>) -> Vec<String> {
    c.into_iter().map(|c| c.as_classname()).collect()
}
