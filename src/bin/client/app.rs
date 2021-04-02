use chrono::{NaiveDate, Utc};
use cursive::{Cursive, CursiveRunnable, event::{Callback, Key}, menu, traits::*, views::{Button, Dialog, EditView, LinearLayout, TextView}};
use cursive_calendar_view::{CalendarView, EnglishLocale};
use feobank::user::{NewUser, UserAction};
use crate::session::Session;
use cursive::menu::MenuTree;
use std::{collections::HashMap, net::TcpStream, str::FromStr};


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
        self.create_menubar();
        self.connect_to_server_dialog();
        self.ui.set_autohide_menu(false);
        self.ui.add_global_callback(Key::Esc, |ui| ui.select_menubar());
    }

    fn create_menubar(&mut self) {
        // The menubar is a list of (label, menu tree) pairs.
        self.ui.menubar()
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

    fn connect_to_server_dialog(&mut self) {
        self.ui.add_layer(
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
                    .button("Ok", |ui| {ui.pop_layer();}),
            );

            if let Ok(s) = TcpStream::connect(addr) {
                // Remove the initial popup
                ui.pop_layer();
                // And put a new one instead
                ui.add_layer(
                    Dialog::around(TextView::new("Connected!"))
                        .button("Ok", |ui| {ui.pop_layer();}),
                );

                //let session = Rc::new(RefCell::new(Session::new(s)));
                ui.set_user_data(Session::new(s));
                App::login_dialog(ui);
            }
        }
    }

    fn login_dialog(ui: &mut Cursive) {
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
                .button("Create account", |ui| {
                    App::create_account_dialog(ui);
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

                    let session = ui.user_data::<Session>().unwrap();
                    match session.login(cpf, password) {
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

    fn create_account_dialog(ui: &mut Cursive) {
        ui.add_layer(
            Dialog::new()
                .title("Create Account")
                // Padding is (left, right, top, bottom)
                .padding_lrtb(1, 1, 1, 0)
                .content(
                    LinearLayout::vertical()
                        .child(TextView::new("Full Name:"))
                        .child(
                        EditView::new()
                            .max_content_width(120)
                            // Give the `EditView` a name so we can refer to it later.
                            .with_name("name")
                        )
                        .child(TextView::new("Email:"))
                        .child(
                        EditView::new()
                            .max_content_width(40)
                            // Give the `EditView` a name so we can refer to it later.
                            .with_name("email")
                        )
                        .child(TextView::new("Birthdate: "))
                        .child(
                            LinearLayout::horizontal()
                            .child(
                                EditView::new()
                                .max_content_width(10)
                                .with_name("birthdate")
                                .fixed_width(11)
                            )
                            .child(
                                Button::new("Select", |ui| {
                                    let calendar = CalendarView::<Utc, EnglishLocale>::new(Utc::today())
                                        .on_submit(|ui, date| {
                                            ui.call_on_name("birthdate", |v: &mut EditView| {
                                                v.set_content(date.format("%Y-%m-%d").to_string());
                                            });
                                            ui.pop_layer();
                                        });
                                    ui.add_layer(
                                        Dialog::new()
                                            .title("Calendar")
                                            .padding_lrtb(1, 1, 1, 0)
                                            .content(calendar)
                                    );
                                })
                            )
                        )
                        .child(TextView::new("Phone number:"))
                        .child(
                        EditView::new()
                            .max_content_width(16)
                            // Give the `EditView` a name so we can refer to it later.
                            .with_name("phone")
                        )
                        .child(TextView::new("Address:"))
                        .child(
                        EditView::new()
                            .max_content_width(16)
                            // Give the `EditView` a name so we can refer to it later.
                            .with_name("address")
                        )
                        .child(TextView::new("CPF:"))
                        .child(
                        EditView::new()
                            .max_content_width(16)
                            // Give the `EditView` a name so we can refer to it later.
                            .with_name("cpf")
                        )
                        .child(TextView::new("Password:"))
                        .child(
                        EditView::new()
                            .max_content_width(16)
                            // Give the `EditView` a name so we can refer to it later.
                            .with_name("password")
                        )
                )
                .button("Cancel", |ui| {
                    ui.pop_layer();
                })
                .button("Create", |ui| {
                    let callbacks = ["name", "email", "birthdate", "phone", "address", "cpf", "password"];
                    let mut values = HashMap::new();
                    for name in callbacks.iter() {
                        let value = ui.call_on_name(name, |v: &mut EditView| v.get_content())
                            .unwrap();
                        values.insert(*name, value);
                    }

                    let birthdate = NaiveDate::from_str(&values["birthdate"]).unwrap();
                    let user = NewUser {
                        name: values["name"].to_string(),
                        email: values["email"].to_string(),
                        cpf: values["cpf"].to_string(),
                        password: values["password"].to_string(),
                        address: values["address"].to_string(),
                        phone: values["phone"].to_string(),
                        birthdate
                    };
                    let session = ui.user_data::<Session>().unwrap();
                    match session.create_user(user) {
                        Ok(()) => {
                            // Remove the initial popup
                            ui.pop_layer();
                            // And put a new one instead
                            ui.add_layer(
                                Dialog::around(TextView::new("User created!"))
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
                    }
                }),
        );
    }

    fn main_menubar(ui: &mut Cursive) {
        todo!()
    }
}