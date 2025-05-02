use dioxus::{html::div, prelude::*};

mod header;
mod viewers;
mod table;

use header::Header;
use viewers::Viewer;
use table::Table;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/tailwind.css") }

        div { 
            class: "min-h-screen bg-gray-50 dark:bg-gray-900 transition-colors duration-300",

            Header{}

            Viewer{}

            Table{}
         }

    }
}

