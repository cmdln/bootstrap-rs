use crate::{prelude::*, props::*, render};
use yew::prelude::*;

pub struct CardBody {
    props: Props,
}

impl Component for CardBody {
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
        render::render_with_prefix(&self.props, "card-body", render::div(&self.props.children))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_view() {
        let comp = CardBody {
            props: Props {
                style: Some("display: none;".into()),
                ..Props::default()
            },
        };

        let expected = html! {
            <div class="card-body" style="display: none;">
            </div>
        };

        assert_eq!(expected, comp.view());
    }
}
