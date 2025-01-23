use models::{
    place::Place,
    yew::{NodeType, SearchInputProps},
};
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component(SearchInput)]
pub fn search_input(props: &SearchInputProps) -> Html {
    let input_ref = use_node_ref();

    let handle_input = {
        let on_search = props.on_search.clone();
        let node_type = props.node_type.clone();
        let input_ref = input_ref.clone();

        Callback::from(move |_e: InputEvent| {
            if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                on_search.emit((input.value(), node_type.clone()));
            }
        })
    };

    let handle_select = {
        let on_select = props.on_select.clone();
        let node_type = props.node_type.clone();

        Callback::from(move |place: Place| {
            on_select.emit((place.clone(), node_type.clone()));
        })
    };

    let get_label = match props.node_type {
        NodeType::Origin => "Origin Place",
        NodeType::Destination => "Destination Place",
    };

    html! {
        <div class="flex flex-col gap-2">
            <label class="text-sm font-medium">{get_label}</label>
            <div class="relative">
                <input
                    type="text"
                    ref={input_ref}
                    class="w-full p-2 border rounded"
                    placeholder="Search places..."
                    value={props.value.clone()}
                    oninput={handle_input}
                />
                if !props.suggestions.is_empty() {
                    <div class="absolute z-10 w-full mt-1 bg-white border rounded shadow-lg max-h-60 overflow-y-auto">
                        {
                            props.suggestions.iter().map(|place| {
                                let place_clone = place.clone();
                                let on_select = handle_select.clone();

                                html! {
                                    <div
                                        class="p-2 hover:bg-gray-100 cursor-pointer"
                                        onclick={Callback::from(move |_| {
                                            on_select.emit(place_clone.clone());
                                        })}
                                    >
                                        <div class="font-medium">{&place.name}</div>
                                        if let Some(region_name) = &place.region_name {
                                            <div class="text-sm text-gray-600">
                                                {format!("Region: {}", region_name)}
                                            </div>
                                        }
                                        if let Some(broad_region_name) = &place.broad_region_name {
                                            <div class="text-sm text-gray-500">
                                                {format!("Broad Region: {}", broad_region_name)}
                                            </div>
                                        }
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    </div>
                }
            </div>
            if props.is_set {
                <div class="mt-2 p-2 bg-blue-50 rounded">
                    {"Selected"}
                </div>
            }
        </div>
    }
}
