use crate::{prelude::*, props::*, render};
use std::collections::HashMap;
use yew::{html::Children, prelude::*};

pub struct BreadcrumbItem {
    props: Props,
}

#[derive(Properties, Clone, PartialEq, Default)]
pub struct Props {
    // component specific
    pub active: bool,

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
        let (prefix, html) = if self.props.active {
            (
                vec!["breadcrumb-item", "active"],
                html! {
                    <li class=self.classes() aria-current="page">
                        { self.props.children.render() }
                    </li>
                },
            )
        } else {
            (
                vec!["breadcrumb-item"],
                html! {
                    <li class=self.classes()>
                        { self.props.children.render() }
                    </li>
                },
            )
        };
        render::render_with_prefix(&self.props, prefix, html)
    }
}

impl BreadcrumbItem {
    fn classes(&self) -> Classes {
        let props: BootstrapProps<'_> = (&self.props).into();
        let mut classes = props.calculate_classes("breadcrumb-item");
        if self.props.active {
            classes.push("active")
        }
        classes
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_active_prop() {
        let item = BreadcrumbItem {
            props: Props {
                active: true,
                id: Some("test".into()),
                margin: Some(Margin(Edge::All, 3)),
                padding: Some(Padding(Edge::Top, 3)),
                ..Props::default()
            },
        };
        let expected = html! {
            <li
                class="breadcrumb-item active m-3 pt-3"
                aria-current="page"
                id="test"
            >
                <></>
            </li>
        };
        assert_eq!(expected, item.view());
    }
}
