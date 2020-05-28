#[derive(Clone, PartialEq)]
pub enum Border {
    All,
    Top,
    Right,
    Bottom,
    Left,
}

impl super::BootstrapClass for Border {
    fn as_classname(&self) -> String {
        match self {
            Self::All => "border".into(),
            Self::Top => "border-top".into(),
            Self::Right => "border-right".into(),
            Self::Bottom => "border-bottom".into(),
            Self::Left => "border-left".into(),
        }
    }
}
