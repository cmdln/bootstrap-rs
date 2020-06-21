use crate::{props::Props, render};
use yew::prelude::*;

pub struct Jumbotron {
    props: Props,
}

impl Component for Jumbotron {
    type Properties = Props;
    type Message = ();

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let html = html! {
            <div>
                { self.props.children.render() }
            </div>
        };
        render::render_with_prefix(&self.props, "jumbotron", html)
    }
}
