use std::thread;
use std::time::Duration;

const I_RADIUS: f32 = 70.0; //radius
const O_RADIUS: f32 = 30.0; //thickness of tube

const LIGHT: [f32; 3] = [1.0, 2.0, -3.0];
const CHARS: &str = ".:-=+*#%@";
const COLOR: [u8; 3] = [161, 128, 37];

const SCALE: f32 = 350.0;

fn main(){
    let mut matrix = make_matrix();
    let normals = find_normals(&mut matrix);
    
    let mut a = 0.0;
    let mut b = 0.04;

    loop{
        print!("\x1B[2J\x1B[1;1H");
        plot(&matrix, &normals, a, b);

        a += 0.04;
        b += 0.03;

        thread::sleep(Duration::from_millis(30));
    }

}

fn make_matrix() -> Vec<[f32; 3]>{
    let mut matrix: Vec<[f32; 3]> = vec![];
    for phi in 0..=360{
        let rad: f32 = (phi as f32).to_radians();
        for theta in 0..=360{
            let rad_t: f32 = (theta as f32).to_radians();

            let point = [(I_RADIUS + O_RADIUS*rad_t.cos())*rad.cos(), 
                        (I_RADIUS + O_RADIUS*rad_t.cos())*rad.sin(),
                        O_RADIUS*rad_t.sin()
            ];

            matrix.push(point);
        }
    }
    matrix
}

fn find_normals(matrix: &mut Vec<[f32; 3]>) -> Vec<[f32; 3]>{
    matrix.sort_by(|a, b| a[0].partial_cmp(&b[0]).unwrap());
    let mut norm: Vec<[f32; 3]> = vec![];
    for point in matrix{
        let dist = (point[0].powi(2) + point[1].powi(2)).sqrt();

        let normal: [f32; 3] = [
            point[0] - I_RADIUS*point[0]/dist,
            point[1] - I_RADIUS*point[1]/dist,
            point[2] - I_RADIUS*point[2]/dist
        ];
        
        norm.push(normal)
    }
    norm
}

fn plot(matrix:& Vec<[f32; 3]>, normals: &Vec<[f32; 3]>, a: f32, b: f32){
    let sin_a = a.sin();
    let cos_a = a.cos();
    let sin_b = b.sin();
    let cos_b = b.cos();

    let ambience = 0.8;
    let contrast = 0.2;

    let z_offset = 2000.0;

    let width = 150;
    let height = 40;

    let mut screen = vec![' '; width*height]; //chars to print
    let mut zbuffer = vec![f32::MAX; width*height]; //keeps track of closest point f32::MIN =
    //smallest possible f32 value

    for (point, normal) in matrix.iter().zip(normals.iter()){ //iterates through normals and matrix
        let x = point[0];
        let y = point[1];
        let z = point[2];

        let y1 = y * cos_a - z * sin_a;
        let z1 = y * sin_a + z * cos_a;
        let x1 = x;

        let x2 = x1 * cos_b - y1 * sin_b;
        let y2 = x1 * sin_b - y1 * cos_b;
        let z2 = z1*1.0;

        let nx = normal[0];
        let ny = normal[1];
        let nz = normal[2];

        let ny1 = ny * cos_a - nz * sin_a;
        let nz1 = ny * sin_a - nz * cos_a;
        let nx1 = nx;

        let nx2 = nx1 * cos_b - ny1 * sin_b;
        let ny2 = nx1 * sin_b - ny1 * cos_b;
        let nz2 = nz1;

        let brightness = ((nx2*LIGHT[0] + ny2*LIGHT[1] + nz2*LIGHT[2]) * contrast + ambience).clamp(0.0, 1.0); //calculates brightness clamp makes it 0 - 1
        
        let inv_z = 1.0/(z2 + z_offset);

        if z2 + z_offset <= 50.0{
            continue;
        }

        let idx = (brightness*(CHARS.len() - 1) as f32) as usize; // maps brightness as an index
        let ch = CHARS.chars().nth(idx).unwrap();

        let screen_x = ((x2)*inv_z*SCALE + width as f32 / 2.0) as isize; //projects it
        let screen_y = ((y2)*inv_z*SCALE*0.5 + height as f32 / 2.0) as isize; //projects it

        if screen_x < 0 || screen_x >= width as isize || screen_y < 0 || screen_y >= height as isize {continue;} // bount checks
        if z2 < -100.0 || z2 > 100.0 {
            continue;
        } //removes arifacts

        let idx = (screen_y as usize) * width + (screen_x as usize); //converts to an index

        let depth = z2 + z_offset;

        if depth < zbuffer[idx]{ //depth test
            zbuffer[idx] = depth;
            screen[idx] = ch;
        }
    }

    for y in 0..height{
        for x in 0..width{
            print!("\x1b[38;2;{};{};{}m{}\x1b[0m",COLOR[0], COLOR[1], COLOR[2], screen[y*width + x]);
        }
        println!();
    }
}
