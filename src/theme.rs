use druid::{Color, Key};

pub mod color {
    use super::*;
    pub const PRIMARY: Key<Color> = Key::new("druid.material.color.primary");
    pub const PRIMARY_VARIANT: Key<Color> = Key::new("druid.material.color.primary-variant-color");
    pub const SECONDARY: Key<Color> = Key::new("druid.material.color.secondary");
    pub const SECONDARY_VARIANT: Key<Color> = Key::new("druid.material.color.secondary-variant");
    pub const BACKGROUND: Key<Color> = Key::new("druid.material.color.background");
    pub const SURFACE: Key<Color> = Key::new("druid.material.color.surface");
    pub const ERROR: Key<Color> = Key::new("druid.material.color.error");
    pub const ON_PRIMARY: Key<Color> = Key::new("druid.material.color.on-primary");
    pub const ON_SECONDARY: Key<Color> = Key::new("druid.material.color.on-secondary");
    pub const ON_BACKGROUND: Key<Color> = Key::new("druid.material.color.on-background");
    pub const ON_SURFACE: Key<Color> = Key::new("druid.material.color.on-surface");
    pub const ON_ERROR: Key<Color> = Key::new("druid.material.color.on-error");
}
    
