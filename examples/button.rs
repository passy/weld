#[macro_use]
extern crate yoga;
extern crate weld;
extern crate pretty_env_logger;
extern crate webrender;
extern crate rand;

use weld::application::Application;
use weld::model::*;
use weld::window::Interaction;
use weld::layout::{FlexDirection, Percent, Point, Wrap};
use weld::layout::FlexStyle::*;
use weld::layout::Align::*;
use webrender::api::*;
use rand::{random, Closed01};

#[derive(Debug)]
struct Container {}

impl Renderer for Container {
    fn id(&self) -> &'static str { "Container" }
    fn render(&self, context: &mut RenderContext) {
        let bounds = context.bounds();
        context.push(RenderElement::Rect(bounds, ColorF::new(1.0, 0.0, 0.0, 1.0)));
        context.next();
    }
}

fn container() -> Component {
    Component::new(Container {})
}

#[derive(Debug)]
struct Button {
    color: ColorF,
}

impl Renderer for Button {
    fn id(&self) -> &'static str { "Button" }
    fn render(&self, context: &mut RenderContext) {
        let bounds = context.bounds();
        context.push(RenderElement::Rect(bounds, self.color));
        context.next();
    }
}

fn button(color: &ColorF) -> Component {
    Component::new(Button {
        color: color.clone()
    })
}

#[derive(Clone, Debug)]
struct MyAppState {
    button_width: i32,
    button_color: ColorF,
}

impl State for MyAppState {
    fn build(&self) -> Component {
        container()
            .styles(make_styles!(
                Width(100 %),
                Height(100 %),
                FlexDirection(FlexDirection::Row),
                Padding(25 pt),
                AlignItems(FlexStart),
                FlexWrap(Wrap::Wrap)
            ))
            .child(
                button(&self.button_color)
                    .styles(make_styles!(
                        Width(self.button_width.point()),
                        Height(32 pt)
                    ))
                    .name("button")
                    .on(Box::new(|state: Self, event| {
                        match *event {
                            Interaction::Pressed => {
                                println!("pressed!");
                                Ok(Self {
                                    button_width: state.button_width + 5,
                                    ..state
                                })
                            }
                            Interaction::Released => {
                                println!("released!");

                                let button_color = ColorF::new(random::<Closed01<f32>>().0, random::<Closed01<f32>>().0, random::<Closed01<f32>>().0, 1.0);
                                Ok(Self {
                                    button_color,
                                    ..state
                                })
                            }
                        }
                    }))
            )
    }
}

fn main() {
    pretty_env_logger::init().unwrap();

    let app = Application::new("Demo", MyAppState {
        button_width: 100,
        button_color: ColorF::new(0.0, 0.0, 1.0, 1.0),
    });

    app.run();
}
