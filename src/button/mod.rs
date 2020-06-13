use crate::prelude::*;
use yew::prelude::*;

pub struct ButtonGroup {
    props: Props,
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
                style=self.props.style.clone()
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
            "btn-group".into()
        } else {
            format!("btn-group {}", self.props.class)
        }
    }
}
