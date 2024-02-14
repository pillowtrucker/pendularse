use crate::{pxpm, Time, Π};
use nannou::prelude::*;
use ode_solvers::{Rk4, System, Vector2, Vector6};

pub type PendulumState = Vector6<f32>;

pub fn θ_mut(s: &mut PendulumState) -> &mut f32 {
    &mut s[0]
}
pub fn ω_mut(s: &mut PendulumState) -> &mut f32 {
    &mut s[1]
}
pub fn anchor_x_mut(s: &mut PendulumState) -> &mut f32 {
    &mut s[2]
}
pub fn anchor_y_mut(s: &mut PendulumState) -> &mut f32 {
    &mut s[3]
}
pub fn anchor_vx_mut(s: &mut PendulumState) -> &mut f32 {
    &mut s[4]
}
pub fn anchor_vy_mut(s: &mut PendulumState) -> &mut f32 {
    &mut s[5]
}
pub fn θ(s: &PendulumState) -> f32 {
    s[0]
}
pub fn ω(s: &PendulumState) -> f32 {
    s[1]
}
pub fn anchor_x(s: &PendulumState) -> f32 {
    s[2]
}
pub fn anchor_y(s: &PendulumState) -> f32 {
    s[3]
}
pub fn anchor_vx(s: &PendulumState) -> f32 {
    s[4]
}
pub fn anchor_vy(s: &PendulumState) -> f32 {
    s[5]
}
pub fn anchor_poz(s: &PendulumState) -> Vec2 {
    vec2(anchor_x(s), anchor_y(s))
}

#[derive(Debug)]
pub(crate) struct PendulumSystem {
    pub(crate) g: f32,
    pub(crate) r: f32,
    pub(crate) anchor_damping: f32,
    pub(crate) pendulum_damping: f32,
    pub(crate) mouse_poz: Vec2,
    pub(crate) mouse_stiffy: f32,
    pub(crate) pendulum_mass: f32,
}

impl Default for PendulumSystem {
    fn default() -> Self {
        Self {
            g: 10.,
            r: 1.,
            anchor_damping: 0.5,
            pendulum_damping: 0.5,
            mouse_poz: Vec2::ZERO,
            mouse_stiffy: 0.,
            pendulum_mass: 1.,
        }
    }
}
#[derive(Debug)]
pub(crate) struct MovableSingleRigidPendulumAnal {
    pub(crate) window: Option<window::Id>,
    pub(crate) pendulum_system: PendulumSystem,
    pub(crate) pendulum_state: PendulumState,
}
impl System<Time, PendulumState> for &PendulumSystem {
    fn system(&self, x: Time, y: &PendulumState, dy: &mut PendulumState) {
        *θ_mut(dy) = ω(y);

        *ω_mut(dy) = -(θ(y).cos() / self.r) * anchor_vx(dy)
            - (θ(y).sin() / self.r) * anchor_vy(dy)
            - (self.pendulum_damping / (self.pendulum_mass * (self.r).pow(2))) * ω(y)
            - (self.g / self.r) * θ(y).sin();

        *anchor_x_mut(dy) = anchor_vx(y);

        *anchor_vx_mut(dy) = -self.anchor_damping * anchor_vx(y)
            + self.mouse_stiffy * (self.mouse_poz.x - anchor_x(y)) / (pxpm * 0.1);
        *anchor_y_mut(dy) = anchor_vy(y);
        *anchor_vy_mut(dy) = -self.anchor_damping * anchor_vy(y)
            + self.mouse_stiffy * (self.mouse_poz.y - anchor_y(y)) / (pxpm * 0.1);
    }

    fn solout(&mut self, _x: Time, y: &PendulumState, dy: &PendulumState) -> bool {
        y.iter().any(|v| !v.is_normal()) || dy.iter().any(|dv| !dv.is_normal())
    }
}
pub(crate) fn model(app: &App) -> MovableSingleRigidPendulumAnal {
    let window = app.new_window().view(view).event(window_event).build().ok();
    let θ = Π / 2.;
    //    let ω = 0.;
    let mut pendulum_state = Vector6::zero();
    *θ_mut(&mut pendulum_state) = θ;
    //    *ω_mut(&mut pendulum_state) = ω;
    let pendulum_system = Default::default();
    MovableSingleRigidPendulumAnal {
        window,
        pendulum_state,
        pendulum_system,
    }
}

pub(crate) fn update(_app: &App, model: &mut MovableSingleRigidPendulumAnal, update: Update) {
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
    //    println!("{res:?}");
    //    println!("{:?}", update.since_last.as_secs_f32());
    *θ_mut(&mut model.pendulum_state) = θ(res);
    *ω_mut(&mut model.pendulum_state) = ω(res);
    *anchor_x_mut(&mut model.pendulum_state) = anchor_x(res);
    *anchor_y_mut(&mut model.pendulum_state) = anchor_y(res);
    //    *anchor_vx_mut(&mut model.pendulum_state) = anchor_vx(res);
    //    *anchor_vy_mut(&mut model.pendulum_state) = anchor_vy(res);
    *anchor_vx_mut(&mut model.pendulum_state) = anchor_vx(res);
    *anchor_vy_mut(&mut model.pendulum_state) = anchor_vy(res);
}

pub(crate) fn view(app: &App, model: &MovableSingleRigidPendulumAnal, frame: Frame) {
    let draw = app.draw();
    let start = anchor_poz(&model.pendulum_state);
    //let start = frame.rect().mid_top();
    let end = start + vec2(0., -model.pendulum_system.r * pxpm).rotate(θ(&model.pendulum_state));
    //let end = vec2(
    //    anchor_x(&model.pendulum_state)
    //        + model.pendulum_system.r * pxpm * θ(&model.pendulum_state).sin(),
    //    anchor_y(&model.pendulum_state)
    //        - model.pendulum_system.r * pxpm * θ(&model.pendulum_state).cos(),
    //);
    draw.line().stroke_weight(5.).start(start).end(end);

    draw.background().color(PLUM);
    draw.ellipse().radius(10.).xy(end).color(STEELBLUE);
    draw.to_frame(app, &frame).unwrap();
}
pub(crate) fn window_event(
    app: &App,
    model: &mut MovableSingleRigidPendulumAnal,
    event: WindowEvent,
) {
    match event {
        KeyPressed(_key) => {}
        KeyReleased(_key) => {}
        ReceivedCharacter(_char) => {}
        MouseMoved(pos) => model.pendulum_system.mouse_poz = pos,
        MousePressed(button) => {
            if let MouseButton::Left = button {
                model.pendulum_system.mouse_stiffy = 3.;
            }
        }
        MouseReleased(button) => {
            if let MouseButton::Left = button {
                model.pendulum_system.mouse_stiffy = 0.;
            }
        }
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
