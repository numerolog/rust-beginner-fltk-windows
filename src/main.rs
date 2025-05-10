

use fltk::{app, button::Button, frame::Frame, prelude::*, window::Window};
use fltk_theme::{ColorTheme, color_themes};

macro_rules! println {
    ($($arg:tt)*) => {};
}

fn main() {
    println!("Hello, world 1!");
    // let app = app::App::default();
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    if is_dark()
    {
        let theme = ColorTheme::new(color_themes::BLACK_THEME);
        theme.apply();
    }
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

#[cfg(windows)]
mod windows
{
    use windows_registry;

    pub fn is_dark() -> bool
    {
        let key = windows_registry::CURRENT_USER
            .open(r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize")
            .expect("failed to open registry key");

        let light_theme = key
            .get_u32("AppsUseLightTheme")
            .expect("failed to read value from registry key");

        return light_theme == 0
    }
}

#[cfg(unix)]
mod linux
{
    use std::env;
    pub fn is_dark() -> bool {
        return env::var("GTK_THEME").unwrap_or("".parse().unwrap()).contains("dark");
    }
}

fn is_dark() -> bool
{
    #[cfg(windows)]
    {
        return windows::is_dark();
    }
    #[cfg(unix)]
    {
        return linux::is_dark();
    }
    #[cfg(not(any(windows, unix)))]
    {
        return false;
    }
}
