use components::port_search::PortSearch;
use db::table::init_db;
use models::voyage::Voyage;
use rusqlite::Connection;
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    #[cfg(target_arch = "wasm32")]
    wasm_logger::init(wasm_logger::Config::default());

    let db_conn = Rc::new(Connection::open_in_memory().unwrap());
    let db_state = use_state(|| None::<Rc<Connection>>);
    let voyages = use_state(Vec::new);

    // Initialize database
    {
        let db_state = db_state.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                match init_db(db_conn.clone()) {
                    Ok(_) => db_state.set(Some(db_conn.clone())),
                    Err(e) => log::error!("Failed to initialize database: {}", e),
                }
            });
        });
    }

    let on_voyages_found = {
        let voyages = voyages.clone();
        Callback::from(move |new_voyages: Vec<Voyage>| {
            voyages.set(new_voyages);
        })
    };

    html! {
        <div class="container mx-auto p-4">
            <div class="max-w-4xl mx-auto">
                if let Some(database) = &*db_state {
                        <PortSearch
                            database={database.clone()}
                            on_voyages_found={on_voyages_found.clone()}
                        />
                } else {
                    <div class="text-center p-4">
                        {"Initializing database..."}
                    </div>
                }
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
