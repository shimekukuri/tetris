use super::shapes::shapes;

pub type Row = [bool; 10];
pub fn new_grid() -> [Row; 20] {
    let arr: [Row; 20] = [
        [
            false, false, false, false, false, false, false, false, false, false,
        ],
        [
            false, false, false, false, false, false, false, false, false, false,
        ],
        [
            false, false, false, false, false, false, false, false, false, false,
        ],
        [
            false, false, false, false, false, false, false, false, false, false,
        ],
        [
            false, false, false, false, false, false, false, false, false, false,
        ],
        [
            false, false, false, false, false, false, false, false, false, false,
        ],
        [
            false, false, false, false, false, false, false, false, false, false,
        ],
        [
            false, false, false, false, false, false, false, false, false, false,
        ],
        [
            false, false, false, false, false, false, false, false, false, false,
        ],
        [
            false, false, false, false, false, false, false, false, false, false,
        ],
        [
            false, false, false, false, false, false, false, false, false, false,
        ],
        [
            false, false, false, false, false, false, false, false, false, false,
        ],
        [
            false, false, false, false, false, false, false, false, false, false,
        ],
        [
            false, false, false, false, false, false, false, false, false, false,
        ],
        [
            false, false, false, false, false, false, false, false, false, false,
        ],
        [
            false, false, false, false, false, false, false, false, false, false,
        ],
        [
            false, false, false, false, false, false, false, false, false, false,
        ],
        [
            false, false, false, false, false, false, false, false, false, false,
        ],
        [
            false, false, false, false, false, false, false, false, false, false,
        ],
        [
            false, false, false, false, false, false, false, false, false, false,
        ],
    ];
    arr
}

pub fn find_base(grid: [Row; 20], col: usize) -> usize {
    let mut top: usize = 0;
    for row in grid {
        if row[col] {
            return top;
        }
        top += 1;
    }
    top
}

pub fn mutate_grid(grid: [Row; 20], shape: shapes, col: usize) {
    let base = find_base(grid, col);
}

//come back to this later
pub fn find_shape_base(grid: [Row; 20], col: usize, shapes: shapes) {
    match shapes {
        shapes::I(i) => {}
    }
}

#[cfg(test)]
mod tests {
    use super::{find_base, new_grid};

    #[test]
    pub fn find_base_test() {
        let mut grid = new_grid();
        grid[4][3] = true;
        let resust = find_base(grid, 3 as usize);
        assert_eq!(resust, 4 as usize);
    }
}
