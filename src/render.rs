use crate::props::*;
use yew::{html::Children, prelude::*, virtual_dom::VNode};

pub(crate) fn render_with_prefix<'a, B: Into<BootstrapProps<'a>>, C: Into<Classes>>(
    props: B,
    prefix: C,
    mut node: VNode,
) -> VNode {
    let props = props.into();
    if let VNode::VTag(tag) = &mut node {
        let classes = &props.calculate_classes(prefix);
        if !classes.is_empty() {
            tag.add_attribute("class", classes);
        }
    }
    props.add_attributes(&mut node);
    node
}

pub(crate) fn div(children: &Children) -> Html {
    html! {
        <div>
            { children.render() }
        </div>
    }
}

pub(crate) fn p(children: &Children) -> Html {
    html! {
        <p>
            { children.render() }
        </p>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{prelude::*, props::Props};

    #[test]
    fn test_multiple_prefixes() {
        let props = Props {
            margin: Some(Margin(Edge::All, 3)),
            ..Props::default()
        };
        let comp = render_with_prefix(&props, vec!["first", "second"], html! { <div/> });
        let expected = html! {
            <div class="first second m-3"/>
        };

        assert_eq!(expected, comp);
    }
}
