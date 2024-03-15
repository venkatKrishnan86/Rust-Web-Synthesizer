pub mod white_keys;
pub mod black_keys;

use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct KeyProps {
    pub label: char
}