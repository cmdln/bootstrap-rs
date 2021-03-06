use crate::{prelude::*, props::Props, render};
use yew::prelude::*;

pub struct ButtonGroup {
    props: Props,
}

impl Component for ButtonGroup {
    type Properties = Props;
    type Message = ();

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        render_if_ne(&mut self.props, props)
    }

    fn view(&self) -> Html {
        render::render_with_prefix(&self.props, "btn-group", render::div(&self.props.children))
    }
}
