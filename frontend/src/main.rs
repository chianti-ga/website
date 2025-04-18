pub use app::App;
use eframe::web_sys;
use lazy_static::lazy_static;
use log::info;
use serde::Deserialize;

mod app;
mod backend_handler;
mod ui;

const GIT_COMMIT: Option<&str> = option_env!("GIT_COMMIT");
const GIT_BRANCH: Option<&str> = option_env!("GIT_BRANCH");
const BUILD_TIMESTAMP: Option<&str> = option_env!("BUILD_TIMESTAMP");
const GIT_TAG: Option<&str> = option_env!("GIT_TAG");


// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast as _;

    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");

        let canvas = document
            .get_element_by_id("wasm_canvas")
            .expect("Failed to find wasm_canvas")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("wasm_canvas was not a HtmlCanvasElement");

        let start_result = eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|cc| Ok(Box::new(App::new(cc)))),
            )
            .await;

        // Remove the loading text and spinner:
        let loading_text = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id("loading_text"));
        if let Some(loading_text) = loading_text {
            match start_result {
                Ok(_) => {
                    loading_text.remove();
                    info!("Runing frontend version {} on branch {} and compiled at {}", GIT_TAG.unwrap_or_else(|| "unknown"), GIT_BRANCH.unwrap_or_else(|| "unknown"), BUILD_TIMESTAMP.unwrap_or_else(|| "unknown"));
                }
                Err(e) => {
                    loading_text.set_inner_html(
                        "<p> The app has crashed. See the developer console for details. </p>",
                    );
                    panic!("Failed to start eframe: {e:?}");
                }
            }
        }
    });
}

