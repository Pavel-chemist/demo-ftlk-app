use fltk::{app, prelude::*, *};

fn main() {
    let application = app::App::default();
    let mut wind = window::Window::new(256, 128, 512, 256, "My first ftlk Window");

    let mut frame = frame::Frame::new(10, 10, 200, 50, "Click on button below:");
    let mut butt = button::Button::new(10, 80, 200, 50, "Click me!");

    butt.set_callback(move |_| frame.set_label("You've clicked on button"));

    wind.end();
    wind.show();

    application.run().unwrap();
}