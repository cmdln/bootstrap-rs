mod group;
mod props;
mod textarea;

use self::props::Props;
pub use self::{group::InputGroup, textarea::TextArea};
use crate::{prelude::*, render};
use std::fmt::{Display, Formatter, Result as FmtResult};
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum InputType {
    Text,
    Date,
    DateTime,
    Checkbox,
    Color,
}

impl Display for InputType {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Self::Text => write!(f, "text"),
            Self::Date => write!(f, "date"),
            Self::DateTime => write!(f, "datetime-local"),
            Self::Checkbox => write!(f, "checkbox"),
            Self::Color => write!(f, "color"),
        }
    }
}

pub struct Input {
    link: ComponentLink<Self>,
    state: String,
    props: Props,
}

#[derive(Debug)]
pub struct InputChange(ChangeData);

impl Component for Input {
    type Message = InputChange;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = props.value.clone();
        Self { props, state, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        if let InputChange(ChangeData::Value(value)) = msg {
            self.state = value.clone();
            self.props.on_change.emit(value);
            true
        } else {
            false
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.value != props.value {
            self.state = props.value.clone();
        }
        render_if_ne(&mut self.props, props)
    }

    fn view(&self) -> Html {
        let input_type = self
            .props
            .input_type
            .as_ref()
            .unwrap_or_else(|| &InputType::Text);
        let mut prefix = if self.props.readonly {
            vec!["form-control-plaintext"]
        } else {
            vec!["form-control"]
        };
        prefix.push(valid_as_class(&self.props.valid));
        let html = html! {
            <input
                type=input_type
                value=&self.state
                readonly=self.props.readonly
                onchange=self.link.callback(|evt| InputChange(evt))
            />
        };
        render::render_with_prefix(&self.props, prefix, html)
    }
}
