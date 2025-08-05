use std::process::exit;
use macroquad::prelude::*;

#[macroquad::main("MyGame")]
async fn main() {
    let mut px: Vec<(f32, f32)> = Vec::new();
    let mut step = 0;
    let max_steps = 7;
    let mut smoothy: Vec<Vec<(f32, f32)>> = Vec::new();
    let mut frame_count: u32 = 0; 

    loop {
        if is_mouse_button_released(MouseButton::Left) {
            let p1 = mouse_position();
            println!("{:?}", p1);
            px.push(p1);
        }

        clear_background(BLACK);

        // if !px.is_empty() {
        //     for i in 0..px.len() - 1 {
        //         draw_line(px[i].0, px[i].1, px[i + 1].0, px[i + 1].1, 1.0, RED);
        //     }
        // }

        if is_key_released(KeyCode::Enter) {
            
            smoothy.clear();
            smoothy.push(px.clone());

            for s in 0..max_steps {
                if s == 0 {
                    first(&px);
                } else {

                    let prev = &smoothy[s - 1];
                    let mut new_px = Vec::new();
                    for j in 0..prev.len() - 1 {
                        let q = (
                            (3.0 / 4.0) * prev[j].0 + (1.0 / 4.0) * prev[j + 1].0,
                            (3.0 / 4.0) * prev[j].1 + (1.0 / 4.0) * prev[j + 1].1,
                        );
                        let r = (
                            (1.0 / 4.0) * prev[j].0 + (3.0 / 4.0) * prev[j + 1].0,
                            (1.0 / 4.0) * prev[j].1 + (3.0 / 4.0) * prev[j + 1].1,
                        );
                        new_px.push(q);
                        new_px.push(r);
                    }
                    smoothy.push(new_px);
                }
            }
            step = 1;
            frame_count = 0;
        }

        if step > 0 && step < smoothy.len() {
            let current = &smoothy[step];
            for i in 0..current.len() - 1 {
                draw_line(current[i].0, current[i].1, current[i + 1].0, current[i + 1].1, 2.0, BLUE);
            }

            frame_count += 1;
            if frame_count % 100 == 0 {
                step += 1;
                if step > max_steps {
                    step = max_steps;
                }
            }
        }

        if is_key_released(KeyCode::Escape) {
            exit(1);
        }

        next_frame().await;
    }
}

fn first(px: &[(f32, f32)]) {
    for i in 0..px.len() - 1 {
        draw_line(px[i].0, px[i].1, px[i + 1].0, px[i + 1].1, 1.0, RED);
    }
}
