use super::InputType;
use crate::{
    prelude::*,
    props::{collect_props, BootstrapProps},
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
    pub children: Children,
}

impl<'a> From<&'a Props> for BootstrapProps<'a> {
    fn from(props: &Props) -> BootstrapProps {
        let class = &props.class;
        let borders = collect_props(&props.border, &props.borders);
        let margins = collect_props(&props.margin, &props.margins);
        let paddings = collect_props(&props.padding, &props.paddings);
        let mut attributes = HashMap::new();
        if let Some(ref style) = props.style {
            attributes.insert("style", style);
        }
        if let Some(ref id) = props.id {
            attributes.insert("id", id);
        }
        if let Some(ref name) = props.name {
            attributes.insert("name", name);
        }
        BootstrapProps {
            class,
            borders,
            margins,
            paddings,
            attributes,
        }
    }
}
