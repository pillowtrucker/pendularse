mod drpa;
mod drpn;
mod msrpa;
mod nrpn;
mod srpa;
mod sspa;

use std::f32::consts::PI;
pub const pxpm: f32 = 100.;
use clap::Parser;
pub type Time = f32;
pub const Π: f32 = PI;
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(
        short,
        long,
        default_value = "./inochi2d-models/arrows-doublespringpendulum.inp"
    )]
    puppet_path: String,
    #[arg(short, long, default_value_t = 0)]
    model_type: u8,
}

fn main() {
    let args = Args::parse();
    match args.model_type {
        0 => {
            nannou::app(srpa::model).update(srpa::update).run();
        }
        1 => {
            nannou::app(drpa::model).update(drpa::update).run();
        }
        3 => {
            nannou::app(drpn::model).update(drpn::update).run();
        }
        5 => {
            nannou::app(msrpa::model).update(msrpa::update).run();
        }
        6 => {
            nannou::app(sspa::model).update(sspa::update).run();
        }
        _ => {}
    };
}

/*
struct Model {
    _window: window::Id,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).event(window_event).build().unwrap();
    Model { _window }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(PLUM);
    draw.ellipse().color(STEELBLUE);
    draw.to_frame(app, &frame).unwrap();
}
fn window_event(app: &App, model: &mut Model, event: WindowEvent) {
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
*/
