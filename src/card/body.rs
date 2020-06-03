use crate::prelude::*;
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
        render_on_change(&mut self.props, props)
    }

    fn view(&self) -> Html {
        let class = calculate_classes("card-body", (&self.props).into());
        html! {
            <div class=class style=self.props.style.clone()>
                { self.props.children.render() }
            </div>
        }
    }
}
