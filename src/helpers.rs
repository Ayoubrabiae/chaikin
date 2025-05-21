use macroquad::{prelude::{Vec2, vec2}};

pub fn distance(point1: (f32, f32), point2: Vec2) -> f32 {
    let dx = point1.0 - point2.x;
    let dy = point1.1 - point2.y;
    (dx * dx + dy * dy).sqrt()
}

pub fn chaikin(arr: &Vec<Vec2>) -> Vec<Vec2> {
    let mut v : Vec<Vec2> = Vec::new();
    let mut index : usize = 1;
    let arr_size : usize = arr.len() as usize;
    let mut visited = vec2(-1.0, -1.0);
    while index < arr_size {
        let q = vec2(0.75 * arr[index - 1].x  + (0.25 * arr[index].x ), 0.75 * arr[index - 1].y  + (0.25 * arr[index].y ));
        let r = vec2(0.25 * arr[index - 1].x  + (0.75 * arr[index].x ), 0.25 * arr[index - 1].y  + (0.75 * arr[index].y ));
        if visited[0] != arr[index - 1].x || visited[1] != arr[index - 1].y {
            v.push(arr[index - 1]);
        }
        visited = arr[index];
        v.push(q);
        v.push(r);
        index += 1;
    }
    v.push(visited);
    v
}