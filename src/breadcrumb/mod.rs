mod item;

use crate::{prelude::*, props::*, render};
pub use item::BreadcrumbItem;
use yew::prelude::*;

pub struct Breadcrumb {
    props: Props,
}

impl Component for Breadcrumb {
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
        let html = html! {
            <nav aria-label="breadcrumb">
                <ol class="breadcrumb">
                    { self.props.children.clone() }
                </ol>
            </nav>
        };
        render::render_with_prefix(&self.props, "breadcrumb", html)
    }
}
