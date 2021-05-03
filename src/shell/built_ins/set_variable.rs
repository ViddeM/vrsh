use crate::shell::common::state::State;

pub fn set_variable(key: String, val: String, state: &mut State) {
    state.variables.insert(key, val);
}