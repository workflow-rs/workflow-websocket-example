use std::{sync::Arc, time::Duration};
use wasm_bindgen::prelude::*;
use client_common::client_example;
// use workflow_html::{Render,html};
use workflow_log::*;

// A tiny log sink to dump logs to screen
struct Sink;
impl workflow_log::Sink for Sink {
    fn write(&self, _level:Level, args : &std::fmt::Arguments<'_>) -> bool {
        let text = format!("{}",args.to_string());
        let document = web_sys::window().unwrap().document().unwrap();
        let html_root = document.get_elements_by_tag_name("html").item(0).unwrap();
        let content = document.get_element_by_id("content").unwrap();
        // ~
        let text_node = document.create_text_node(&text);
        content.append_child(&text_node).unwrap();
        content.append_child(&document.create_element("br").unwrap()).unwrap();
        // ~ TODO fix html rendering issues:
        // html!({text}<br>123</br>).unwrap().inject_into(&content).unwrap();
        // html!({text}<br/>).unwrap().inject_into(&content).unwrap();
        // html!({"::"}{text}<br/>).unwrap().inject_into(&content).unwrap();
        // ~
        html_root.set_scroll_top(i32::MAX);
        false
    }
}

// ~

#[wasm_bindgen(start)]
pub async fn main() -> Result<(),String> {

    let sink = Sink {};
    workflow_log::pipe(Arc::new(sink));
    workflow_log::set_log_level(LevelFilter::Info);

    log_info!("starting...");

    let result = client_example(Duration::from_millis(1000)).await;
    log_info!("{:#?}", result);

    Ok(())
}


