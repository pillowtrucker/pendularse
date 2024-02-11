use nannou::prelude::*;
#[derive(Debug)]
pub(crate) struct SingleRigidPendulumAnal {
    pub(crate) window: window::Id,
}

pub(crate) fn model(app: &App) -> SingleRigidPendulumAnal {
    let window = app
        .new_window()
        .view(view)
        .event(window_event)
        .build()
        .unwrap();
    SingleRigidPendulumAnal { window }
}

pub(crate) fn update(_app: &App, _model: &mut SingleRigidPendulumAnal, _update: Update) {}

pub(crate) fn view(app: &App, _model: &SingleRigidPendulumAnal, frame: Frame) {
    let draw = app.draw();
    draw.background().color(PLUM);
    draw.ellipse().color(STEELBLUE);
    draw.to_frame(app, &frame).unwrap();
}
pub(crate) fn window_event(app: &App, model: &mut SingleRigidPendulumAnal, event: WindowEvent) {
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
