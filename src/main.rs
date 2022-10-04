use fltk::{app, prelude::*, *, enums::{FrameType, Color}, frame::Frame};

fn main() {
    let application = app::App::default();
    let mut wind = window::Window::new(256, 128, 512, 512, "My first ftlk Window");

    let mut frame = frame::Frame::new(10, 10, 492, 40, "Click on button below:");
    frame.set_frame(FrameType::EngravedFrame);
    let mut butt = button::Button::new(10, 60, 200, 40, "Click me!");

    let mut file_path_frame = frame::Frame::new(10, 110, 492, 40, "File path");
    file_path_frame.set_frame(FrameType::ShadowBox);
    file_path_frame.set_color( Color::White);
    let mut file_select_button = button::Button::new(10, 160, 200, 40, "Chose a file");

    wind.end();
    wind.show();

    butt.set_callback(move |_| butt_action(&mut frame));
    file_select_button.set_callback(move |_| open_file_dialog(&mut file_path_frame));

    application.run().unwrap();
}

fn butt_action(frame: &mut Frame) {
    return frame.set_label("You've clicked on button");
}

fn open_file_dialog(frame: &mut Frame) {
    let mut dialog = dialog::NativeFileChooser::new(dialog::NativeFileChooserType::BrowseFile);
    dialog.show();

    let file_name: String = dialog.filename().into_os_string().into_string().unwrap();
    
    if file_name.len() > 0 {
        return frame.set_label(&file_name);    
    }

    return frame.set_label("No file chosen");
}