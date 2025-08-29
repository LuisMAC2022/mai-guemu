use macroquad::prelude::*;

#[macroquad::main("Mai guemu")]
async fn main() {
    loop{
        clear_background(DARKBLUE);
        next_frame().await
    }

}
