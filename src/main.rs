mod planet;
mod vec3;

use macroquad::prelude::*;
use planet::Planet;
use vec3::Vec3;

fn window_conf() -> Conf {
    Conf {
        window_title: "Window name".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}
#[macroquad::main(window_conf)]
async fn main() {
    let width: f64 = screen_width() as f64;
    let height: f64 = screen_height() as f64;
    let p1 = Planet::new(
        5.0,
        Vec3::new(width / 2.0, 1.0, 0.0),
        Vec3::new(50.0, 0.0, 0.0),
    );
    let p2 = Planet::new(
        300.0,
        Vec3::new(width / 2.0, height / 2.0, 0.0),
        Vec3::new(0.0, -1.0, 0.0),
    );

    let p3 = Planet::new(
        2.0,
        Vec3::new(width / 2.0, height / 2.0 + 100.0, 0.0),
        Vec3::new(10.0, -1.0, 0.0),
    );
    // init planets
    let mut planets = vec![p1, p2, p3];

    // init mouse status
    let mut button_down_flag = false;
    let mut mouse_before = Vec3::new(0.0, 0.0, 0.0);
    loop {
        clear_background(color_u8!(20, 20, 50, 1));

        // check mouse control
        let (x, y) = mouse_position();
        if is_mouse_button_down(MouseButton::Left) {
            button_down_flag = true;
            mouse_before = Vec3::new(x as f64, y as f64, 0.0);
        } else if button_down_flag {
            // add new planet when mouse up
            let p_pos = Vec3::new(x as f64, y as f64, 0.0);
            let p_vel = &p_pos - &mouse_before;
            let p_new = Planet::new(5.0, p_pos, p_vel);

            planets.push(p_new);
            button_down_flag = false;
        }

        planets.iter().for_each(|p| draw_planet(p));
        update_planets(&mut planets);
        next_frame().await
    }
}

fn update_planets(planets: &mut Vec<Planet>) {
    let num_p = planets.len();
    for i in 0..num_p {
        let mut fs = Vec::<Vec3>::new();
        for j in 0..num_p {
            if i == j {
                continue;
            } else {
                fs.push(planets.get(i).unwrap().calc_gravitation(&planets[j]))
            }
        }
        let f = fs.iter().fold(Vec3::new(0.0, 0.0, 0.0), |sum, x| &sum + x);
        planets[i].next(f);
    }
}

fn draw_planet(p: &Planet) {
    let color = color_u8!(255, 150, 50, 255);
    let r = (1000.0 * p.m).powf(0.33);
    draw_circle(p.position.x as f32, p.position.y as f32, r as f32, color);
    let color_history = Color::new(color.r, color.g, color.b, 0.3);
    let calc_r = |i: usize| 2.0 * (500.0 - i as f32) / 500.0;

    p.position_history
        .iter()
        .enumerate()
        .for_each(|(i, pos)| draw_circle(pos.x as f32, pos.y as f32, calc_r(i), color_history))
}
