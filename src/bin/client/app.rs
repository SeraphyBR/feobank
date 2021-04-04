use chrono::{NaiveDate, NaiveDateTime, Utc};
use cursive::{Cursive, CursiveRunnable, event::{Callback, Key}, menu, traits::*, views::{Button, Dialog, EditView, LinearLayout, SelectView, TextView}};
use cursive_calendar_view::{CalendarView, EnglishLocale};
use feobank::{bill::Bill, user::{NewUser, User, UserAction}};
use uuid::Uuid;
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
        self.ui.menubar()
            .add_subtree("Help",App::help_menu())
            .add_delimiter()
            .add_leaf("Quit", |ui| {
                    match ui.take_user_data::<Session>() {
                        Some(mut s) => {
                           s.close();
                        },
                        None => {}
                    };
                    ui.quit()
                }
            );
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
                .padding_lrtb(1, 1, 1, 0)
                .content(
                    EditView::new()
                        .max_content_width(24)
                        .content("127.0.0.1:7364")
                        .on_submit(App::try_connect_server)
                        .with_name("connection_dialog")
                        .fixed_width(25)
                )
                .button("Ok", |ui| {
                    let addr = ui
                        .call_on_name("connection_dialog", |view: &mut EditView| {
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
            ui.add_layer(Dialog::info("Please enter the feobank's server address!"));
        } else {
            let content = format!("Connecting to {}...", addr);
            ui.pop_layer();
            ui.add_layer(
                Dialog::around(TextView::new(content))
                    .button("Ok", |ui| {ui.pop_layer();}),
            );

            if let Ok(s) = TcpStream::connect(addr) {
                ui.pop_layer();

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
                .padding_lrtb(1, 1, 1, 0)
                .content(
                    LinearLayout::vertical()
                        .child(TextView::new("CPF:"))
                        .child(
                        EditView::new()
                            .max_content_width(11)
                            .content("02352154650")
                            .with_name("login_cpf")
                        )
                        .child(TextView::new("Password:"))
                        .child(
                        EditView::new()
                            .max_content_width(16)
                            .content("Senha")
                            .secret()
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
                            view.get_content()
                        })
                        .unwrap()
                        .to_string();
                    let password = ui
                        .call_on_name("login_password", |view: &mut EditView| {
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
                            App::action_menu_dialog(ui);
                            App::update_info_main_menu(ui);
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
                .padding_lrtb(1, 1, 1, 0)
                .content(
                    LinearLayout::vertical()
                        .child(TextView::new("Full Name:"))
                        .child(
                        EditView::new()
                            .max_content_width(120)
                            .with_name("name")
                        )
                        .child(TextView::new("Email:"))
                        .child(
                        EditView::new()
                            .max_content_width(40)
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
                            .with_name("phone")
                        )
                        .child(TextView::new("Address:"))
                        .child(
                        EditView::new()
                            .max_content_width(16)
                            .with_name("address")
                        )
                        .child(TextView::new("CPF:"))
                        .child(
                        EditView::new()
                            .max_content_width(16)
                            .with_name("cpf")
                        )
                        .child(TextView::new("Password:"))
                        .child(
                        EditView::new()
                            .max_content_width(16)
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

                    let birthdate = NaiveDate::from_str(&values["birthdate"]).unwrap().and_hms(1, 0, 0);
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

    fn update_info_main_menu(ui: &mut Cursive) {
        let session = ui.user_data::<Session>().unwrap();
        match session.get_basic_info() {
            Some(i) => {
                ui.call_on_name("main_menu", |v: &mut Dialog|{
                    v.set_title(format!("Main Menu - User: {} - Balance: ${}", i.0, i.1))
                });
            }
            None => ()
        };
    }

    fn action_menu_dialog(ui: &mut Cursive) {
        ui.add_layer(
        Dialog::new()
            .title("Main Menu")
            // Padding is (left, right, top, bottom)
            .padding_lrtb(1, 1, 1, 0)
            .content(
                SelectView::new()
                .item("Pay Bill", 1)
                .item("Transfer Money", 2)
                .item("Get Statment", 3)
                .item("Create Bill", 4)
                .item("Log out", 5)
                .on_submit(|ui, item| {
                    match item {
                        1 => App::action_pay_bill_dialog(ui),
                        2 => App::action_transfer_money_dialog(ui),
                        3 => App::action_get_statment_dialog(ui),
                        4 => App::action_create_bill_dialog(ui),
                        5 => App::action_log_out(ui),
                        _ => unreachable!("no such item"),
                    };
                })
            )
            .with_name("main_menu")
        )
    }

    fn action_pay_bill_dialog(ui: &mut Cursive) {
        ui.add_layer(
            Dialog::new()
            .title("Pay Bill")
            .padding_lrtb(1, 1, 1, 0)
            .content(
                LinearLayout::vertical()
                .child(TextView::new("Bill Code:"))
                .child(
                EditView::new()
                    .max_content_width(36)
                    .with_name("bill_id")
                    .fixed_width(37)
                )
            )
            .button("Cancel", |ui| {ui.pop_layer();})
            .button("Next", |ui| {
                let bill_id = ui.call_on_name("bill_id", |v: &mut EditView| v.get_content())
                    .unwrap();
                let bill_id = Uuid::parse_str(&bill_id).unwrap();
                let session = ui.user_data::<Session>().unwrap();
                match session.get_bill_info(bill_id) {
                    Ok(bill) => App::display_bill_info_dialog(ui, bill),
                    Err(msg) => {
                        ui.add_layer(
                            Dialog::around(TextView::new(msg))
                                .button("Ok", |ui| {ui.pop_layer();}),
                        );
                    }
                };
            })
        )
    }

    fn display_bill_info_dialog(ui: &mut Cursive, bill: Bill) {
        ui.add_layer(
            Dialog::new()
            .title("Bill Info")
            .padding_lrtb(1, 1, 1, 0)
            .content(
                LinearLayout::vertical()
                .child(TextView::new(format!("Value: {}", bill.value)))
                .child(TextView::new(format!("Favored: {}", bill.favored_name)))
                .child(TextView::new(format!("Bill Code: {}", bill.id)))
            )
            .button("Cancel", |ui| {ui.pop_layer();})
            .button("Pay", move |ui| {
                let session = ui.user_data::<Session>().unwrap();
                match session.pay_bill(bill.id) {
                    Ok(()) => {},
                    Err(msg) => {
                        ui.add_layer(
                            Dialog::around(TextView::new(msg))
                                .button("Ok", |ui| {ui.pop_layer();}),
                        );
                    }
                };
            })
        )
    }

    fn action_transfer_money_dialog(ui: &mut Cursive) {
        ui.add_layer(
            Dialog::new()
            .title("Transfer Money")
            .padding_lrtb(1, 1, 1, 0)
            .content(
                LinearLayout::vertical()
                .child(TextView::new("CPF:"))
                .child(
                EditView::new()
                    .max_content_width(14)
                    .with_name("cpf")
                    .fixed_width(15)
                )
                .child(TextView::new("Value:"))
                .child(
                EditView::new()
                    .max_content_width(14)
                    .on_edit_mut(|ui, c, _|{
                        if let Err(_) = c.parse::<f32>() {
                            ui.call_on_name("value", |v: &mut EditView|{
                                let mut c = v.get_content().to_string();
                                c.pop();
                                v.set_content(&c);
                            });
                        }
                    })
                    .with_name("value")
                    .fixed_width(15)
                )
            )
            .button("Cancel", |ui| {ui.pop_layer();})
            .button("Next", |ui| {
                let cpf = ui.call_on_name("cpf", |v: &mut EditView| v.get_content())
                    .unwrap().to_string();
                //todo Validate CPF
                let value = ui.call_on_name("value", |v: &mut EditView| v.get_content())
                    .unwrap();
                let value = value.parse::<f32>().unwrap();
                let session = ui.user_data::<Session>().unwrap();
                match session.transfer_money(cpf, value) {
                    Ok(()) => {
                        App::update_info_main_menu(ui);
                        ui.add_layer(
                            Dialog::around(TextView::new("Successfully transferred!"))
                                .button("Ok", |ui| {ui.pop_layer();}),
                        );
                    },
                    Err(msg) => {
                        ui.add_layer(
                            Dialog::around(TextView::new(msg))
                                .button("Ok", |ui| {ui.pop_layer();}),
                        );
                    }
                };
            })
        )
    }

    fn action_get_statment_dialog(ui: &mut Cursive) {

    }

    fn action_create_bill_dialog(ui: &mut Cursive) {

    }

    fn action_log_out(ui: &mut Cursive) {
        let session = ui.user_data::<Session>().unwrap();
        session.logout();
        ui.pop_layer();
    }

}