use crate::prelude::*;
use yew::{html::Children, prelude::*};

pub struct ButtonGroup {
    props: Props,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub style: String,
    #[prop_or_default]
    pub role: String,
    #[prop_or_default]
    pub aria_label: String,
    #[prop_or_default]
    pub children: Children,
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
        render_on_change(&mut self.props, props)
    }

    fn view(&self) -> Html {
        html! {
            <div class=self.class()
                style=self.style()
                role=self.props.role.clone()
                aria-label=self.props.aria_label.clone()
            >
                { self.props.children.render() }
            </div>
        }
    }
}

impl ButtonGroup {
    fn class(&self) -> String {
        if self.props.class.is_empty() {
            "btn-grp".into()
        } else {
            format!("btn-group {}", self.props.class)
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
