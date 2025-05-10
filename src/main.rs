

use fltk::{app, button::Button, frame::Frame, prelude::*, window::Window};
macro_rules! println {
    ($($arg:tt)*) => {};
}

fn main() {
    println!("Hello, world 1!");
    let app = app::App::default();
    let mut wind = Window::new(100, 100, 400, 300, "Hello from rust");
    let mut frame = Frame::new(0, 0, 400, 200, "");
    let mut but = Button::new(160, 210, 80, 40, "Click me!");
    println!("Hello, world 2!");
    wind.end();
    println!("Hello, world 3!");
    wind.show();
    println!("Hello, world 4!");
    but.set_callback(move |_| frame.set_label("Hello World!")); // the closure capture is mutable borrow to our button
    println!("Hello, world 5!");
    app.run().unwrap();
    println!("Hello, world 6!");
}
