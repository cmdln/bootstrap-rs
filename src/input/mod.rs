mod group;
mod textarea;

pub use self::{group::InputGroup, textarea::TextArea};
use crate::prelude::*;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum InputType {
    Text,
}

impl InputType {
    fn as_str(&self) -> &str {
        match self {
            Self::Text => "text",
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

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub name: String,
    #[prop_or_default]
    pub id: String,
    pub on_signal: Callback<String>,
    pub input_type: InputType,
    #[prop_or_default]
    pub readonly: bool,
    #[prop_or_default]
    pub value: String,
}

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
            self.props.on_signal.emit(value);
            true
        } else {
            false
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.value != props.value {
            self.state = props.value.clone();
        }
        render_on_change(&mut self.props, props)
    }

    fn view(&self) -> Html {
        let class = if self.props.readonly {
            "form-control-plaintext pl-3"
        } else {
            "form-control"
        };
        html! {
            <input
                name=&self.props.name
                id=&self.props.id
                type=self.props.input_type.as_str()
                class=class
                value=&self.state
                readonly=self.props.readonly
                onchange=self.link.callback(|evt| InputChange(evt))
            />
        }
    }
}
