use crate::prelude::*;
use yew::prelude::*;

pub struct TextArea {
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
}

impl Component for TextArea {
    type Message = InputChange;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = String::default();
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
        render_on_change(&mut self.props, props)
    }

    fn view(&self) -> Html {
        html! {
            <textarea
                name=&self.props.name
                id=&self.props.id
                class="form-control"
                onchange=self.link.callback(|evt| InputChange(evt))
            >
                { &self.state }
            </textarea>
        }
    }
}
