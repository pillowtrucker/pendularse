use crate::{pxpm, Time, Π};
use nannou::prelude::*;
use ode_solvers::{Rk4, System, Vector4};

pub type PendulumState = Vector4<f32>;

pub fn bob_x_mut(s: &mut PendulumState) -> &mut f32 {
    &mut s[0]
}
pub fn bob_y_mut(s: &mut PendulumState) -> &mut f32 {
    &mut s[1]
}
pub fn bob_vx_mut(s: &mut PendulumState) -> &mut f32 {
    &mut s[2]
}
pub fn bob_vy_mut(s: &mut PendulumState) -> &mut f32 {
    &mut s[3]
}

pub fn bob_x(s: &PendulumState) -> f32 {
    s[0]
}
pub fn bob_y(s: &PendulumState) -> f32 {
    s[1]
}
pub fn bob_vx(s: &PendulumState) -> f32 {
    s[2]
}
pub fn bob_vy(s: &PendulumState) -> f32 {
    s[3]
}

#[derive(Debug)]
pub(crate) struct PendulumSystem {
    pub(crate) g: f32,
    pub(crate) spring_rest_len: f32,
    pub(crate) spring_const: f32,
    pub(crate) damping: f32,
    pub(crate) bob_mass: f32,
    pub(crate) anchor_poz: Vec2,
}

impl Default for PendulumSystem {
    fn default() -> Self {
        Self {
            g: 9.81,
            spring_rest_len: 100.,
            spring_const: 4.,
            damping: 0.5,
            bob_mass: 1.,
            anchor_poz: Vec2::ZERO,
        }
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
        // fuck this shit. the explanation on the page is not even close to the typescript implementation
        *bob_x_mut(dy) = bob_vx(y);
        *bob_y_mut(dy) = bob_vy(y);
        let cur_bob = vec2(bob_x(y), bob_y(y));
        let cur_spring_len = cur_bob.distance(self.anchor_poz);
        let θ = cur_bob.angle_between(vec2(0., -1.));
        println!("{θ}");
        let cur_spring_displacement = cur_spring_len - self.spring_rest_len;
        *bob_vx_mut(dy) = (self.spring_const / self.bob_mass) * cur_spring_displacement * θ.sin()
            - (self.damping / self.bob_mass) * bob_vx(y);
        *bob_vy_mut(dy) = self.g
            - (self.spring_const / self.bob_mass) * cur_spring_displacement * θ.cos()
            - (self.damping / self.bob_mass) * bob_vy(y);
    }
    fn solout(&mut self, _x: Time, y: &PendulumState, dy: &PendulumState) -> bool {
        y.iter().any(|v| !v.is_normal()) || dy.iter().any(|dv| !dv.is_normal())
    }
}
pub(crate) fn model(app: &App) -> SingleSpringPendulumAnal {
    let mut pendulum_state = Vector4::<f32>::zeros();
    let bob = vec2(100., 100.);
    *bob_x_mut(&mut pendulum_state) = bob.x;
    *bob_y_mut(&mut pendulum_state) = bob.y;
    let pendulum_system = Default::default();
    let window = app.new_window().view(view).event(window_event).build().ok();

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
    //    println!("{res:?}");
    *bob_x_mut(&mut model.pendulum_state) = bob_x(res);
    *bob_y_mut(&mut model.pendulum_state) = bob_y(res);
    *bob_vx_mut(&mut model.pendulum_state) = bob_vx(res);
    *bob_vy_mut(&mut model.pendulum_state) = bob_vy(res);
}

pub(crate) fn view(app: &App, model: &SingleSpringPendulumAnal, frame: Frame) {
    let draw = app.draw();
    let start = model.pendulum_system.anchor_poz;
    let cur_bob = vec2(bob_x(&model.pendulum_state), bob_y(&model.pendulum_state));
    //    let cur_spring_len = cur_bob.distance(start);
    let end = cur_bob;

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
