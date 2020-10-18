use crate::{prelude::*, props::*, render};
use yew::prelude::*;

pub struct Container {
    props: Props,
}

impl Component for Container {
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
        render::render_with_prefix(&self.props, "container", render::div(&self.props.children))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let container = Container {
            props: Props {
                margin: Some(Margin(Edge::All, 3)),
                padding: Some(Padding(Edge::Top, 3)),
                ..Props::default()
            },
        };
        let expected = html! {
            <div class="container m-3 pt-3">
            </div>
        };
        assert_eq!(expected, container.view());
    }
}
