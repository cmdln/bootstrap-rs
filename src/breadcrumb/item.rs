use crate::prelude::*;
use yew::{html::Children, prelude::*};

pub struct BreadcrumbItem {
    props: Props,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub active: bool,
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

impl Component for BreadcrumbItem {
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
        if self.props.active {
            html! {
                <li class=self.classes() aria-current="page">
                    { self.props.children.render() }
                </li>
            }
        } else {
            html! {
                <li class=self.classes()>
                    { self.props.children.render() }
                </li>
            }
        }
    }
}

impl BreadcrumbItem {
    fn classes(&self) -> String {
        if self.props.active {
            calculate_classes("breadcrumb-item active", (&self.props).into())
        } else {
            calculate_classes("breadcrumb-item", (&self.props).into())
        }
    }
}

impl<'a> From<&'a Props> for BootstrapProps<'a> {
    fn from(props: &Props) -> BootstrapProps {
        let class = &props.class;
        let borders = collect_bs(&props.border, &props.borders);
        let margins = collect_bs(&props.margin, &props.margins);
        let paddings = collect_bs(&props.padding, &props.paddings);
        BootstrapProps {
            class,
            borders,
            margins,
            paddings,
        }
    }
}
