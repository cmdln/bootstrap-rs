mod convert;

use crate::prelude::*;
use yew::{html::Children, prelude::*};

pub struct Alert {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Properties, Default, Clone, PartialEq)]
pub struct Props {
    pub on_close: Callback<()>,
    pub color: Color,
    #[prop_or_default]
    pub aria_label: String,
    #[prop_or_default]
    pub role: String,
    #[prop_or_default]
    pub border: Option<Border>,
    #[prop_or_default]
    pub borders: Vec<Border>,
    #[prop_or_default]
    pub margin: Option<Margin>,
    #[prop_or_default]
    pub margins: Vec<Margin>,
    #[prop_or_default]
    pub padding: Option<Padding>,
    #[prop_or_default]
    pub paddings: Vec<Padding>,
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub style: String,
    #[prop_or_default]
    pub children: Children,
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
        let color_class = self.props.color.with_prefix("alert-");
        let class = calculate_classes(format!("alert {}", color_class), (&self.props).into());
        html! {
            <div class=class>
                { self.props.children.render() }
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
        }
    }
}
