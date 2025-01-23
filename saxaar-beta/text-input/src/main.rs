use port_search::{Port, init_db, search_ports_with_region};
use rusqlite::Connection;
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Clone)]
struct DbState {
    conn: Rc<Connection>,
}

#[derive(Properties, PartialEq)]
struct PortListProps {
    ports: Vec<Port>,
}

#[function_component(PortList)]
fn port_list(props: &PortListProps) -> Html {
    if props.ports.is_empty() {
        html! {}
    } else {
        html! {
            <div class="list-group">
                {props.ports.iter().map(|port| {
                    html! {
                        <div class="list-group-item">
                            <h5 class="mb-1">{&port.name}</h5>
                            <small>{&port.specific_region}</small>
                        </div>
                    }
                }).collect::<Html>()}
            </div>
        }
    }
}

#[function_component(App)]
fn app() -> Html {
    let ports = use_state(Vec::new);
    let input_ref = use_node_ref();
    let db_state = use_state(|| None::<DbState>);
    let db_state_in_move = db_state.clone();

    // Initialize database on component mount
    use_effect_with((), move |_| {
        let db_state = db_state_in_move;
        spawn_local(async move {
            match init_db() {
                Ok(conn) => {
                    db_state.set(Some(DbState {
                        conn: Rc::new(conn),
                    }));
                }
                Err(e) => {
                    log::error!("Failed to initialize database: {:?}", e);
                }
            }
        });
    });

    let oninput = {
        let ports = ports.clone();
        let input_ref = input_ref.clone();
        let db_state = db_state.clone();

        Callback::from(move |_| {
            let input = input_ref.cast::<HtmlInputElement>().unwrap();
            let search_term = input.value();

            if search_term.is_empty() {
                ports.set(Vec::new());
                return;
            }

            if let Some(db) = &*db_state {
                let conn = db.conn.clone();
                let ports_state = ports.clone();

                spawn_local(async move {
                    match search_ports_with_region(&conn, &search_term) {
                        Ok(matched_ports) => {
                            ports_state.set(matched_ports);
                        }
                        Err(e) => {
                            log::error!("Search failed: {:?}", e);
                        }
                    }
                });
            }
        })
    };

    html! {
        <div class="container mt-5">
            <div class="row">
                <div class="col-md-6 offset-md-3">
                    <input
                        type="text"
                        class="form-control"
                        placeholder="Search ports..."
                        ref={input_ref}
                        oninput={oninput}
                    />
                    <PortList ports={(*ports).clone()} />
                </div>
            </div>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
