use crate::prelude::*;
use yew::{html::Children, prelude::*};

pub struct InputGroup {
    props: Props,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub margin: Option<Margin>,
    #[prop_or_default]
    pub margins: Vec<Margin>,
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub children: Children,
}

impl<'a> From<&'a Props> for BootstrapProps<'a> {
    fn from(props: &'a Props) -> BootstrapProps<'a> {
        let borders = Vec::new();
        let border_colors = Vec::new();
        let margins = collect_bs(&props.margin, &props.margins);
        Self {
            borders,
            border_colors,
            margins,
        }
    }
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
        let classes = calculate_classes((&self.props).into());
        html! {
            <div class=format!("input-group {} {}", classes, self.props.class)>
                { self.props.children.render() }
            </div>
        }
    }
}
