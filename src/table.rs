use dioxus::{prelude::*, CapturedError, Ok};
use serde::{Deserialize, Serialize};


#[component]
pub fn Table() -> Element {
    let mut items= use_signal(|| Vec::<Person>::new());

    rsx! {
        div {
            class: "relative overflow-x-auto",

            button {
                id: "list",
                class: "text-white bg-gradient-to-r from-pink-400 via-pink-500 to-pink-600 hover:bg-gradient-to-br focus:ring-4 focus:outline-none focus:ring-pink-300 dark:focus:ring-pink-800 font-medium rounded-lg text-sm px-5 py-2.5 text-center me-2 mb-2",
                onclick: move |_| async move {
                    items.set(list_person().await.unwrap())
                },

                "查询列表"
            }

           
            table {
                class: "w-full text-sm text-left rtl:text-right text-gray-500 dark:text-gray-400",
                
                thead {
                    class: "text-xs text-gray-700 uppercase bg-gray-50 dark:bg-gray-700 dark:text-gray-400",
                    
                    tr {
                        th {
                            scope: "col",
                            class: "px-6 py-3",
                            "Name"
                        }
                        th {
                            scope: "col",
                            class: "px-6 py-3",
                            "Color"
                        }
                        th {
                            scope: "col",
                            class: "px-6 py-3",
                            "Sex"
                        }
                        th {
                            scope: "col",
                            class: "px-6 py-3",
                            "Age"
                        }
                        th {
                            scope: "col",
                            class:"px-6 py-3",

                            span {
                                class: "sr-only",
                                "Edit"
                            }
                        }
                    }
                }
                
                tbody {
                    for item in items.iter() {
                        tr {
                            class: "bg-white border-b dark:bg-gray-800 dark:border-gray-700 border-gray-200",
                            
                            th {
                                scope: "row",
                                class: "px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white",
                                "{item.name}"
                            }
                            td {
                                class: "px-6 py-4",
                                "{item.color}"
                            }
                            td {
                                class: "px-6 py-4",
                                "{item.sex}"
                            }
                            td {
                                class: "px-6 py-4",
                                "{item.age}"
                            }
                            td {
                                class: "px-6 py-4 text-right",
    
                                a {
                                    href: "#",
                                    class: "font-medium text-blue-600 dark:text-blue-500 hover:underline",
                                    "Edit"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
} 

async fn list_person() -> Result<Vec<Person>, CapturedError> {
    let persons = vec![
        Person {
            name: "John Doe".to_string(),
            color: "Blue".to_string(),
            sex: "Male".to_string(),
            age: 25,
        },
        Person {
            name: "Jane Smith".to_string(), 
            color: "Red".to_string(),
            sex: "Female".to_string(),
            age: 30,
        },
        Person {
            name: "Alex Johnson".to_string(),
            color: "Green".to_string(), 
            sex: "Non-binary".to_string(),
            age: 28,
        },
        Person {
            name: "Sarah Williams".to_string(),
            color: "Purple".to_string(),
            sex: "Female".to_string(), 
            age: 35,
        }
    ];

    // reqwest::Client::new()
    //     .post("http://localhost:9090/api/list_person")
    //     .json(&persons)
    //     .send()
    //     .await?;

    Ok(persons)
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Person {
    pub name: String,
    pub color: String,
    pub sex: String,
    pub age: u32,
}