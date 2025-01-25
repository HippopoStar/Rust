
// Nested Modules p.179
// To avoid warnings about unused items within an imported module, make them
// visible to other crates by marking them and all enclosing modules as public
mod puzzle;

fn main() {
    println!("Hello, world!");
    let p = puzzle::Puzzle::new();
    p.show();
}
