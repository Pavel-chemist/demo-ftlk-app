use fltk::{app, prelude::*, window, button};

fn main() {
    let application = app::App::default();
    let mut wind = window::Window::new(256, 128, 512, 256, "My first ftlk Window");
    let butt = button::Button::new(192, 96, 128, 32, "Click me!");

    wind.end();
    wind.show();

    application.run().unwrap();
}