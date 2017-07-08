extern crate gtk;
extern crate gio;

pub mod application;
pub mod ffi;

#[cfg(test)]
mod tests {
    use super::ffi;
    use std;
    use std::iter::Iterator;
    use gio::ApplicationFlags;
    use gtk::prelude::*;
    use gtk::{self, Application, Button, Window, WindowType};

    #[test]
    fn it_works() {
        unsafe {
            let granite_app: ffi::GraniteApplication =
                *ffi::granite_application_construct(ffi::granite_application_get_type());
            println!("{:?}", granite_app.program_name);
        }
        let application = Application::new(Some("granite.test.app"), ApplicationFlags::empty());

        if let Err(error) = application {
            panic!("{:?}", error);
        }

        let application = application.unwrap();
        let window = Window::new(WindowType::Toplevel);
        window.set_title("First GTK+ Program");
        window.set_default_size(350, 70);
        let button = Button::new_with_label("Click me!");
        window.add(&button);
        window.show_all();

        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });

        button.connect_clicked(|_| {
            println!("Clicked!");
        });

        application.add_window(&window);
        application.activate();
        application.run(0, &[]);
        gtk::main();
    }
}
