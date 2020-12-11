// Copyright 2019 The Druid Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! A checkbox widget.

use std::time::Duration;

use druid::kurbo::{BezPath, Circle, Size};
use druid::piet::{LineCap, LineJoin, LinearGradient, RenderContext, StrokeStyle, UnitPoint};
use druid::{Rect, TimerToken, theme};
use druid::widget::{prelude::*, Label, LabelText};

const EXTRA_PADDING: f64 = 25.0;
/// A checkbox that toggles a `bool`.
pub struct Checkbox {
    child_label: Label<bool>,
    expanding_time: Option<TimerToken>,
    expanding_radius: Option<f64>,
}

impl Checkbox {
    /// Create a new `Checkbox` with a text label.
    pub fn new(text: impl Into<LabelText<bool>>) -> Checkbox {
        Checkbox {
            child_label: Label::new(text).with_text_size(18.0),
            expanding_time: None,
            expanding_radius: None,
        }
    }

    /// Update the text label.
    pub fn set_text(&mut self, label: impl Into<LabelText<bool>>) {
        self.child_label.set_text(label);
    }
}

impl Widget<bool> for Checkbox {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut bool, env: &Env) {
        match event {
            Event::MouseDown(_) => {
                ctx.set_active(true);
                ctx.request_paint();
            }
            Event::MouseUp(_) => {
                if ctx.is_active() {
                    ctx.set_active(false);
                    if ctx.is_hot() {
                        if *data {
                            *data = false;
                        } else {
                            *data = true;
                        }
                        ctx.request_anim_frame();
                        let timer = ctx.request_timer(Duration::from_millis(200));
                        self.expanding_time = Some(timer);
                        self.expanding_radius = Some(10.0);
                    }
                    ctx.request_paint();
                }
            }
            Event::AnimFrame(f) if self.expanding_time.is_some() => {
                let fraction_passed = (*f as f64) / 1.0e6 / 200.0;
                let to_expand_radii = (env.get(theme::BASIC_WIDGET_HEIGHT) + EXTRA_PADDING) / 2.0;
                *self.expanding_radius.as_mut().unwrap() += fraction_passed * (to_expand_radii - 10.0);
                ctx.request_anim_frame();
                ctx.request_paint();
            }
            Event::Timer(t) if self.expanding_time == Some(*t) => {
                self.expanding_time = None;
                self.expanding_radius = None;
                ctx.request_paint();
            },
            _ => (),
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &bool, env: &Env) {
        self.child_label.lifecycle(ctx, event, data, env);
        if let LifeCycle::HotChanged(_) = event {
            ctx.request_paint();
        }
        if let LifeCycle::WidgetAdded = event {
            ctx.register_for_focus();
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &bool, data: &bool, env: &Env) {
        self.child_label.update(ctx, old_data, data, env);
        ctx.request_paint();
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &bool, env: &Env) -> Size {
        bc.debug_check("Checkbox");
        let check_size = env.get(theme::BASIC_WIDGET_HEIGHT) + EXTRA_PADDING;
        let label_size = self.child_label.layout(ctx, &bc, data, env);

        let desired_size = Size::new(
            check_size + label_size.width,
            check_size.max(label_size.height),
        );
        let our_size = bc.constrain(desired_size);
        let baseline = self.child_label.baseline_offset() + (our_size.height - label_size.height);
        ctx.set_baseline_offset(baseline);
        our_size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &bool, env: &Env) {
        let size = env.get(theme::BASIC_WIDGET_HEIGHT);
        let border_width = 2.;
        let radius = (size + EXTRA_PADDING) / 2.;
        let border_color = if *data {
            env.get(crate::color::PRIMARY)
        } else {
            env.get(theme::BORDER_LIGHT)
        };

        // circles
        if let Some(r) = self.expanding_radius {
            let circle = Circle::new((radius, radius), r);
            ctx.fill(circle, &border_color.clone().with_alpha(0.3));
        }
        else if ctx.is_focused() {
            let circle = Circle::new((radius, radius), radius);
            ctx.fill(circle, &border_color.clone());
        } else if ctx.is_hot() {
            let circle = Circle::new((radius, radius), radius);
            ctx.fill(circle, &border_color.clone().with_alpha(0.05));
        }

        // simple
        let rect = Rect::from_origin_size((EXTRA_PADDING / 2., EXTRA_PADDING / 2.), (size, size)).to_rounded_rect(2.0);

        if *data {
            ctx.fill(rect, &env.get(crate::color::PRIMARY));
        };
        

        
        ctx.stroke(rect, &border_color, border_width);

        if *data {
            // Paint the checkmark
            let mut path = BezPath::new();
            path.move_to((4.0 + EXTRA_PADDING / 2.0, 9.0+ EXTRA_PADDING / 2.));
            path.line_to((8.0 + EXTRA_PADDING / 2.0, 13.0+ EXTRA_PADDING / 2.));
            path.line_to((14.0 + EXTRA_PADDING / 2.0, 5.0+ EXTRA_PADDING / 2.));

            let style = StrokeStyle::new()
                .line_cap(LineCap::Round)
                .line_join(LineJoin::Round);

            ctx.stroke_styled(path, &env.get(crate::color::ON_PRIMARY), 2., &style);
        }

        

        // Paint the text label
        self.child_label.draw_at(ctx, (size + EXTRA_PADDING, -3.0 + EXTRA_PADDING / 2.));
    }
}
