use crate::{place::Place, voyage::Voyage};
use rusqlite::Connection;
use std::rc::Rc;
use yew::{Callback, Properties};

#[derive(Clone, PartialEq)]
pub struct SearchNode {
    pub state: SearchState,
    pub selected_place: Option<Place>,
    pub input_value: String,
}

// Types for state management
#[derive(Clone, PartialEq)]
pub enum SearchState {
    NotSet,
    Set,
}

#[derive(Clone, PartialEq, Debug)]
pub enum NodeType {
    Origin,
    Destination,
}

// SearchInput Component
#[derive(Properties, PartialEq)]
pub struct SearchInputProps {
    pub node_type: NodeType,
    pub value: String,
    pub on_search: Callback<(String, NodeType)>,
    pub on_select: Callback<(Place, NodeType)>,
    pub suggestions: Vec<Place>,
    pub is_set: bool,
}

impl PartialEq for PortSearchProps {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(self, other)
    }
}

impl std::ops::DerefMut for PortSearchProps {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.database
    }
}

#[derive(Properties)]
pub struct PortSearchProps {
    pub database: Rc<Connection>,
    pub on_voyages_found: Callback<Vec<Voyage>>,
}

impl PortSearchProps {
    pub fn new(database: Rc<Connection>, on_voyages_found: Callback<Vec<Voyage>>) -> Self {
        Self {
            database,
            on_voyages_found,
        }
    }
}

impl std::ops::Deref for PortSearchProps {
    type Target = Rc<Connection>;

    fn deref(&self) -> &Self::Target {
        &self.database
    }
}
