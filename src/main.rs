mod ascii_3d;

use std::process::Command;

use std::thread;
use std::time::Duration;

fn main(){
    Command::new("clear").status().unwrap();

    let mut a = 0.0;
    let mut b = 0.0;

    let torus = ascii_3d::torus(100.0, 30.0);

    loop{
        ascii_3d::plot(&torus, a, b, [255, 255, 255], 250.0, [0.5, 0.5, -1.0]);

        a += 0.03;
        b += 0.04;

        thread::sleep(Duration::from_millis(20));

        Command::new("clear").status().unwrap();
    }
}
