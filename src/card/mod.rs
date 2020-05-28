mod body;
mod header;
mod text;

pub use self::{body::CardBody, header::CardHeader, text::CardText};
use crate::prelude::*;
use yew::{html::Children, prelude::*};

pub struct Card {
    props: Props,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub margin: Option<Margin>,
    #[prop_or_default]
    pub margins: Vec<Margin>,
    #[prop_or_default]
    pub border: Option<Border>,
    #[prop_or_default]
    pub borders: Vec<Border>,
    #[prop_or_default]
    pub border_color: Option<BorderColor>,
    #[prop_or_default]
    pub border_colors: Vec<BorderColor>,
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub style: String,
    #[prop_or_default]
    pub children: Children,
}

impl Component for Card {
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
            <div class=self.class() style=self.style()>
                { self.props.children.render() }
            </div>
        }
    }
}

impl Card {
    fn class(&self) -> String {
        if self.props.class.is_empty() {
            format!("card {}", calculate_classes((&self.props).into()))
        } else {
            format!(
                "card {} {}",
                self.props.class,
                calculate_classes((&self.props).into())
            )
        }
    }

    fn style(&self) -> &str {
        if self.props.style.is_empty() {
            ""
        } else {
            &self.props.style
        }
    }
}

impl<'a> From<&'a Props> for BootstrapProps<'a> {
    fn from(props: &'a Props) -> Self {
        let borders = collect_bs(&props.border, &props.borders);
        let border_colors = collect_bs(&props.border_color, &props.border_colors);
        let margins = collect_bs(&props.margin, &props.margins);
        Self {
            borders,
            border_colors,
            margins,
        }
    }
}
