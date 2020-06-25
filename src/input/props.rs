use super::InputType;
use crate::{
    prelude::*,
    props::{add_opt_attr, collect_props, BootstrapProps},
};
use std::collections::HashMap;
use yew::{html::Children, prelude::*};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    // component specific
    #[prop_or_default]
    pub name: Option<String>,
    #[prop_or_default]
    pub value: String,
    #[prop_or_default]
    pub valid: Option<bool>,
    #[prop_or_default]
    pub on_change: Callback<String>,
    #[prop_or_default]
    pub readonly: bool,
    #[prop_or_default]
    pub input_type: Option<InputType>,

    // bootstrap specific
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

    // html specific
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: Option<String>,
    #[prop_or_default]
    pub aria_label: Option<String>,
    #[prop_or_default]
    pub aria_describedby: Option<String>,
    #[prop_or_default]
    pub children: Children,
}

impl<'a> From<&'a Props> for BootstrapProps<'a> {
    fn from(props: &Props) -> BootstrapProps {
        let class = &props.class;
        let borders = collect_props(&props.border, &props.borders);
        let margins = collect_props(&props.margin, &props.margins);
        let paddings = collect_props(&props.padding, &props.paddings);
        let mut attributes = HashMap::new();
        add_opt_attr(&mut attributes, "id", &props.id);
        add_opt_attr(&mut attributes, "name", &props.name);
        add_opt_attr(&mut attributes, "style", &props.style);
        add_opt_attr(&mut attributes, "aria-label", &props.aria_label);
        add_opt_attr(&mut attributes, "aria-describedby", &props.aria_describedby);
        BootstrapProps {
            class,
            borders,
            margins,
            paddings,
            attributes,
        }
    }
}
