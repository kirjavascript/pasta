static URLCHARS: &str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ-._~";

pub struct UrlHash {
    index: usize,
    max_length: u32,
}

impl UrlHash {
    pub fn new(max_length: u32) -> Self {
        UrlHash {
            index: 0,
            max_length,
        }
    }

    pub fn get(&mut self) -> String {
        let length = URLCHARS.len();
        let mut chars = URLCHARS.chars();
        let mut index = self.index % (usize::pow(length, self.max_length));
        self.index += 1;

        let indices = if index == 0 {
            vec![0]
        } else {
            let mut indices = Vec::new();
            while index > 0 {
                let cur = index % length;
                indices.push(cur);
                index -= cur;
                index /= length;
            }
            indices
        };

        indices
            .iter()
            .map(|i| &URLCHARS[*i..=*i])
            .collect()
    }
}
