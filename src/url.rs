static URLCHARS: &str = "mDctrCgv7BJoNhdOWRlesYI28FfKEnx1uP0pGzb9jQUaH6-iLV_4Z.5kqTSM3ywAX~";

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

    pub fn next(&mut self) -> String {
        let index = self.index;
        self.index += 1;
        Self::get(index, self.max_length)
    }

    fn get(index: usize, max_length: u32) -> String {
        let length = URLCHARS.len();
        let mut index = index % length.pow(max_length);

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
