mod group;
mod props;
mod toolbar;

use self::props::Props;
pub use self::{group::ButtonGroup, toolbar::ButtonToolbar};
use crate::{prelude::render_on_change, render};
use std::fmt::{Display, Formatter, Result};
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum ButtonType {
    Button,
    Submit,
    Reset,
}

impl Default for ButtonType {
    fn default() -> Self {
        ButtonType::Button
    }
}

impl Display for ButtonType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use ButtonType::*;
        match self {
            Submit => write!(f, "submit"),
            Reset => write!(f, "reset"),
            _ => write!(f, "button"),
        }
    }
}

pub struct Button {
    link: ComponentLink<Self>,
    props: Props,
}

impl Component for Button {
    type Properties = Props;
    type Message = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        self.props.on_click.emit(());
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        render_on_change(&mut self.props, props)
    }

    fn view(&self) -> Html {
        let html = html! {
            <button
                button=self.props.button_type
                onclick=self.link.callback(|_| ())
                disabled=self.props.disabled.unwrap_or_default()
            >
            { self.props.children.render() }
            </button>
        };
        render::render_with_prefix(&self.props, self.calculate_prefix(), html)
    }
}

impl Button {
    fn calculate_prefix(&self) -> Classes {
        let mut prefix = Classes::from("btn");
        if self.props.active.unwrap_or_default() {
            prefix.push("active");
        }
        if let Some(ref color) = self.props.color {
            prefix.push(&color.with_prefix("btn"));
        }
        if let Some(ref outline) = self.props.outline {
            prefix.push(&outline.with_prefix("btn-outline"));
        }
        if self.props.text_nowrap.unwrap_or_default() {
            prefix.push("text-nowrap");
        }
        if self.props.small.unwrap_or_default() {
            prefix.push("btn-sm");
        }
        if self.props.large.unwrap_or_default() {
            prefix.push("btn-lg");
        }
        if self.props.block.unwrap_or_default() {
            prefix.push("btn-block");
        }
        prefix
    }
}
