extern crate glib;
extern crate gtk;
extern crate webkit2gtk;
use crate::{html, utils};
use glib::clone;
use gtk::{gdk, prelude::*, Box, Button, IconSize, Orientation, SearchEntry, Window};
use webkit2gtk::{SettingsExt, WebContext, WebView, WebViewExt};

pub fn build(window: &Window) {
    // CSS styles
    let css_provider = gtk::CssProvider::new();
    css_provider
        .load_from_data(
            r#"
       .search {
            color: #fff;
            border: 0.05px solid #fff;
            border-radius: 5px;
        }

        button {
            border: 0.05px solid #fff;
            border-radius: 5px;
        }
    "#
            .as_bytes(),
        )
        .unwrap();
    gtk::StyleContext::add_provider_for_screen(
        &gdk::Screen::default().unwrap(),
        &css_provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    let vbox = Box::new(Orientation::Vertical, 0);

    let webview = WebView::with_context(&WebContext::default().unwrap());

    webview.load_html(&html::index(), None);

    let settings = WebViewExt::settings(&webview).unwrap();
    settings.set_enable_developer_extras(true);

    let navbox = Box::builder()
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .spacing(12)
        .build();

    let search_bar = SearchEntry::new();
    let reload_btn = Button::new();
    reload_btn.set_image(Some(&gtk::Image::from_icon_name(
        Some("view-refresh"),
        IconSize::Button,
    )));
    let other_btn = Button::new();
    other_btn.set_image(Some(&gtk::Image::from_icon_name(
        Some("zoom-select-fit"),
        IconSize::Button,
    )));
    /*let back_btn = Button::new();
    back_btn.set_image(Some(&gtk::Image::from_icon_name(Some("go-previous"), IconSize::Button)));
    let forward_btn = Button::new();
    forward_btn.set_image(Some(&gtk::Image::from_icon_name(Some("go-next"), IconSize::Button)));*/

    //navbox.pack_start(&back_btn, false, false, 0);
    //navbox.pack_start(&forward_btn, false, false, 0);
    navbox.pack_start(&reload_btn, false, false, 1);
    navbox.pack_start(&search_bar, true, true, 0);
    navbox.pack_start(&other_btn, false, false, 1);

    search_bar.connect_activate(clone!(@weak webview => move |input| {
        let text = input.text();

        if utils::is_url(&text) {
            webview.load_uri(&text);
        } else {
            let search_string = format!("https://duckduckgo.com/?q={}", text.replace(" ", "+"));
            webview.load_uri(&search_string);
        }
    }));

    reload_btn.connect_clicked(clone!(@weak webview => move |_| {
        webview.reload()
    }));

    webview.connect_is_loading_notify(clone!(@weak reload_btn, @weak search_bar  => move |webview| {
        let is_loading = webview.is_loading();

        if is_loading {
            reload_btn.set_image(Some(&gtk::Image::from_icon_name(Some("process-stop"), gtk::IconSize::Button)));
            search_bar.set_text(webview.uri().unwrap().as_str());
        } else {
            reload_btn.set_image(Some(&gtk::Image::from_icon_name(Some("view-refresh"), gtk::IconSize::Button)));
            search_bar.set_text(webview.uri().unwrap().as_str());
        }
    }));

    // Vbox components
    vbox.pack_start(&navbox, false, false, 0);
    vbox.pack_start(&webview, true, true, 0);

    window.set_child(Some(&vbox));
}
