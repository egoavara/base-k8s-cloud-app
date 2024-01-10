use std::collections::HashSet;

pub struct GuardState {
    cached: HashSet<(String, String)>,
}