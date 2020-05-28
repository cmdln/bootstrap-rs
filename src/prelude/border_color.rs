#[derive(Clone, PartialEq)]
pub enum BorderColor {
    Primary,
    Secondary,
    Unset,
}

impl Default for BorderColor {
    fn default() -> Self {
        BorderColor::Unset
    }
}

impl super::BootstrapClass for BorderColor {
    fn as_classname(&self) -> String {
        match self {
            Self::Primary => "border-primary".into(),
            Self::Secondary => "border-secondary".into(),
            Self::Unset => "".into(),
        }
    }
}
