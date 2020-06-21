use crate::prelude::*;
use std::collections::HashMap;
use yew::{prelude::*, virtual_dom::VNode};

pub(crate) trait IntoBsClass {
    fn as_classname(&self) -> String;
}

pub(crate) fn collect_props<'a, T>(t: &'a Option<T>, ts: &'a [T]) -> Vec<&'a T> {
    if let Some(t) = t.as_ref() {
        let mut r = vec![t];
        r.append(&mut ts.iter().collect());
        r
    } else {
        ts.iter().collect()
    }
}

#[derive(Properties, Clone, PartialEq, Default)]
pub struct Props {
    // html specific
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub aria_label: Option<String>,
    #[prop_or_default]
    pub role: Option<String>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: Option<String>,
    #[prop_or_default]
    pub children: Children,

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
}

impl<'a> From<&'a Props> for BootstrapProps<'a> {
    fn from(props: &Props) -> BootstrapProps {
        let class = &props.class;
        let borders = collect_props(&props.border, &props.borders);
        let margins = collect_props(&props.margin, &props.margins);
        let paddings = collect_props(&props.padding, &props.paddings);
        let mut attributes = HashMap::new();
        if let Some(ref id) = props.id {
            attributes.insert("id", id);
        }
        if let Some(ref aria_label) = props.aria_label {
            attributes.insert("aria-label", aria_label);
        }
        if let Some(ref role) = props.role {
            attributes.insert("role", role);
        }
        if let Some(ref style) = props.style {
            attributes.insert("style", style);
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

pub struct BootstrapProps<'a> {
    pub class: &'a Classes,
    pub borders: Vec<&'a Border>,
    pub margins: Vec<&'a Margin>,
    pub paddings: Vec<&'a Padding>,
    pub attributes: HashMap<&'static str, &'a String>,
}

impl<'a> BootstrapProps<'a> {
    pub fn add_attributes(&self, html: &mut VNode) {
        if let VNode::VTag(tag) = html {
            let attrs = self
                .attributes
                .iter()
                .map(|(key, value)| (key.to_string(), (*value).to_owned()))
                .collect();
            tag.add_attributes(attrs);
        }
    }

    pub fn calculate_classes<C: Into<Classes>>(&self, prefix: C) -> Classes {
        let BootstrapProps {
            class,
            borders,
            margins,
            paddings,
            ..
        } = self;
        let classes = prefix.into();
        let classes = classes.extend((*class).to_owned());
        let classes = classes.extend(into_classnames(borders));
        let classes = classes.extend(into_classnames(margins));
        classes.extend(into_classnames(paddings))
    }
}

fn into_classnames<C: IntoBsClass>(c: &[&C]) -> Classes {
    c.iter().fold(Classes::new(), |mut cs, c| {
        cs.push(&c.as_classname());
        cs
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_aria_label_override() {
        let props = Props {
            aria_label: Some("override".into()),
            ..Props::default()
        };

        let expected = html! {
            <div class="" aria-label="override" />
        };

        assert_eq!(
            expected,
            crate::render::render_with_prefix(&props, "", html! { <div aria-label="test" /> })
        );
    }
}
