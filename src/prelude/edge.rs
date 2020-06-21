use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, PartialEq)]
pub enum Edge {
    All,
    Top,
    Right,
    Bottom,
    Left,
    LeftAndRight,
    TopAndBottom,
}

impl Display for Edge {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        match self {
            Self::All => write!(fmt, ""),
            Self::Top => write!(fmt, "t"),
            Self::Bottom => write!(fmt, "b"),
            Self::Right => write!(fmt, "r"),
            Self::Left => write!(fmt, "l"),
            Self::LeftAndRight => write!(fmt, "x"),
            Self::TopAndBottom => write!(fmt, "y"),
        }
    }
}

impl Edge {
    fn suffix(&self) -> &str {
        match self {
            Self::Top => "-top",
            _ => "",
        }
    }

    pub(crate) fn with_prefix<S: AsRef<str>>(&self, prefix: S) -> String {
        match self {
            Self::All => prefix.as_ref().to_owned(),
            Self::LeftAndRight => format!(
                "{0}{1} {0}{2}",
                prefix.as_ref(),
                Self::Left.suffix(),
                Self::Right.suffix()
            ),
            Self::TopAndBottom => format!(
                "{0}{1} {0}{2}",
                prefix.as_ref(),
                Self::Top.suffix(),
                Self::Bottom.suffix()
            ),
            _ => format!("{}{}", prefix.as_ref(), self.suffix()),
        }
    }
}
