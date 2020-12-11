use druid::widget::{CrossAxisAlignment, Flex, MainAxisAlignment};
use druid::{AppLauncher, Widget, WidgetExt, WindowDesc};
use druid_material::button::Button;
use druid_material::checkbox::Checkbox;
use druid_material::radio::Radio;
use druid_material::switch::Switch;


fn main() {
    let window = WindowDesc::new(ui);
    AppLauncher::with_window(window).configure_env(|env, _| {
        druid_material::config_env(env);
    }).launch(true);
}

fn ui() -> impl Widget<bool> {
    Flex::column()
        .with_child(Radio::new("Some Checkbox", true))
        .with_child(Radio::new("Some Radio", false))
        .with_spacer(20.0)
        .with_child(Button::new("Btn"))
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .padding(40.0)
}