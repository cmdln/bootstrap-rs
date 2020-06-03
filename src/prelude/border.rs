use super::{Color, Edge};
#[derive(Clone, PartialEq)]
pub struct Border(pub Edge, pub Color);

impl super::BootstrapClass for Border {
    fn as_classname(&self) -> String {
        let edge = match self.0 {
            Edge::All => "border".to_owned(),
            _ => self.0.with_prefix("border"),
        };
        format!("{} {}", edge, self.1.with_prefix("border"))
    }
}
