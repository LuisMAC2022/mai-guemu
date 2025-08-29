use macroquad::prelude::*;

#[macroquad::main("Mai guemu")]
async fn main() {
    loop{
        clear_background(RED);
        next_frame().await
    }

}
