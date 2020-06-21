#[derive(Clone, PartialEq)]
pub enum Color {
    Primary,
    Secondary,
    Success,
    Danger,
    Warning,
    Info,
    Light,
    Dark,
    White,
    Unset,
}

impl Default for Color {
    fn default() -> Self {
        Self::Unset
    }
}

impl Color {
    pub fn with_prefix<S: AsRef<str>>(&self, prefix: S) -> String {
        match self {
            Self::Primary => format!("{}-primary", prefix.as_ref()),
            Self::Secondary => format!("{}-secondary", prefix.as_ref()),
            Self::Success => format!("{}-success", prefix.as_ref()),
            Self::Danger => format!("{}-danger", prefix.as_ref()),
            Self::Warning => format!("{}-warning", prefix.as_ref()),
            Self::Info => format!("{}-info", prefix.as_ref()),
            Self::Light => format!("{}-light", prefix.as_ref()),
            Self::Dark => format!("{}-dark", prefix.as_ref()),
            Self::White => format!("{}-white", prefix.as_ref()),
            Self::Unset => "".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_prefix() {
        assert_eq!(Color::Primary.with_prefix("alert"), "alert-primary");
    }
}
