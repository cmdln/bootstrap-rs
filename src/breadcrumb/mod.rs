mod item;

use crate::prelude::*;
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
        render_on_change(&mut self.props, props)
    }

    fn view(&self) -> Html {
        let class = calculate_classes("breadcrumb", (&self.props).into());
        html! {
            <nav
                class=class
                aria-label="breadcrumb"
                style=self.props.style.clone()
            >
                <ol
                    class="breadcrumb"
                >
                    { self.props.children.render() }
                </ol>
            </nav>
        }
    }
}
