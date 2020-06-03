use bootstrap_rs::{prelude::*, Breadcrumb, BreadcrumbItem, Container, Jumbotron};
use yew::{html::Children, prelude::*};

pub struct Example {
    props: Props,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
}

impl Component for Example {
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
            <Container>
                <Jumbotron>
                    <h1>{ "Example!" }</h1>
                </Jumbotron>
                <Breadcrumb
                    margin=Margin(Edge::Bottom, 3)
                >
                    <BreadcrumbItem active=false>
                        { "Grandparent" }
                    </BreadcrumbItem>
                    <BreadcrumbItem active=false>
                        { "Parent" }
                    </BreadcrumbItem>
                    <BreadcrumbItem active=true>
                        { "Active" }
                    </BreadcrumbItem>
                </Breadcrumb>
            </Container>
        }
    }
}
