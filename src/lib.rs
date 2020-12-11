use druid::{Color, Env, FontDescriptor};

pub mod checkbox;
pub mod theme;
use theme::color;

pub fn config_env(env: &mut Env) {
    env.set(color::PRIMARY, Color::Rgba32(0x6200EEFF));
    env.set(color::PRIMARY_VARIANT, Color::Rgba32(0x3700B3FF));
    env.set(color::SECONDARY, Color::Rgba32(0x03DAC6FF));
    env.set(color::SECONDARY_VARIANT, Color::Rgba32(0x018786FF));
    env.set(color::BACKGROUND, Color::Rgba32(0xFFFFFFFF));
    env.set(color::SURFACE, Color::Rgba32(0xFFFFFFFF));
    env.set(color::ON_PRIMARY, Color::Rgba32(0xFFFFFFFF));
    env.set(color::ON_SECONDARY, Color::Rgba32(0x000000FF));
    env.set(color::ON_BACKGROUND, Color::Rgba32(0x000000FF));
    env.set(color::ON_SURFACE, Color::Rgba32(0x000000FF));
    env.set(color::ON_ERROR, Color::Rgba32(0xFFFFFFFF));
    env.set(druid::theme::BORDER_DARK, Color::Rgba32(0x222222FF));
    env.set(druid::theme::BORDER_LIGHT, Color::Rgba32(0x444444FF));
    env.set(druid::theme::WINDOW_BACKGROUND_COLOR, Color::Rgba32(0xFFFFFFFF));
    env.set(druid::theme::BACKGROUND_DARK, Color::Rgba32(0xFFFFFFFF));
    env.set(druid::theme::LABEL_COLOR, Color::Rgba32(0x000000FF));

}