#![windows_subsystem = "windows"]
extern crate gio;
extern crate gtk;
extern crate beemovie;

use gio::prelude::*;
use gtk::prelude::*;
use glib::clone;
use gtk::AboutDialog;
use std::thread;
use std::env::args;
use std::path::Path;
use std::fs;

fn system_menu(application: &gtk::Application) {
    let menu = gio::Menu::new();
    menu.append(Some("About"), Some("app.about"));
    menu.append(Some("Quit"), Some("app.quit"));
    menu.append(Some("Webhook"), Some("app.webhook"));
    application.set_app_menu(Some(&menu));
}

fn add_actions(application: &gtk::Application, window: &gtk::ApplicationWindow) {
    let about = gio::SimpleAction::new("about", None);
    about.connect_activate(clone!(@weak window => move |_, _| {
        let pog = AboutDialog::new();
        pog.set_transient_for(Some(&window));
        pog.set_website_label(Some("@roosterchicken"));
        pog.set_website(Some("https://github.com/roosterchicken/beemovie-gui"));
        pog.set_authors(&["Rooster"]);
        pog.set_title("About");
        pog.set_modal(true);
        pog.set_comments(Some("GUI for the Bee Movie crate written in GTK3."));
        pog.set_license_type(gtk::License::MitX11);
        pog.set_wrap_license(true);
        pog.set_program_name("beemovie-gui");
        pog.set_version(Some("v0.1.0"));
        pog.connect_response(|dialog, _| dialog.close());
        pog.show_all();
    })); 
    let quit = gio::SimpleAction::new("quit", None);
    quit.connect_activate(clone!(@weak window => move |_, _| {
        window.close();
    }));
    let weebhook = gio::SimpleAction::new("webhook", None);
    weebhook.connect_activate(clone!(@weak window => move |_, _| {
        if cfg!(feature = "discord") {
            #[cfg(feature="discord")]
            let configdir = dirs::config_dir().unwrap().into_os_string().into_string().unwrap() + "/beemovie-gui";
            #[cfg(feature="discord")]
            let configfile: String;
            #[cfg(feature="discord")]
            if cfg!(windows){
                configfile = dirs::config_dir().unwrap().into_os_string().into_string().unwrap() + "\\beemovie-gui\\webhook.txt";
            } else {
                configfile = dirs::config_dir().unwrap().into_os_string().into_string().unwrap() + "/beemovie-gui/webhook.txt";
            }
            #[cfg(feature="discord")]
            let configfilestr: &str = &configfile.clone();
            #[cfg(feature="discord")]
            if !Path::new(&configdir).exists() {
                fs::create_dir_all(&configdir).expect("Error creating directory");
                let data = "";
                fs::write(configfilestr, data).expect("Error writing file");
                let msg = "Please paste your Webhook URL into this file:\n".to_string() + configfilestr; 
                let dialog = gtk::MessageDialog::new(
                    Some(&window),
                    gtk::DialogFlags::MODAL,
                    gtk::MessageType::Error,
                    gtk::ButtonsType::Close,
                    &msg
                );
                dialog.connect_response(|dialog, _| dialog.close());
                dialog.show_all();
            } else {
                let data = fs::read_to_string(configfilestr).expect("Unable to read file");
                let dat = data;
                if dat == "" {
                    let msg = "Please paste your Webhook URL into this file:\n".to_string() + configfilestr; 
                    let dialog = gtk::MessageDialog::new(
                        Some(&window),
                        gtk::DialogFlags::MODAL,
                        gtk::MessageType::Error,
                        gtk::ButtonsType::Close,
                        &msg
                    );
                    dialog.connect_response(|dialog, _| dialog.close());
                    dialog.show_all();
                } else {
                    let data = fs::read_to_string(configfile).expect("Unable to read file");
                    let hook = data;
                    let message: String = beemovie::paragraph(1);
                    #[cfg(feature="discord")]
                    pog(hook.to_string(), message);
                    //window.set_title("Sent!");
                }
            }
        } else {
            let dialog = gtk::MessageDialog::new(
                Some(&window),
                gtk::DialogFlags::MODAL,
                gtk::MessageType::Error,
                gtk::ButtonsType::Close,
                "The Discord feature needs to be enabled at compile time to use Webhook. 😔"
            );
            dialog.connect_response(|dialog, _| dialog.close());
            dialog.show_all();
        }
    }));
    application.add_action(&about);
    application.add_action(&quit);
    application.add_action(&weebhook);
}

fn add_accelerators(application: &gtk::Application) {
    application.set_accels_for_action("app.about", &["F1"]);
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);
    window.set_title("Barry Benson");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(700, 140);
    window.set_resizable(false);
    let label = gtk::Label::new(None);
    label.set_markup("<span size='xx-large'>Click the button for a Bee Movie Paragraph!</span>");
    let buttonlabel = gtk::Label::new(None);
    buttonlabel.set_markup("<span size='xx-large'>Barry</span>");
    let button = gtk::Button::new();
    let text_view = gtk::TextView::new();
    text_view.set_wrap_mode(gtk::WrapMode::Word);
    text_view.set_editable(false);
    button.add(&buttonlabel);
    button.connect_clicked(clone!(@weak text_view => move |_| {
        let barry = &beemovie::paragraph(1);
        if cfg!(feature = "notify") {
        #[cfg(feature="notify")]
        use notify_rust::Notification;
        #[cfg(feature="notify")]
        Notification::new()
            .summary("beemovie-gui")
            .body(barry)
            .show();
        }
        text_view.get_buffer().expect("pog").set_text(barry);
    }));
    button.connect_clicked(clone!(@weak window => move |_| {
        window.resize(700, 1);
    }));
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    vbox.pack_start(&label, false, false, 0);
    vbox.pack_start(&button, false, false, 10);
    vbox.pack_start(&text_view, true, true, 0);
    window.add(&vbox);
    system_menu(application);
    add_actions(application, &window);
    window.show_all();
}

fn main() {
    let application =
        gtk::Application::new(Some("cf.roooster.beemovie.app"), Default::default())
            .expect("GTK Failed to load :(");
    application.connect_startup(|app| {
        add_accelerators(app);
    });
    application.connect_activate(|app| {
        build_ui(app)
    });
    application.run(&args().collect::<Vec<_>>());
}

// Don't look at this, I don't even know how I wrote this, like really.
#[cfg(feature="discord")]
fn pog(webhook_link: String, message: String) {
thread::spawn(|| {
    let hooklink = webhook_link;
    let mess = message;
    let client = reqwest::blocking::Client::new();
    let mut request_header = reqwest::header::HeaderMap::new();
    let pogmessage: String = String::from("{\"content\": \"") + &mess + &String::from("\"}");
    request_header.insert("Content-Type", reqwest::header::HeaderValue::from_static("application/json"));
    client.post(&hooklink)
        .body(pogmessage)
        .headers(request_header)
        .send();
});
}   