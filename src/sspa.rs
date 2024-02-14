use crate::{pxpm, Time, Π};
use nannou::prelude::*;
use ode_solvers::{Rk4, System, Vector2};

pub type PendulumState = Vector2<f32>;

pub fn θ_mut(s: &mut PendulumState) -> &mut f32 {
    &mut s[0]
}
pub fn ω_mut(s: &mut PendulumState) -> &mut f32 {
    &mut s[1]
}
pub fn θ(s: &PendulumState) -> f32 {
    s[0]
}
pub fn ω(s: &PendulumState) -> f32 {
    s[1]
}

#[derive(Debug)]
pub(crate) struct PendulumSystem {
    pub(crate) g: f32,
    pub(crate) r: f32,
}

impl Default for PendulumSystem {
    fn default() -> Self {
        Self { g: 9.81, r: 0.3 }
    }
}
#[derive(Debug)]
pub(crate) struct SingleSpringPendulumAnal {
    pub(crate) window: Option<window::Id>,
    pub(crate) pendulum_system: PendulumSystem,
    pub(crate) pendulum_state: PendulumState,
}
impl System<Time, PendulumState> for &PendulumSystem {
    fn system(&self, x: Time, y: &PendulumState, dy: &mut PendulumState) {
        *θ_mut(dy) = ω(y);
        *ω_mut(dy) = -(self.g / self.r) * θ(y).sin()
    }
}
pub(crate) fn model(app: &App) -> SingleSpringPendulumAnal {
    let window = app.new_window().view(view).event(window_event).build().ok();
    let pendulum_state = Vector2::new(Π / 2., 0.);
    let pendulum_system = Default::default();
    SingleSpringPendulumAnal {
        window,
        pendulum_state,
        pendulum_system,
    }
}

pub(crate) fn update(_app: &App, model: &mut SingleSpringPendulumAnal, update: Update) {
    let mut stepper = Rk4::new(
        &model.pendulum_system,
        (update.since_start - update.since_last).as_secs_f32(),
        model.pendulum_state,
        update.since_start.as_secs_f32(),
        update.since_last.as_secs_f32(),
    );
    let Ok(_) = stepper.integrate() else { return };
    let (_, y_out) = stepper.results().get();
    let res = y_out.last().unwrap();
    *θ_mut(&mut model.pendulum_state) = θ(res);
    *ω_mut(&mut model.pendulum_state) = ω(res);
}

pub(crate) fn view(app: &App, model: &SingleSpringPendulumAnal, frame: Frame) {
    let draw = app.draw();
    let start = frame.rect().mid_top();
    let end = start + vec2(0., -model.pendulum_system.r * pxpm).rotate(θ(&model.pendulum_state));

    draw.line().stroke_weight(5.).start(start).end(end);

    draw.background().color(PLUM);
    draw.ellipse().xy(end).color(STEELBLUE);
    draw.to_frame(app, &frame).unwrap();
}
pub(crate) fn window_event(app: &App, model: &mut SingleSpringPendulumAnal, event: WindowEvent) {
    match event {
        KeyPressed(_key) => {}
        KeyReleased(_key) => {}
        ReceivedCharacter(_char) => {}
        MouseMoved(_pos) => {}
        MousePressed(_button) => {}
        MouseReleased(_button) => {}
        MouseEntered => {}
        MouseExited => {}
        MouseWheel(_amount, _phase) => {}
        Moved(_pos) => {}
        Resized(_size) => {}
        Touch(_touch) => {}
        TouchPressure(_pressure) => {}
        HoveredFile(_path) => {}
        DroppedFile(_path) => {}
        HoveredFileCancelled => {}
        Focused => {}
        Unfocused => {}
        Closed => {}
    }
}
