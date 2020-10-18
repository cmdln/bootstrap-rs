use crate::{prelude::*, props::*, render};
use yew::prelude::*;

pub struct FormGroup {
    pub props: Props,
}

impl Component for FormGroup {
    type Message = ();
    type Properties = Props;

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
        render::render_with_prefix(&self.props, "form-group", render::div(&self.props.children))
    }
}
