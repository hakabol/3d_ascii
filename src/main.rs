mod ascii_3d;

use std::process::Command;

use std::thread;
use std::time::Duration;

fn main(){
    Command::new("clear").status().unwrap();

    let mut a = 0.0;
    let mut b = 0.0;

    let torus = ascii_3d::torus(100.0, 30.0);
    let cube = ascii_3d::cube(100, [100.0, 0.0, 100.0]);

    let screen = ascii_3d::combine(&[cube, torus]);

    loop{
        ascii_3d::plot(&screen, a, b, [255, 255, 255], 250.0, [0.5, 0.5, -1.0, 0.5, 0.5]);

        a += 0.03;
        b += 0.04;

        thread::sleep(Duration::from_millis(30));

        Command::new("clear").status().unwrap();
    }
}
