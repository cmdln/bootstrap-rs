use crate::prelude::*;
use yew::{html::Children, prelude::*};

pub struct Jumbotron {
    props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
    #[prop_or_default]
    pub margin: Option<Margin>,
    #[prop_or_default]
    pub margins: Vec<Margin>,
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub children: Children,
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
        html! {
            <div class=format!("jumbotron {}", self.props.class)>
                { self.props.children.render() }
            </div>
        }
    }
}
