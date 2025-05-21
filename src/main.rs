mod helpers;
use helpers::*;
use macroquad::{prelude::*};

#[macroquad::main("Chaikin")]
async fn main() {
    let mut circles: Vec<Vec2> = Vec::new();
    let mut selected_circle: Option<usize> = None;
    let circle_radius = 3.0;
    let mut paint = false;
    let mut anim_num = 0;
    let mut counter = 0;
    
    loop {
        clear_background(BLACK);
        
        if paint {
            let mut new_ps = circles.clone();

            for _ in 0..anim_num {
                new_ps = chaikin(&new_ps);
            }
                
            for (i, c) in new_ps.iter().enumerate() {
                if i+1 == new_ps.len() {
                    break;
                }
                
                draw_line(c.x, c.y, new_ps[i+1].x, new_ps[i+1].y, 2.0, BLUE);
            }

            if counter == 20 {
                counter = 0;
                if anim_num != 7 {
                    anim_num += 1;
                } else {
                    anim_num = 0;
                }
            } else {
                counter += 1;
            }
        }

        if is_key_pressed(KeyCode::Enter) {
            paint = true;
        }

        if is_key_pressed(KeyCode::Space) {
            counter = 0;
            anim_num = 0;
            paint = false;
            circles.clear();
        }

        let mouse_pos = mouse_position();
        
        if is_mouse_button_pressed(MouseButton::Left) && !paint {
            let mut clicked_on_circle = false;
            
            for (i, &pos) in circles.iter().enumerate() {
                if distance(mouse_pos, pos) < circle_radius {
                    selected_circle = Some(i);
                    clicked_on_circle = true;
                    break;
                }
            }
            
            if !clicked_on_circle {
                circles.push(vec2(mouse_pos.0, mouse_pos.1));
                selected_circle = Some(circles.len() - 1);
            }
        }
        
        if is_mouse_button_pressed(MouseButton::Right) {
            let mut circle_to_remove = None;
            
            for (i, &pos) in circles.iter().enumerate() {
                if distance(mouse_pos, pos) < circle_radius {
                    circle_to_remove = Some(i);
                    break;
                }
            }
            
            if let Some(index) = circle_to_remove {
                circles.remove(index);
                selected_circle = None;
            }
        }
        
        if let Some(index) = selected_circle {
            if is_mouse_button_down(MouseButton::Left) {
                circles[index] = vec2(mouse_pos.0, mouse_pos.1);
            } else {
                selected_circle = None;
            }
        }
        
        for (i, &pos) in circles.iter().enumerate() {
            let color = if Some(i) == selected_circle { BLUE } else { WHITE };
            let size = if Some(i) == selected_circle { 5.0 } else { circle_radius };
            draw_circle_lines(pos.x, pos.y, size, 1.0, color);
        }
                
        next_frame().await;
    }
}

 