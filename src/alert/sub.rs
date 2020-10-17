use super::{Alert, Bus};
use crate::prelude::*;
use yew::prelude::*;

pub struct AlertSubscriber {
    link: ComponentLink<Self>,
    alert: Option<(Color, String)>,
    _producer: Box<dyn Bridge<Bus>>,
}

impl Component for AlertSubscriber {
    type Message = Option<(Color, String)>;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let _producer = Bus::bridge(link.callback(|msg| msg));
        let alert = None;
        Self {
            link,
            alert,
            _producer,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        if self.alert == msg {
            false
        } else {
            self.alert = msg;
            true
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        if let Some((color, message)) = self.alert.as_ref() {
            html! {
                <Alert
                    on_close=self.link.callback(|_| None)
                    color=color
                >
                { message }
                </Alert>
            }
        } else {
            html! {}
        }
    }
}
