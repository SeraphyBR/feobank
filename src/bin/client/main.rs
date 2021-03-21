use cursive::{event::{Callback, Key}, menu, traits::*, views::Dialog};
use std::sync::atomic::{AtomicUsize, Ordering};
use cursive::menu::MenuTree;

fn main() {
    let mut ui = cursive::default();

    // The menubar is a list of (label, menu tree) pairs.
    ui.menubar()
        // We add a new "File" tree
        .add_subtree(
            "File",
            MenuTree::new()
                .leaf("New", move |s| {

                    s.add_layer(Dialog::info("New file!"));
                })
                .delimiter(),
        )
        .add_subtree(
            "Help",
            MenuTree::new()
                .subtree(
                    "Help",
                    MenuTree::new()
                        .leaf("General", |s| {
                            s.add_layer(Dialog::info("Help message!"))
                        })
                        .leaf("Online", |s| {
                            let text = "Google it yourself!\n\
                                        Kids, these days...";
                            s.add_layer(Dialog::info(text))
                        }),
                )
                .leaf("About", |s| {
                    s.add_layer(Dialog::info("Cursive v0.0.0"))
                }),
        )
        .add_delimiter()
        .add_leaf("Quit", |s| s.quit());

    ui.set_autohide_menu(false);
    ui.add_global_callback(Key::Esc, |s| s.select_menubar());
    ui.add_layer(Dialog::text("Hit <Esc> to show the menu!"));

    ui.run();
}