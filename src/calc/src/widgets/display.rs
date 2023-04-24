//! The display UI part of the calculator

use crate::CalcState;
use druid::widget::{Align, Container, Flex, ViewSwitcher, TextBox, Axis};
use druid::{theme, Color, Env, UnitPoint, WidgetExt, FontDescriptor, FontFamily, Point, Rect, RenderContext};
use druid::{widget::Label, Widget};
use math::number::Radix;

pub const ACTIVE_RADIX_COLOR: Color = Color::GREEN;
pub const ACTIVE_TRIG_UNITS_COLOR: Color = Color::YELLOW;
pub const ERROR_MSG_COLOR: Color = Color::RED;

/// Widget displaying, **status of calculator**,
/// **current expression** and the **computed result**.
pub struct DisplayUI;
impl DisplayUI {
    /// Create the DisplayUI widget.
    pub fn build_ui() -> impl Widget<CalcState> {
        get_display()
    }
}

struct Display {
    lbl: Label<CalcState>,
    font: FontDescriptor,
    offset: f64,
}

impl Display {
    const OVERFLOW_RESERVE: f64 = 10.0;

    fn new(str: &str) -> Self {
        let font  = FontDescriptor::new(FontFamily::SYSTEM_UI)
            .with_size(30.0);
        let lbl = Label::new(str)
            .with_font(font.clone());
        Self { lbl, font, offset: 0.0 }
    }
}

impl Widget<CalcState> for Display {
    fn event(&mut self, ctx: &mut druid::EventCtx, event: &druid::Event, data: &mut CalcState, env: &Env) {
        self.lbl.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut druid::LifeCycleCtx, event: &druid::LifeCycle, data: &CalcState, env: &Env) {
        self.lbl.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut druid::UpdateCtx, old_data: &CalcState, data: &CalcState, env: &Env) {
        self.lbl.update(ctx, old_data, data, env);
    }

    fn layout(&mut self, ctx: &mut druid::LayoutCtx, bc: &druid::BoxConstraints, data: &CalcState, env: &Env) -> druid::Size {
        let inf = druid::BoxConstraints::new(
            druid::Size { width: 0.0, height: 0.0 },
            druid::Size { width: 100000000.0, height: 1000.0 }
        );
        let lbl_width = self.lbl.compute_max_intrinsic(Axis::Horizontal, ctx, &inf, data, env);
        let max_width = bc.max().width;

        if lbl_width > max_width - Display::OVERFLOW_RESERVE {
            self.offset = max_width - Display::OVERFLOW_RESERVE - lbl_width;
        }

        self.lbl.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, _data: &CalcState, _env: &Env) {
        self.lbl.draw_at(ctx, Point::new(self.offset, 0.0));
    }
}

#[rustfmt::skip]
fn get_display() -> impl Widget<CalcState> {
    // With druid it is hard to dynamically change color based on CalcState data state.
    // To solve this we change the disabled color and use the `disabled_if` method, 
    // which atleast gives us two possible states. See the status_row below.
    let gen_env = |color: Color| {
        move |env: &mut Env, _data: &CalcState| env.set(theme::DISABLED_TEXT_COLOR, color)
    };
    let radix_env = gen_env(ACTIVE_RADIX_COLOR);
    let tuni_env = gen_env(ACTIVE_TRIG_UNITS_COLOR);
    let radix_eq = |radix: Radix| move |data: &CalcState, _: &Env| data.radix == radix;
    let tuni_eq = |units: bool| move |data: &CalcState, _: &Env| data.degrees == units;

    let status_row = Align::left(
        Flex::row()
            .with_flex_child(Label::new("Dec").with_text_size(10.0).disabled_if(radix_eq(Radix::Dec)).env_scope(radix_env), 1.0)
            .with_flex_child(Label::new("Hex").with_text_size(10.0).disabled_if(radix_eq(Radix::Hex)).env_scope(radix_env), 1.0)
            .with_flex_child(Label::new("Oct").with_text_size(10.0).disabled_if(radix_eq(Radix::Oct)).env_scope(radix_env), 1.0)
            .with_flex_child(Label::new("Bin").with_text_size(10.0).disabled_if(radix_eq(Radix::Bin)).env_scope(radix_env), 1.0)
            .with_flex_spacer(1.0)
            .with_flex_child(Label::new("Deg").with_text_size(10.0).disabled_if(tuni_eq(true)).env_scope(tuni_env), 1.0)
            .with_flex_child(Label::new("Rad").with_text_size(10.0).disabled_if(tuni_eq(false)).env_scope(tuni_env), 1.0),
    );

    let disp = ViewSwitcher::new(
        move |data: &CalcState, _| data.expr_man.get_display_str(true),
        move |_selector, data, _| {
            let d = Display::new(&data.expr_man.get_display_str(true));

            Box::new(d)
        }
    );

    let expr_row = Align::left(
        disp
    );

    let result_row = Align::right(
        Label::new(|data: &CalcState, _env: &_| data.result.clone())
            .with_text_size(28.0)
            .disabled_if(|data: &CalcState, _| data.result_is_err)
            .env_scope(gen_env(ERROR_MSG_COLOR))
            .lens(CalcState::all),
    );

    Container::new(
        Flex::column()
            .with_flex_child(status_row, 1.0)
            .with_flex_child(expr_row, 1.0)
            .with_flex_child(result_row, 2.0)
            .align_vertical(UnitPoint::TOP)
            .padding((10.0, 0.0)),
    )
    .border(Color::GRAY, 2.0)
    .rounded(5.0)
}
