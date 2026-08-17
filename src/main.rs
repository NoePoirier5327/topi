mod topi;
extern crate sdl2;

fn main() {
    let mut topi = topi::Topi::new("Test", 1000, 1000);
    topi.run();
}
