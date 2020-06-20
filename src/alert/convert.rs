use super::Props;
use crate::prelude::*;

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
