use cursive::{Cursive, CursiveRunnable, event::{Callback, Key}, menu, traits::*, views::{Dialog, EditView, LinearLayout, TextView}};
use feobank::account::AccountAction;
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
        self.ui.add_global_callback(Key::Esc, |s| s.select_menubar());
    }

    fn create_menubar(ui: &mut CursiveRunnable) {
        // The menubar is a list of (label, menu tree) pairs.
        ui.menubar()
            .add_subtree("Help",App::help_menu())
            .add_delimiter()
            .add_leaf("Quit", |s| s.quit());
    }

    fn help_menu() -> MenuTree {
        MenuTree::new()
        .leaf("About", |ui|{
            let message = "Feobank é um projeto de internet banking para o trabalho de laborátorio de redes.";
            ui.add_layer(Dialog::info(message));
        })
    }

    fn connect_to_server_dialog(ui: &mut CursiveRunnable) {
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
                        // Call `show_popup` when the user presses `Enter`
                        .on_submit(App::try_connect_server)
                        // Give the `EditView` a name so we can refer to it later.
                        .with_name("connection_dialog")
                        // Wrap this in a `ResizedView` with a fixed width.
                        // Do this _after_ `with_name` or the name will point to the
                        // `ResizedView` instead of `EditView`!
                        .fixed_width(20)
                )
                .button("Ok", |s| {
                    // This will run the given closure, *ONLY* if a view with the
                    // correct type and the given name is found.
                    let addr = s
                        .call_on_name("connection_dialog", |view: &mut EditView| {
                            // We can return content from the closure!
                            view.get_content()
                        })
                        .unwrap();

                    // Run the next step
                    App::try_connect_server(s, &addr);
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

            if let Ok(mut s) = TcpStream::connect(addr) {
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

    fn login_dialog(ui: &mut Cursive, s: RefCell<Session>) {
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
                            // Give the `EditView` a name so we can refer to it later.
                            .with_name("login_cpf")
                            // Wrap this in a `ResizedView` with a fixed width.
                            // Do this _after_ `with_name` or the name will point to the
                            // `ResizedView` instead of `EditView`!
                            .fixed_width(11)
                        )
                        .child(TextView::new("Password:"))
                        .child(
                        EditView::new()
                            // Give the `EditView` a name so we can refer to it later.
                            .with_name("login_password")
                            // Wrap this in a `ResizedView` with a fixed width.
                            // Do this _after_ `with_name` or the name will point to the
                            // `ResizedView` instead of `EditView`!
                            .fixed_width(16)
                        )
                )
                .button("Create account", |c| {

                })
                .button("Enter", move |c| {
                    // This will run the given closure, *ONLY* if a view with the
                    // correct type and the given name is found.
                    let cpf = c
                        .call_on_name("login_cpf", |view: &mut EditView| {
                            // We can return content from the closure!
                            view.get_content()
                        })
                        .unwrap()
                        .to_string();
                    let password = c
                        .call_on_name("login_password", |view: &mut EditView| {
                            // We can return content from the closure!
                            view.get_content()
                        })
                        .unwrap()
                        .to_string();

                    s.borrow_mut().login(cpf, password);
                }),
        );
    }

    fn main_menubar(ui: &mut Cursive, s: Session) {

    }
}