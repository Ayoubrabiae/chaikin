use macroquad::{prelude::*};

#[macroquad::main("Chaikin")]
async fn main() {
    let mut circles: Vec<Vec2> = Vec::new(); // Store positions of circles created by clicks
    let mut selected_circle: Option<usize> = None;
    let circle_radius = 3.0;
    let mut paint = false;
    
    loop {
        clear_background(BLACK);
        
        if paint {
            let mut new_ps = circles.clone();

            for _ in 0..7 {
                new_ps = chaikin(&new_ps);
            }

            for (i, c) in new_ps.iter().enumerate() {
                // println!("{:?}", c);
                if i+1 == new_ps.len() {
                    break;
                }

                draw_line(c.x, c.y, new_ps[i+1].x, new_ps[i+1].y, 2.0, BLUE);
            }


        }

        if is_key_pressed(KeyCode::Enter) {
            paint = true;
        }

        if is_key_pressed(KeyCode::Space) {
            paint = false;
            circles.clear();
        }

        // Get current mouse position
        let mouse_pos = mouse_position();
        
        // Handle left mouse button click (create new circle)
        if is_mouse_button_pressed(MouseButton::Left) && !paint {
            // Check if we're clicking on an existing circle
            let mut clicked_on_circle = false;
            
            for (i, &pos) in circles.iter().enumerate() {
                if distance(mouse_pos, pos) < circle_radius {
                    selected_circle = Some(i);
                    clicked_on_circle = true;
                    break;
                }
            }
            
            // If not clicking on a circle, create a new one
            if !clicked_on_circle {
                circles.push(vec2(mouse_pos.0, mouse_pos.1));
                selected_circle = Some(circles.len() - 1);
            }
        }
        
        // Handle right mouse button click (delete circle)
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
        
        // If a circle is selected and left mouse button is down, move it
        if let Some(index) = selected_circle {
            if is_mouse_button_down(MouseButton::Left) {
                circles[index] = vec2(mouse_pos.0, mouse_pos.1);
            } else {
                // Release selection when button is released
                selected_circle = None;
            }
        }
        
        // Draw all circles
        for (i, &pos) in circles.iter().enumerate() {
            // Draw selected circles in a different color
            let color = if Some(i) == selected_circle { BLUE } else { WHITE };
            let size = if Some(i) == selected_circle { 5.0 } else { circle_radius };
            draw_circle_lines(pos.x, pos.y, size, 1.0, color);
        }
                
        next_frame().await
    }
}

// Helper function to calculate distance between two points
fn distance(point1: (f32, f32), point2: Vec2) -> f32 {
    let dx = point1.0 - point2.x;
    let dy = point1.1 - point2.y;
    (dx * dx + dy * dy).sqrt()
}

fn chaikin(points: &Vec<Vec2>) -> Vec<Vec2> {
    let mut new_points: Vec<Vec2> = Vec::new();

    for (i, c) in points.iter().enumerate() {
        if i+1 == points.len() {
            break;
        }

        let p0 = c;
        let p1 = points[i+1];
        let q = vec2(0.75*p0[0]+0.25*p1[0], 0.75*p0[1]+0.25*p1[1]);
        let r = vec2(0.25*p0[0]+0.75*p1[0], 0.25*p0[1]+0.75*p1[1]);

        // if i % 2 == 0 {
        //     new_points.push(p0.clone());
        // } else {
        //     new_points.push(p1);
        // }

        new_points.push(q);
        new_points.push(r);
    }

    new_points
}