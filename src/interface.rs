extern crate gtk;
extern crate glib;
extern crate webkit2gtk;
use gtk::{prelude::*, Box, Orientation, SearchEntry, Window};
use webkit2gtk::{WebContext, WebView, WebViewExt, SettingsExt};
use glib::clone;

use crate::{html, utils};


pub fn build(window: &Window) {
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

    search_bar.connect_activate(clone!(@weak webview => move |input| {
        let text = input.text();
        
        if utils::is_url(&text) {
            webview.load_uri(&text);
        } else {
            let search_string = format!("https://www.google.com/search?q={}", text.replace(" ", "+"));

            webview.load_uri(&search_string);
        }
    }));


    navbox.pack_start(&search_bar, true, true, 0);

    // Vbox components
    vbox.pack_start(&navbox, false, false, 0);
    vbox.pack_start(&webview, true, true, 0);

    window.set_child(Some(&vbox));
}
