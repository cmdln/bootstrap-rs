mod item;

use crate::prelude::*;
pub use item::BreadcrumbItem;
use yew::{html::Children, prelude::*};

pub struct Breadcrumb {
    props: Props,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub border: Option<Border>,
    #[prop_or_default]
    pub borders: Vec<Border>,
    #[prop_or_default]
    pub border_color: Option<BorderColor>,
    #[prop_or_default]
    pub border_colors: Vec<BorderColor>,
    #[prop_or_default]
    pub margin: Option<Margin>,
    #[prop_or_default]
    pub margins: Vec<Margin>,
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub style: String,
    #[prop_or_default]
    pub children: Children,
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
        html! {
            <nav
                class=self.props.class.clone()
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

impl<'a> From<&'a Props> for BootstrapProps<'a> {
    fn from(props: &Props) -> BootstrapProps {
        let borders = collect_bs(&props.border, &props.borders);
        let border_colors = collect_bs(&props.border_color, &props.border_colors);
        let margins = collect_bs(&props.margin, &props.margins);
        BootstrapProps {
            borders,
            border_colors,
            margins,
        }
    }
}
