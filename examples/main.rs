use druid::{AppLauncher, Widget, WidgetExt, WindowDesc};
use druid_material::checkbox::Checkbox;


fn main() {
    let window = WindowDesc::new(ui);
    AppLauncher::with_window(window).configure_env(|env, _| {
        druid_material::config_env(env);
    }).launch(true);
}

fn ui() -> impl Widget<bool> {
    Checkbox::new("Some Checkbox").center()
}