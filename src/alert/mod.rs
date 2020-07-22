mod props;

use self::props::Props;
use crate::{prelude::*, render};
use yew::prelude::*;

pub struct Alert {
    link: ComponentLink<Self>,
    props: Props,
}

impl Component for Alert {
    type Properties = Props;
    type Message = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        self.props.on_close.emit(());
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        render_on_change(&mut self.props, props)
    }

    fn view(&self) -> Html {
        let html = html! {
            <div>
                { self.props.children.clone() }
                <button
                    type="button"
                    class="close"
                    data-dismiss="alert"
                    aria-label="Close"
                    onclick=self.link.callback(|_| ())
                >
                    <span aria-hidden="true">{ "×" }</span>
                </button>
            </div>
        };
        render::render_with_prefix(
            &self.props,
            vec!["alert", &self.props.color.with_prefix("alert")],
            html,
        )
    }
}
