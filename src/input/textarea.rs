use super::props::Props;
use crate::{prelude::*, render};
use yew::{prelude::*, virtual_dom::VNode};

pub struct TextArea {
    link: ComponentLink<Self>,
    state: String,
    props: Props,
}

#[derive(Debug)]
pub struct InputChange(ChangeData);

impl Component for TextArea {
    type Message = InputChange;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = if props.children.is_empty() {
            props.value.clone()
        } else {
            let node = props.children.render();
            if let VNode::VText(text) = node {
                text.text
            } else {
                props.value.clone()
            }
        };
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
        let should_render = render_on_change(&mut self.props, props);
        if should_render {
            self.state = self.props.value.clone();
        }
        should_render
    }

    fn view(&self) -> Html {
        let prefix = vec!["form-control", &valid_as_class(&self.props.valid)];
        let html = html! {
            <textarea
                onchange=self.link.callback(|evt| InputChange(evt))
            >
                { &self.state }
            </textarea>
        };
        render::render_with_prefix(&self.props, prefix, html)
    }
}
