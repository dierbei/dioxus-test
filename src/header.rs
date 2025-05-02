use dioxus::prelude::*;

#[component]
pub fn Header() -> Element {
    rsx!(
        header { 
            class: "bg-white dark:bg-gray-800 shadow-sm transition-colors duration-300",
    
            div {
                class: "container mx-auto px-4 py-4",
                
                div {
                    class: "flex items-center justify-between",
    
                    div {
                        class: "flex items-center gap-2",
    
                        h1 {
                            class: "text-xl font-semibold text-gray-900 dark:text-white",
                            "WiFi Password Viewer"
                        }
                    }
                }
            }
         }
    )
}