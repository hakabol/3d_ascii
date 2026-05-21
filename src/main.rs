mod ascii_3d;

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{enable_raw_mode, disable_raw_mode},
};

use std::thread;
use std::time::Duration;

fn main(){
    println!("\x1b[?25l");
    thread::sleep(Duration::from_millis(500));
    print!("\x1b[2J\x1b[H");

    let mut a = 0.0;
    let mut b = 0.0;

    let mut y = 30.0;

    let mut vel = 0.0;

    enable_raw_mode().unwrap();

    loop{
        //let torus = ascii_3d::torus(100.0, 30.0);
        let mut cube = ascii_3d::cube(100, [0.0, 0.0, 0.0]);
        cube = ascii_3d::rotate(a, b, &cube);

        let mut screen = ascii_3d::combine(&[cube]);
        screen = ascii_3d::rotate(10.0, 0.0, &screen);
        screen = ascii_3d::displace([0.0, 0.0, y], screen);

        ascii_3d::plot(&screen, 0.0, 0.0, [255, 255, 255], 250.0, [0.5, 0.5, -1.0, 0.5, 0.5]);

        a += 0.0;
        b += 0.0;
        y -= vel;

        let exit = get_input(&mut vel);

        if exit{
            disable_raw_mode().unwrap();
            break;
        }


        thread::sleep(Duration::from_millis(11));

        print!("\x1b[2J\x1b[H");
    }
}

fn get_input(vel_y: &mut f32) -> bool{

    let mut ans = false;
    if event::poll(Duration::from_millis(1)).unwrap(){
        if let Event::Key(key) = event::read().unwrap(){
            match key.code {
                KeyCode::Esc => ans = true,

                KeyCode::Char('w') => *vel_y += 4.0,
                KeyCode::Char('s') => *vel_y -= 4.0,

                _ => {}
            }
        }
    }

    if *vel_y > 0.0 {
        *vel_y -= 3.0;
    }

    else if *vel_y < 0.0 {
        *vel_y += 3.0;
    }

    *vel_y = vel_y.clamp(-10.0, 10.0);


    ans
}
