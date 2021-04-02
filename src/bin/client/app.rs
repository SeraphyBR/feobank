use cursive::{Cursive, CursiveRunnable, event::{Callback, Key}, menu, traits::*, views::{Dialog, EditView, LinearLayout, TextView}};
use feobank::user::UserAction;
use crate::session::Session;
use std::{cell::RefCell, io::Write, sync::atomic::{AtomicUsize, Ordering}};
use cursive::menu::MenuTree;
use std::net::TcpStream;


pub struct App {
    ui: CursiveRunnable
}

impl App {
    pub fn new() -> App {
        App {
            ui: cursive::default()
        }
    }

    pub fn run(&mut self) {
        self.create_ui();
        self.ui.run();
    }

    fn create_ui(&mut self) {
        App::create_menubar(&mut self.ui);
        App::connect_to_server_dialog(&mut self.ui);
        self.ui.set_autohide_menu(false);
        self.ui.add_global_callback(Key::Esc, |ui| ui.select_menubar());
    }

    fn create_menubar(ui: &mut Cursive) {
        // The menubar is a list of (label, menu tree) pairs.
        ui.menubar()
            .add_subtree("Help",App::help_menu())
            .add_delimiter()
            .add_leaf("Quit", |ui| ui.quit());
    }

    fn help_menu() -> MenuTree {
        MenuTree::new()
        .leaf("About", |ui|{
            let message = "Feobank é um projeto de internet banking para o trabalho de laborátorio de redes.";
            ui.add_layer(Dialog::info(message));
        })
    }

    fn connect_to_server_dialog(ui: &mut Cursive) {
        // Create a dialog with an edit text and a button.
        // The user can either hit the <Ok> button,
        // or press Enter on the edit text.
        ui.add_layer(
            Dialog::new()
                .title("Enter server ip address and port:")
                // Padding is (left, right, top, bottom)
                .padding_lrtb(1, 1, 1, 0)
                .content(
                    EditView::new()
                        .max_content_width(24)
                        // Call `show_popup` when the user presses `Enter`
                        .on_submit(App::try_connect_server)
                        // Give the `EditView` a name so we can refer to it later.
                        .with_name("connection_dialog")
                        .fixed_width(25)
                )
                .button("Ok", |ui| {
                    // This will run the given closure, *ONLY* if a view with the
                    // correct type and the given name is found.
                    let addr = ui
                        .call_on_name("connection_dialog", |view: &mut EditView| {
                            // We can return content from the closure!
                            view.get_content()
                        })
                        .unwrap();

                    // Run the next step
                    App::try_connect_server(ui, &addr);
                }),
        );
    }

    // This will replace the current layer with a new popup.
    // If the name is empty, we'll show an error message instead.
    fn try_connect_server(ui: &mut Cursive, addr: &str) {
        if addr.is_empty() {
            // Try again as many times as we need!
            ui.add_layer(Dialog::info("Please enter the feobank's server address!"));
        } else {
            let content = format!("Connecting to {}...", addr);
            // Remove the initial popup
            ui.pop_layer();
            // And put a new one instead
            ui.add_layer(
                Dialog::around(TextView::new(content))
                    .button("Ok", |s| {s.pop_layer();}),
            );

            if let Ok(s) = TcpStream::connect(addr) {
                // Remove the initial popup
                ui.pop_layer();
                // And put a new one instead
                ui.add_layer(
                    Dialog::around(TextView::new("Connected!"))
                        .button("Ok", |s| {s.pop_layer();}),
                );

                let session = RefCell::new(Session::new(s));
                App::login_dialog(ui, session);
            }
        }
    }

    fn login_dialog(ui: &mut Cursive, session: RefCell<Session>) {
        ui.add_layer(
            Dialog::new()
                .title("Login")
                // Padding is (left, right, top, bottom)
                .padding_lrtb(1, 1, 1, 0)
                .content(
                    LinearLayout::vertical()
                        .child(TextView::new("CPF:"))
                        .child(
                        EditView::new()
                            .max_content_width(11)
                            // Give the `EditView` a name so we can refer to it later.
                            .with_name("login_cpf")
                        )
                        .child(TextView::new("Password:"))
                        .child(
                        EditView::new()
                            .max_content_width(16)
                            .secret()
                            // Give the `EditView` a name so we can refer to it later.
                            .with_name("login_password")
                        )
                )
                .button("Create account", |c| {

                })
                .button("Enter", move |ui| {
                    // This will run the given closure, *ONLY* if a view with the
                    // correct type and the given name is found.
                    let cpf = ui
                        .call_on_name("login_cpf", |view: &mut EditView| {
                            // We can return content from the closure!
                            view.get_content()
                        })
                        .unwrap()
                        .to_string();
                    let password = ui
                        .call_on_name("login_password", |view: &mut EditView| {
                            // We can return content from the closure!
                            view.get_content()
                        })
                        .unwrap()
                        .to_string();

                    match session.borrow_mut().login(cpf, password){
                        Ok(()) => {
                            // Remove the initial popup
                            ui.pop_layer();
                            // And put a new one instead
                            ui.add_layer(
                                Dialog::around(TextView::new("successfully logged in!"))
                                    .button("Ok", |ui| {ui.pop_layer();}),
                            );
                        }
                        Err(msg) => {
                            let content = format!("Error: {}", msg);
                            ui.add_layer(
                                Dialog::around(TextView::new(content))
                                    .button("Ok", |ui| {ui.pop_layer();}),
                            );
                        }
                    };
                }),
        );
    }

    fn main_menubar(ui: &mut Cursive, s: Session) {
        todo!()
    }
}