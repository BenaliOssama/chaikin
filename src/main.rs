use std::process::exit;
use macroquad::prelude::*;

#[macroquad::main("chaikin")]
async fn main() {
    let mut px: Vec<(f32, f32)> = Vec::new();
    let mut step = 0;
    let mut animation_started = false;
    
    let max_steps = 6; 
    let mut smoothy: Vec<Vec<(f32, f32)>> = Vec::new();
    let mut frame_count: u32 = 0; 
    let animation_speed = 60; 

    loop {
        clear_background(BLACK);
        
        let step_text = if animation_started {
            format!("Step: {}/{}", step + 1, max_steps + 1)
        } else {
            format!("Step: 0/{}", max_steps + 1)
        };
        draw_text(&step_text, 10.0, 30.0, 24.0, WHITE);
        
        if is_key_pressed(KeyCode::C) {
            animation_started = false;
            px.clear();
            smoothy.clear();
        }
        
        if is_mouse_button_released(MouseButton::Left) && !animation_started {
            let p1 = mouse_position();
            px.push(p1);
        }
        if !px.is_empty()  {
            for i in 0..px.len() {
                draw_circle_lines(px[i].0, px[i].1, 3.0, 1.0, WHITE);
            }
        }
        if !px.is_empty() && px.len() >= 2 && is_key_released(KeyCode::Enter) {
            for i in 0..px.len() - 1 {
                draw_line(px[i].0, px[i].1, px[i + 1].0, px[i + 1].1, 1.0, RED);
            }
        }

        if is_key_released(KeyCode::Enter) &&  px.len() >= 2  {
            smoothy.clear();
            smoothy.push(px.clone());
            for s in 0..max_steps {
                let prev = &smoothy[s];
                let mut new_px = Vec::new();
                let n = prev.len();
                new_px.push(prev[0]);
                for j in 0..n - 1 {
                    let q = (
                        0.75 * prev[j].0 + 0.25 * prev[j + 1].0,
                        0.75 * prev[j].1 + 0.25 * prev[j + 1].1,
                    );
                    let r = (
                        0.25 * prev[j].0 + 0.75 * prev[j + 1].0,
                        0.25 * prev[j].1 + 0.75 * prev[j + 1].1,
                    );
                    new_px.push(q);
                    new_px.push(r);
                }
                new_px.push(prev[n - 1]);
                smoothy.push(new_px);
            }
            step = 0;
            frame_count = 0;
            animation_started = true;
        }

        if step < smoothy.len() {
            let current = &smoothy[step];
            for i in 0..current.len() - 1 {
                draw_line(current[i].0, current[i].1, current[i + 1].0, current[i + 1].1, 2.0, BLUE);
            }
            frame_count += 1;
            if frame_count >= animation_speed {
                step += 1;
                frame_count = 0;
                if step >= smoothy.len() {
                    step = 0;
                }
            }
        }
        
        if is_key_released(KeyCode::Escape) {
            exit(1);
        }

        next_frame().await;
    }
}
