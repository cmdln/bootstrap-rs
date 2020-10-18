mod body;
mod header;
mod text;

pub use self::{body::CardBody, header::CardHeader, text::CardText};
use crate::{prelude::*, props::*, render};
use yew::prelude::*;

pub struct Card {
    props: Props,
}

impl Component for Card {
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
        render::render_with_prefix(&self.props, "card", render::div(&self.props.children))
    }
}
