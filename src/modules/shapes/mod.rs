use std::vec;

pub enum shapes {
    I(I),
}

pub struct I {
    pub degree_0: [[bool; 4]; 4],
    pub degree_90: [[bool; 4]; 4],
    pub degree_180: [[bool; 4]; 4],
    pub degree_270: [[bool; 4]; 4],
    pub offset_x: i32,
    pub offset_y: i32,
}

impl I {
    pub fn new() -> Self {
        I {
            degree_0: [
                [false, false, false, false],
                [true, true, true, true],
                [false, false, false, false],
                [false, false, false, false],
            ],
            degree_90: [
                [false, false, true, false],
                [false, false, true, false],
                [false, false, true, false],
                [false, false, true, false],
            ],
            degree_180: [
                [false, false, false, false],
                [false, false, false, false],
                [true, true, true, true],
                [false, false, false, false],
            ],
            degree_270: [
                [false, true, false, false],
                [false, true, false, false],
                [false, true, false, false],
                [false, true, false, false],
            ],
        }
    }
}

impl IntoIterator for I {
    type Item = [[bool; 4]; 4];
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![
            self.degree_0,
            self.degree_90,
            self.degree_180,
            self.degree_270,
        ]
        .into_iter()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn get_field() {
        fn test(shape: shapes, orientation: String) {
            match shape {
                shapes::I(i) => i[orientation],
            }
        }
    }
}
