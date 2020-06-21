use crate::{prelude::*, props::*, render};
use yew::prelude::*;

pub struct CardText {
    props: Props,
}

impl Component for CardText {
    type Properties = Props;
    type Message = ();

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        render_on_change(&mut self.props, props)
    }

    fn view(&self) -> Html {
        render::render_with_prefix(&self.props, "card-text", render::p(&self.props.children))
    }
}
