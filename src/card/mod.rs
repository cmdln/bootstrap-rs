mod body;
mod header;
mod text;

pub use self::{body::CardBody, header::CardHeader, text::CardText};
use crate::prelude::*;
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
        render_on_change(&mut self.props, props)
    }

    fn view(&self) -> Html {
        let class = calculate_classes("card", (&self.props).into());
        html! {
            <div class=class style=self.props.style.clone()>
                { self.props.children.render() }
            </div>
        }
    }
}
