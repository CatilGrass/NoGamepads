use std::collections::VecDeque;

pub fn convert_deque_to_vec <V: Clone>(deque: &VecDeque<V>) -> Vec<V> {
    let vec_deque_ref = deque;
    let mut vec = Vec::new();
    for item in vec_deque_ref {
        vec.push(item.clone())
    }
    vec
}