use crate::prelude::*;
use yew::{html::Children, prelude::*};

pub struct BreadcrumbItem {
    props: Props,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub active: bool,
    #[prop_or_default]
    pub children: Children,
}

impl Component for BreadcrumbItem {
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
        if self.props.active {
            html! {
                <li class="breadcrumb-item active" aria-current="page">
                    { self.props.children.render() }
                </li>
            }
        } else {
            html! {
                <li class="breadcrumb-item">
                    { self.props.children.render() }
                </li>
            }
        }
    }
}
