// use crate::{
//     search_input::SearchInput,
//     voyage_table::VoyageTable,
// };
// use db::search::{search_places, get_voyages};
// use models::{
//     place::Place,
//     yew::{PortSearchProps, SearchNode, SearchState, NodeType}
// };
// use wasm_bindgen_futures::spawn_local;
// use yew::prelude::*;

// #[function_component(PortSearch)]
// pub fn port_search(props: &PortSearchProps) -> Html {
//     let database = props.database.clone();

//     let origin_node = use_state(|| SearchNode {
//         state: SearchState::NotSet,
//         selected_place: None,
//         input_value: String::new(),
//     });

//     let destination_node = use_state(|| SearchNode {
//         state: SearchState::NotSet,
//         selected_place: None,
//         input_value: String::new(),
//     });

//     let suggestions = use_state(Vec::new);
//     let voyages = use_state(Vec::new);
//     let error = use_state(|| None::<String>);

//     let handle_search = {
//         let suggestions = suggestions.clone();
//         let database = database.clone();
//         let origin_node = origin_node.clone();
//         let destination_node = destination_node.clone();
//         let error = error.clone();

//         Callback::from(move |search_params: (String, NodeType)| {
//             let (query, node_type) = search_params;
//             let database = database.clone();

//             if query.is_empty() {
//                 suggestions.set(vec![]);
//                 return;
//             }

//             let other_node = match node_type {
//                 NodeType::Origin => (*destination_node).clone(),
//                 NodeType::Destination => (*origin_node).clone(),
//             };

//             let suggestions = suggestions.clone();
//             let error = error.clone();

//             spawn_local(async move {
//                 match search_places(
//                     &database,
//                     &query,
//                     &node_type,
//                     other_node.selected_place.as_ref()
//                 ) {
//                     Ok(places) => suggestions.set(places),
//                     Err(error_message) => error.set(Some(error_message.to_string())),
//                 }
//             });
//         })
//     };

//     let handle_select = {
//         let origin_node = origin_node.clone();
//         let destination_node = destination_node.clone();
//         let suggestions = suggestions.clone();
//         let database = database.clone();
//         let voyages = voyages.clone();
//         let error = error.clone();

//         Callback::from(move |select_params: (Place, NodeType)| {
//             let (place, node_type) = select_params;
//             let database = database.clone();

//             match node_type {
//                 NodeType::Origin => {
//                     origin_node.set(SearchNode {
//                         state: SearchState::Set,
//                         selected_place: Some(place.clone()),
//                         input_value: place.name.clone(),
//                     });
//                 },
//                 NodeType::Destination => {
//                     destination_node.set(SearchNode {
//                         state: SearchState::Set,
//                         selected_place: Some(place.clone()),
//                         input_value: place.name.clone(),
//                     });
//                 },
//             }

//             suggestions.set(vec![]);

//             // If both nodes are set, fetch voyages
//             if let (Some(origin), Some(destination)) = (
//                 &origin_node.selected_place,
//                 &destination_node.selected_place
//             ) {
//                 // Clone the values we need in the async block
//                 let origin = origin.clone();
//                 let destination = destination.clone();
//                 let voyages = voyages.clone();
//                 let error = error.clone();
//                 let database = database.clone();

//                 spawn_local(async move {
//                     match get_voyages(&database, &origin, &destination) {
//                         Ok(found_voyages) => voyages.set(found_voyages),
//                         Err(error_message) => error.set(Some(error_message.to_string())),
//                     }
//                 });
//             }
//         })
//     };

//     // let handle_select = {
//     //     let origin_node = origin_node.clone();
//     //     let destination_node = destination_node.clone();
//     //     let suggestions = suggestions.clone();
//     //     let database = database.clone();
//     //     let voyages = voyages.clone();
//     //     let error = error.clone();

//     //     Callback::from(move |select_params: (Place, NodeType)| {
//     //         let (place, node_type) = select_params;
//     //         let database = database.clone();

//     //         match node_type {
//     //             NodeType::Origin => {
//     //                 origin_node.set(SearchNode {
//     //                     state: SearchState::Set,
//     //                     selected_place: Some(place.clone()),
//     //                     input_value: place.name.clone(),
//     //                 });
//     //             },
//     //             NodeType::Destination => {
//     //                 destination_node.set(SearchNode {
//     //                     state: SearchState::Set,
//     //                     selected_place: Some(place.clone()),
//     //                     input_value: place.name.clone(),
//     //                 });
//     //             },
//     //         }

//     //         suggestions.set(vec![]);

//     //         // If both nodes are set, fetch voyages
//     //         if let (Some(origin), Some(destination)) = (
//     //             &origin_node.selected_place,
//     //             &destination_node.selected_place
//     //         ) {
//     //             let voyages = voyages.clone();
//     //             let error = error.clone();
//     //             let database = database.clone();

//     //             spawn_local(async move {
//     //                 match get_voyages(&database, origin, destination) {
//     //                     Ok(found_voyages) => voyages.set(found_voyages),
//     //                     Err(error_message) => error.set(Some(error_message.to_string())),
//     //                 }
//     //             });
//     //         }
//     //     })
//     // };

//     let reset_search = {
//         let origin_node = origin_node.clone();
//         let destination_node = destination_node.clone();
//         let suggestions = suggestions.clone();
//         let voyages = voyages.clone();
//         let error = error.clone();

//         Callback::from(move |_| {
//             origin_node.set(SearchNode {
//                 state: SearchState::NotSet,
//                 selected_place: None,
//                 input_value: String::new(),
//             });
//             destination_node.set(SearchNode {
//                 state: SearchState::NotSet,
//                 selected_place: None,
//                 input_value: String::new(),
//             });
//             suggestions.set(vec![]);
//             voyages.set(vec![]);
//             error.set(None);
//         })
//     };

//     html! {
//         <div class="w-full max-w-4xl mx-auto p-4">
//             if let Some(error_message) = &*error {
//                 <div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">
//                     {error_message}
//                 </div>
//             }

//             <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-4">
//                 <SearchInput
//                     node_type={NodeType::Origin}
//                     value={origin_node.input_value.clone()}
//                     on_search={handle_search.clone()}
//                     on_select={handle_select.clone()}
//                     suggestions={(*suggestions).clone()}
//                     is_set={matches!(origin_node.state, SearchState::Set)}
//                 />

//                 <SearchInput
//                     node_type={NodeType::Destination}
//                     value={destination_node.input_value.clone()}
//                     on_search={handle_search.clone()}
//                     on_select={handle_select.clone()}
//                     suggestions={(*suggestions).clone()}
//                     is_set={matches!(destination_node.state, SearchState::Set)}
//                 />
//             </div>

//             <button
//                 onclick={reset_search}
//                 class="px-4 py-2 bg-gray-200 rounded hover:bg-gray-300 mb-4"
//             >
//                 {"Reset Search"}
//             </button>

//             if !(*voyages).is_empty() {
//                 <VoyageTable voyages={(*voyages).clone()} />
//             }
//         </div>

//     }
// }

// use crate::{
//     search_input::SearchInput,
//     voyage_table::VoyageTable,
// };
// use db::search::{search_places, get_voyages};
// use models::{
//     place::Place,
//     yew::{PortSearchProps, SearchNode, SearchState, NodeType}
// };
// use wasm_bindgen_futures::spawn_local;
// use yew::prelude::*;

// #[function_component(PortSearch)]
// pub fn port_search(props: &PortSearchProps) -> Html {
//     let database = props.database.clone();

//     let origin_node = use_state(|| SearchNode {
//         state: SearchState::NotSet,
//         selected_place: None,
//         input_value: String::new(),
//     });

//     let destination_node = use_state(|| SearchNode {
//         state: SearchState::NotSet,
//         selected_place: None,
//         input_value: String::new(),
//     });

//     // Separate suggestion states for origin and destination
//     let origin_suggestions = use_state(Vec::new);
//     let destination_suggestions = use_state(Vec::new);
//     let voyages = use_state(Vec::new);
//     let error = use_state(|| None::<String>);

//     let handle_search = {
//         let database = database.clone();
//         let origin_node = origin_node.clone();
//         let destination_node = destination_node.clone();
//         let origin_suggestions = origin_suggestions.clone();
//         let destination_suggestions = destination_suggestions.clone();
//         let error = error.clone();

//         Callback::from(move |search_params: (String, NodeType)| {
//             let (query, node_type) = search_params;
//             let database = database.clone();

//             // Update input value for the appropriate node
//             match node_type {
//                 NodeType::Origin => {
//                     origin_node.set(SearchNode {
//                         state: SearchState::NotSet,
//                         selected_place: None,
//                         input_value: query.clone(),
//                     });
//                 },
//                 NodeType::Destination => {
//                     destination_node.set(SearchNode {
//                         state: SearchState::NotSet,
//                         selected_place: None,
//                         input_value: query.clone(),
//                     });
//                 },
//             }

//             if query.is_empty() {
//                 match node_type {
//                     NodeType::Origin => origin_suggestions.set(vec![]),
//                     NodeType::Destination => destination_suggestions.set(vec![]),
//                 }
//                 return;
//             }

//             let other_node = match node_type {
//                 NodeType::Origin => (*destination_node).clone(),
//                 NodeType::Destination => (*origin_node).clone(),
//             };

//             let suggestions = match node_type {
//                 NodeType::Origin => origin_suggestions.clone(),
//                 NodeType::Destination => destination_suggestions.clone(),
//             };
//             let error = error.clone();

//             spawn_local(async move {
//                 match search_places(
//                     &database,
//                     &query,
//                     &node_type,
//                     other_node.selected_place.as_ref()
//                 ) {
//                     Ok(places) => suggestions.set(places),
//                     Err(error_message) => error.set(Some(error_message.to_string())),
//                 }
//             });
//         })
//     };

//     let handle_select = {
//         let origin_node = origin_node.clone();
//         let destination_node = destination_node.clone();
//         let origin_suggestions = origin_suggestions.clone();
//         let destination_suggestions = destination_suggestions.clone();
//         let database = database.clone();
//         let voyages = voyages.clone();
//         let error = error.clone();

//         Callback::from(move |select_params: (Place, NodeType)| {
//             let (place, node_type) = select_params;
//             let database = database.clone();

//             match node_type {
//                 NodeType::Origin => {
//                     origin_node.set(SearchNode {
//                         state: SearchState::Set,
//                         selected_place: Some(place.clone()),
//                         input_value: place.name.clone(),
//                     });
//                     origin_suggestions.set(vec![]);
//                 },
//                 NodeType::Destination => {
//                     destination_node.set(SearchNode {
//                         state: SearchState::Set,
//                         selected_place: Some(place.clone()),
//                         input_value: place.name.clone(),
//                     });
//                     destination_suggestions.set(vec![]);
//                 },
//             }

//             // If both nodes are set, fetch voyages
//             if let (Some(origin), Some(destination)) = (
//                 &origin_node.selected_place,
//                 &destination_node.selected_place
//             ) {
//                 let origin = origin.clone();
//                 let destination = destination.clone();
//                 let voyages = voyages.clone();
//                 let error = error.clone();
//                 let database = database.clone();

//                 spawn_local(async move {
//                     match get_voyages(&database, &origin, &destination) {
//                         Ok(found_voyages) => voyages.set(found_voyages),
//                         Err(error_message) => error.set(Some(error_message.to_string())),
//                     }
//                 });
//             }
//         })
//     };

//     let reset_search = {
//         let origin_node = origin_node.clone();
//         let destination_node = destination_node.clone();
//         let origin_suggestions = origin_suggestions.clone();
//         let destination_suggestions = destination_suggestions.clone();
//         let voyages = voyages.clone();
//         let error = error.clone();

//         Callback::from(move |_| {
//             origin_node.set(SearchNode {
//                 state: SearchState::NotSet,
//                 selected_place: None,
//                 input_value: String::new(),
//             });
//             destination_node.set(SearchNode {
//                 state: SearchState::NotSet,
//                 selected_place: None,
//                 input_value: String::new(),
//             });
//             origin_suggestions.set(vec![]);
//             destination_suggestions.set(vec![]);
//             voyages.set(vec![]);
//             error.set(None);
//         })
//     };

//     html! {
//         <div class="w-full max-w-4xl mx-auto p-4">
//             if let Some(error_message) = &*error {
//                 <div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">
//                     {error_message}
//                 </div>
//             }

//             <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-4">
//                 <SearchInput
//                     node_type={NodeType::Origin}
//                     value={origin_node.input_value.clone()}
//                     on_search={handle_search.clone()}
//                     on_select={handle_select.clone()}
//                     suggestions={(*origin_suggestions).clone()}
//                     is_set={matches!(origin_node.state, SearchState::Set)}
//                 />

//                 <SearchInput
//                     node_type={NodeType::Destination}
//                     value={destination_node.input_value.clone()}
//                     on_search={handle_search.clone()}
//                     on_select={handle_select.clone()}
//                     suggestions={(*destination_suggestions).clone()}
//                     is_set={matches!(destination_node.state, SearchState::Set)}
//                 />
//             </div>

//             <button
//                 onclick={reset_search}
//                 class="px-4 py-2 bg-gray-200 rounded hover:bg-gray-300 mb-4"
//             >
//                 {"Reset Search"}
//             </button>

//             if !(*voyages).is_empty() {
//                 <VoyageTable voyages={(*voyages).clone()} />
//             }
//         </div>
//     }
// }

use crate::{search_input::SearchInput, voyage_table::VoyageTable};
use db::search::{get_voyages, search_places};
use models::{
    place::Place,
    yew::{NodeType, PortSearchProps, SearchNode, SearchState},
};
use wasm_bindgen_futures::spawn_local;
// use web_sys::console::log;
use yew::prelude::*;

#[function_component(PortSearch)]
pub fn port_search(props: &PortSearchProps) -> Html {
    let database = props.database.clone();

    let origin_node = use_state(|| SearchNode {
        state: SearchState::NotSet,
        selected_place: None,
        input_value: String::new(),
    });

    let destination_node = use_state(|| SearchNode {
        state: SearchState::NotSet,
        selected_place: None,
        input_value: String::new(),
    });

    // Separate suggestion states for origin and destination
    let origin_suggestions = use_state(Vec::new);
    let destination_suggestions = use_state(Vec::new);
    let voyages = use_state(Vec::new);
    let error = use_state(|| None::<String>);

    let handle_search = {
        let database = database.clone();
        let origin_node = origin_node.clone();
        let destination_node = destination_node.clone();
        let origin_suggestions = origin_suggestions.clone();
        let destination_suggestions = destination_suggestions.clone();
        let error = error.clone();

        Callback::from(move |search_params: (String, NodeType)| {
            let (query, node_type) = search_params;
            let database = database.clone();

            log::info!(
                "Search triggered - query: '{}', node_type: {:?}, origin_set: {}, destination_set: {}",
                query,
                node_type,
                origin_node.selected_place.is_some(),
                destination_node.selected_place.is_some()
            );

            // Update input value for the appropriate node
            match node_type {
                NodeType::Origin => {
                    origin_node.set(SearchNode {
                        state: SearchState::NotSet,
                        selected_place: None,
                        input_value: query.clone(),
                    });
                }
                NodeType::Destination => {
                    destination_node.set(SearchNode {
                        state: SearchState::NotSet,
                        selected_place: None,
                        input_value: query.clone(),
                    });
                }
            }

            if query.is_empty() {
                match node_type {
                    NodeType::Origin => origin_suggestions.set(vec![]),
                    NodeType::Destination => destination_suggestions.set(vec![]),
                }
                return;
            }

            let other_node = match node_type {
                NodeType::Origin => (*destination_node).clone(),
                NodeType::Destination => (*origin_node).clone(),
            };

            let suggestions = match node_type {
                NodeType::Origin => origin_suggestions.clone(),
                NodeType::Destination => destination_suggestions.clone(),
            };
            let error = error.clone();

            spawn_local(async move {
                match search_places(
                    &database,
                    &query,
                    &node_type,
                    other_node.selected_place.as_ref(),
                ) {
                    Ok(places) => suggestions.set(places),
                    Err(error_message) => error.set(Some(error_message.to_string())),
                }
            });
        })
    };

    let handle_select = {
        let origin_node = origin_node.clone();
        let destination_node = destination_node.clone();
        let origin_suggestions = origin_suggestions.clone();
        let destination_suggestions = destination_suggestions.clone();
        let database = database.clone();
        let voyages = voyages.clone();
        let error = error.clone();

        Callback::from(move |select_params: (Place, NodeType)| {
            let (place, node_type) = select_params;
            let database = database.clone();

            match node_type {
                NodeType::Origin => {
                    origin_node.set(SearchNode {
                        state: SearchState::Set,
                        selected_place: Some(place.clone()),
                        input_value: place.name.clone(),
                    });
                    origin_suggestions.set(vec![]);
                }
                NodeType::Destination => {
                    destination_node.set(SearchNode {
                        state: SearchState::Set,
                        selected_place: Some(place.clone()),
                        input_value: place.name.clone(),
                    });
                    destination_suggestions.set(vec![]);
                }
            }

            // If both nodes are set, fetch voyages
            if let (Some(origin), Some(destination)) = (
                &origin_node.selected_place,
                &destination_node.selected_place,
            ) {
                let origin = origin.clone();
                let destination = destination.clone();
                let voyages = voyages.clone();
                let error = error.clone();
                let database = database.clone();

                spawn_local(async move {
                    match get_voyages(&database, &origin, &destination) {
                        Ok(found_voyages) => voyages.set(found_voyages),
                        Err(error_message) => error.set(Some(error_message.to_string())),
                    }
                });
            }
        })
    };

    let reset_search = {
        let origin_node = origin_node.clone();
        let destination_node = destination_node.clone();
        let origin_suggestions = origin_suggestions.clone();
        let destination_suggestions = destination_suggestions.clone();
        let voyages = voyages.clone();
        let error = error.clone();

        Callback::from(move |_| {
            origin_node.set(SearchNode {
                state: SearchState::NotSet,
                selected_place: None,
                input_value: String::new(),
            });
            destination_node.set(SearchNode {
                state: SearchState::NotSet,
                selected_place: None,
                input_value: String::new(),
            });
            origin_suggestions.set(vec![]);
            destination_suggestions.set(vec![]);
            voyages.set(vec![]);
            error.set(None);
        })
    };

    html! {
        <div class="w-full max-w-4xl mx-auto p-4">
            if let Some(error_message) = &*error {
                <div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">
                    {error_message}
                </div>
            }

            <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-4">
                <SearchInput
                    node_type={NodeType::Origin}
                    value={origin_node.input_value.clone()}
                    on_search={handle_search.clone()}
                    on_select={handle_select.clone()}
                    suggestions={(*origin_suggestions).clone()}
                    is_set={matches!(origin_node.state, SearchState::Set)}
                />

                <SearchInput
                    node_type={NodeType::Destination}
                    value={destination_node.input_value.clone()}
                    on_search={handle_search.clone()}
                    on_select={handle_select.clone()}
                    suggestions={(*destination_suggestions).clone()}
                    is_set={matches!(destination_node.state, SearchState::Set)}
                />
            </div>

            <button
                onclick={reset_search}
                class="px-4 py-2 bg-gray-200 rounded hover:bg-gray-300 mb-4"
            >
                {"Reset Search"}
            </button>

            if !(*voyages).is_empty() {
                <VoyageTable voyages={(*voyages).clone()} />
            }
        </div>
    }
}
