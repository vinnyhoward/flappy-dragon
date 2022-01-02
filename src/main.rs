use bracket_lib::prelude::*;

struct State {}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print(1, 1, "Hello, Bracket Terminal!");
    }
}

enum GameMode {
    Menu,
    Playing,
    End
}

fn main() -> BError {
    println!("Hello, world!");
    let context: BTerm = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;

    main_loop(context, State{})
}
