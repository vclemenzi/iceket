extern crate gtk;
extern crate webkit2gtk;
use gtk::{prelude::*, Box, Orientation, SearchEntry, Window};
use webkit2gtk::{WebContext, WebView, WebViewExt};


pub fn build(window: &Window) {
    let vbox = Box::new(Orientation::Vertical, 0); 

    let webview = WebView::with_context(&WebContext::default().unwrap());

    webview.load_uri("https://www.google.com/"); // Open google

    let navbox = Box::builder()
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .spacing(12)
        .build();

    let search_bar = SearchEntry::new();


    navbox.pack_start(&search_bar, true, true, 0);

    // Vbox components
    vbox.pack_start(&navbox, false, false, 0);
    vbox.pack_start(&webview, true, true, 0);

    window.set_child(Some(&vbox));
}
