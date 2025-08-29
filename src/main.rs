use macroquad::prelude::*;

#[macroquad::main("Mai guemu")]
async fn main() {
    loop{
        clear_background(DARKPURPLE);
        next_frame().await
    }

}
