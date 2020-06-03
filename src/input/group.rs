use crate::prelude::*;
use yew::prelude::*;

pub struct InputGroup {
    props: Props,
}

impl Component for InputGroup {
    type Message = ();
    type Properties = Props;

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
        let class = calculate_classes("input-group", (&self.props).into());
        html! {
            <div class=class>
                { self.props.children.render() }
            </div>
        }
    }
}
