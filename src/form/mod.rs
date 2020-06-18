use crate::prelude::*;
use yew::prelude::*;

pub struct FormGroup {
    pub props: Props,
}

impl Component for FormGroup {
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
        html! {
            <div class=calculate_classes("form-group", (&self.props).into())>
            { self.props.children.render() }
            </div>
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let form_group = FormGroup {
            props: Props {
                margin: Some(Margin(Edge::All, 3)),
                padding: Some(Padding(Edge::Top, 3)),
                ..Props::default()
            },
        };
        let expected = html! {
            <div class="form-group m-3 pt-3">
                <></>
            </div>
        };
        assert_eq!(expected, form_group.view());
    }
}
