use rltk::{GameState, Rltk};

struct State {}
impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();
        ctx.print(1, 1, "Hello Rustacians");
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Rust Like")
        .build()
        .expect("Could not create context!!");
    let gs = State {};
    rltk::main_loop(context, gs)
}
