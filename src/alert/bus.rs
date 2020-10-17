use crate::prelude::*;
use std::collections::HashSet;
use yew::worker::{Agent, AgentLink, Context, HandlerId};

pub struct Bus {
    link: AgentLink<Bus>,
    subscribers: HashSet<HandlerId>,
}

pub enum Request {
    Primary(String),
    Secondary(String),
    Success(String),
    Danger(String),
    Warning(String),
    Info(String),
    Clear,
}

impl Into<Option<(Color, String)>> for &Request {
    fn into(self) -> Option<(Color, String)> {
        use Request::*;
        match self {
            Primary(alert) => Some((Color::Primary, alert.clone())),
            Secondary(alert) => Some((Color::Secondary, alert.clone())),
            Success(alert) => Some((Color::Success, alert.clone())),
            Danger(alert) => Some((Color::Danger, alert.clone())),
            Warning(alert) => Some((Color::Warning, alert.clone())),
            Info(alert) => Some((Color::Info, alert.clone())),
            Clear => None,
        }
    }
}

impl Agent for Bus {
    type Reach = Context<Self>;
    type Message = ();
    type Input = Request;
    type Output = Option<(Color, String)>;

    fn create(link: AgentLink<Self>) -> Self {
        let subscribers = HashSet::new();
        Self { link, subscribers }
    }

    fn update(&mut self, _: Self::Message) {}

    fn handle_input(&mut self, input: Self::Input, _id: HandlerId) {
        for sub in self.subscribers.iter() {
            self.link.respond(*sub, (&input).into());
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}
