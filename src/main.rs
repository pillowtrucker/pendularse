use clap::Parser;

use into_variant::{IntoVariant, VariantFrom};
use nannou::{
    app::{ModelFn, UpdateFn},
    prelude::*,
    window::{EventFn, ViewFn},
};
use pendularse_macros::NannouModel;
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
#[derive(NannouModel, VariantFrom)]
enum NannouModels {
    SingleRigidPendulumModel(SingleRigidPendulumModel),
    DoubleRigidPendulumModel(DoubleRigidPendulumModel),
}

trait NannouModel<T> {
    fn model(&self) -> ModelFn<T>;
    fn update(&self) -> UpdateFn<T>;
    fn view(model: &NannouModels) -> ViewFn<T>;
    fn window_event(model: &NannouModels) -> EventFn<T>;
}

fn main() {
    let args = Args::parse();
    let the_model = match args.model_type {
        0 => NannouModels::SingleRigidPendulumModel(SingleRigidPendulumModel::default()),
        1 => NannouModels::DoubleRigidPendulumModel(DoubleRigidPendulumModel::default()),
        _ => panic!("unsupported model"),
    };
    nannou::app(the_model.model())
        .update(the_model.update())
        .run();
}
#[derive(Default, Debug)]
struct DoubleRigidPendulumModel {}
impl NannouModel<NannouModels> for DoubleRigidPendulumModel {
    fn model(&self) -> ModelFn<NannouModels> {
        todo!()
    }

    fn update(&self) -> UpdateFn<NannouModels> {
        todo!()
    }

    fn view(_model: &NannouModels) -> ViewFn<NannouModels> {
        todo!()
    }

    fn window_event(_model: &NannouModels) -> EventFn<NannouModels> {
        todo!()
    }
}
#[derive(Default, Debug)]
struct SingleRigidPendulumModel {
    window: Option<window::Id>,
}
impl NannouModel<NannouModels> for SingleRigidPendulumModel {
    fn model(&self) -> ModelFn<NannouModels> {
        |app: &App| -> NannouModels {
            let window = app
                .new_window()
                .view(SingleRigidPendulumModel::view(
                    &SingleRigidPendulumModel::default().into_variant(),
                ))
                .event(SingleRigidPendulumModel::window_event(
                    &SingleRigidPendulumModel::default().into_variant(),
                ))
                .build()
                .unwrap();
            SingleRigidPendulumModel {
                window: Some(window),
            }
            .into_variant()
        }
    }

    fn update(&self) -> UpdateFn<NannouModels> {
        |app: &App, model: &mut NannouModels, update: Update| {}
    }
    fn view(_model: &NannouModels) -> ViewFn<NannouModels> {
        |app: &App, model: &NannouModels, frame: Frame| {
            let draw = app.draw();
            draw.background().color(PLUM);
            draw.ellipse().color(STEELBLUE);
            draw.to_frame(app, &frame).unwrap();
        }
    }
    fn window_event(_model: &NannouModels) -> EventFn<NannouModels> {
        |app: &App, model: &mut NannouModels, event: WindowEvent| match event {
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
}
