mod modules;
fn main() {
    println!("Hello, world!");
    let grid = modules::grid::new_grid();

    println!("VAL = {:?}", grid[0][2]);

    let i = modules::shapes::I::new();

    for degree in i.into_iter() {
        println!("{:?}", degree);
    }
}
