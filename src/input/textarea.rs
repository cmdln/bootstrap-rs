use crate::prelude::*;
use yew::prelude::*;

pub struct TextArea {
    link: ComponentLink<Self>,
    state: String,
    props: Props,
}

#[derive(Debug)]
pub struct InputChange(ChangeData);

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub name: String,
    #[prop_or_default]
    pub id: String,
    pub on_change: Callback<String>,
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
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub style: String,
    #[prop_or_default]
    pub value: String,
}

impl Component for TextArea {
    type Message = InputChange;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = props.value.clone();
        Self { props, state, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        if let InputChange(ChangeData::Value(value)) = msg {
            self.state = value.clone();
            self.props.on_change.emit(value);
            true
        } else {
            false
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let should_render = render_on_change(&mut self.props, props);
        if should_render {
            self.state = self.props.value.clone();
        }
        should_render
    }

    fn view(&self) -> Html {
        html! {
            <textarea
                name=&self.props.name
                id=&self.props.id
                class=calculate_classes("form-control", (&self.props).into())
                onchange=self.link.callback(|evt| InputChange(evt))
            >
                { &self.state }
            </textarea>
        }
    }
}

impl<'a> From<&'a Props> for BootstrapProps<'a> {
    fn from(props: &Props) -> BootstrapProps {
        let class = &props.class;
        let borders = collect_bs(&props.border, &props.borders);
        let margins = collect_bs(&props.margin, &props.margins);
        let paddings = collect_bs(&props.padding, &props.paddings);
        BootstrapProps {
            class,
            borders,
            margins,
            paddings,
        }
    }
}
