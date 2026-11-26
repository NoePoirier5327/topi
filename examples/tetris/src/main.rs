mod app;

fn main() {
    topi::App::new(
        "Example - Tetris",
        720, 640, 
        app::App::new()
    ).run();
}
