use super::home::ActiveSection;

#[derive(Clone, PartialEq)]
pub struct AnimationState {
    pub shaking: bool,
    pub target_section: ActiveSection,
}

impl Default for AnimationState {
    fn default() -> Self {
        Self {
            shaking: false,
            target_section: ActiveSection::Profile,
        }
    }
}
