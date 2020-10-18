use crate::{prelude::*, props::*, render};
use yew::prelude::*;

pub struct CardHeader {
    props: Props,
}

impl Component for CardHeader {
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
        render::render_with_prefix(&self.props, "card-header", render::p(&self.props.children))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_comp() {
        let comp = CardHeader {
            props: Props::default(),
        };

        let expected = html! {
            <p class="card-header">
            </p>
        };

        assert_eq!(expected, comp.view());
    }
}
