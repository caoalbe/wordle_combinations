#[derive(Clone)]
pub enum Superposition {
    Known(char),
    Unknown(Vec<char>),
}

impl Superposition {
    pub fn drop_state(&mut self, c: char) {
        if let Superposition::Unknown(vec) = self {
        vec.retain(|&x| x != c);
        if vec.len() == 1 {
            let last = vec.pop().unwrap();
            *self = Superposition::Known(last);
        }
    }
    }
}