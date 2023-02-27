mod map;
mod prelude {
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub use crate::map::*;
}
use bracket_lib::prelude::*;
use prelude::*;

struct State {}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print(1, 1, "Hello!")
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Rusty Rouge Crawler")
        .build()?;
    main_loop(context, State {})
}
