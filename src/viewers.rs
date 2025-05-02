use dioxus::prelude::*;
use dioxus_free_icons::icons::bs_icons::BsSearch;
use dioxus_free_icons::Icon;

#[component]
pub fn Viewer() -> Element {
    rsx!(
        div {
            class: "container mx-auto px-4 py-6",

            div {
                class: "space-y-6",
                
                div {
                    class: "flex flex-col sm:flex-row gap-4",
                    
                    div {
                        class: "relative flex-1",
                        
                        div {
                            class: "absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none",
                            Icon {
                                // width: 30,
                                // height: 30,
                                // fill: "test",
                                icon: BsSearch,
                            }
                        }
                        
                        input {
                            r#type: "text",
                            placeholder: "Search WiFi networks...",
                            class: "block w-full pl-10 pr-3 py-2 border border-gray-300 dark:border-gray-700 rounded-lg focus:ring-blue-500 focus:border-blue-500 bg-white dark:bg-gray-800 text-gray-900 dark:text-white transition-colors duration-200",
                            // value: "{cx.props.search_term}",
                            // oninput: move |e| cx.props.set_search_term.call(e.value.clone()),
                        }
                    }
                    
                    div {
                        select {
                            // value: "{cx.props.sort_by}",
                            // onchange: move |e| cx.props.set_sort_by.call(e.value.clone()),
                            class: "block w-full sm:w-auto px-3 py-2 border border-gray-300 dark:border-gray-700 rounded-lg focus:ring-blue-500 focus:border-blue-500 bg-white dark:bg-gray-800 text-gray-900 dark:text-white transition-colors duration-200",
                            
                            option {
                                value: "name",
                                "Sort by Name"
                            }
                            option {
                                value: "strength",
                                "Sort by Signal Strength"
                            }
                            option {
                                value: "frequency",
                                "Sort by Frequency"
                            }
                        }
                    }
                }
            }
         }   
    )
}