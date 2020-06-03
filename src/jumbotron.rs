use crate::prelude::*;
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
        let class = calculate_classes("jumbotron", (&self.props).into());
        html! {
            <div class=class>
                { self.props.children.render() }
            </div>
        }
    }
}
