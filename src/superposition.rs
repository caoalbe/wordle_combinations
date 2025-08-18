#[derive(Clone)]
pub enum Superposition {
    Known(char),
    Unknown(Vec<char>),
}

pub fn superposition_drop_state(c: char, target: &mut Superposition) {
    if let Superposition::Unknown(vec) = target {
        vec.retain(|&x| x != c);
        if vec.len() == 1 {
            let last = vec.pop().unwrap();
            *target = Superposition::Known(last);
        }
    }
}
