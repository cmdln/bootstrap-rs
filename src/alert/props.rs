use crate::{prelude::*, props::*};
use std::collections::HashMap;
use yew::prelude::*;

#[derive(Properties, Default, Clone, PartialEq)]
pub struct Props {
    // component specific
    pub on_close: Callback<()>,
    pub color: Color,

    // html specific
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: String,
    #[prop_or_default]
    pub aria_label: Option<String>,
    #[prop_or_default]
    pub role: Option<String>,
    #[prop_or_default]
    pub children: Children,

    // bootstrap
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
}

impl<'a> From<&'a Props> for BootstrapProps<'a> {
    fn from(props: &Props) -> BootstrapProps {
        let class = &props.class;
        let borders = collect_props(&props.border, &props.borders);
        let margins = collect_props(&props.margin, &props.margins);
        let paddings = collect_props(&props.padding, &props.paddings);
        let attributes = HashMap::new();
        BootstrapProps {
            class,
            borders,
            margins,
            paddings,
            attributes,
        }
    }
}
