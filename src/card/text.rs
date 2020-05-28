use crate::prelude::*;
use yew::{html::Children, prelude::*};

pub struct CardText {
    props: Props,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub style: String,
    #[prop_or_default]
    pub children: Children,
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
        html! {
            <p class=self.class() style=self.style()>
                { self.props.children.render() }
            </p>
        }
    }
}

impl CardText {
    fn class(&self) -> String {
        if self.props.class.is_empty() {
            "card-text".into()
        } else {
            format!("card-text {}", self.props.class)
        }
    }

    fn style(&self) -> &str {
        if self.props.style.is_empty() {
            ""
        } else {
            &self.props.style
        }
    }
}
