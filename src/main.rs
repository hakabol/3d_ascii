use std::process::Command;

const I_RADIUS: f32 = 70.0; //radius
const O_RADIUS: f32 = 30.0; //thickness of tube

const LIGHT: [f32; 3] = [0.0, 1.0, -1.0];
const CHARS: &str = ".:-=+*#%@";

const SCALE: f32 = 0.3;

fn main(){
    Command::new("clear").status().unwrap();
    
    let mut matrix = make_matrix();
    let normals = find_normals(&mut matrix);

    plot(matrix, normals);

}

fn make_matrix() -> Vec<[f32; 3]>{
    let mut matrix: Vec<[f32; 3]> = vec![];
    for phi in (0..=360).step_by(1){
        let rad: f32 = (phi as f32).to_radians();
        for theta in (0..=360).step_by(5){
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

fn plot(matrix: Vec<[f32; 3]>, normals: Vec<[f32; 3]>){
    let ambience = 0.8;
    let contrast = 0.2;

    let vertical_shift = 0.0;
    let horizontal_shift = 0.0;

    let width = 150;
    let height = 40;

    let mut screen = vec![' '; width*height]; //chars to print
    let mut zbuffer = vec![f32::MIN; width*height]; //keeps track of closest point f32::MIN =
    //smallest possible f32 value

    for (point, normal) in matrix.iter().zip(normals.iter()){ //iterates through normals and matrix
        let brightness = ((normal[0]*LIGHT[0] + normal[1]*LIGHT[1] + normal[2]*LIGHT[2]) * contrast + ambience).clamp(0.0, 1.0); //calculates brightness clamp makes it 0 - 1

        let idx = (brightness*(CHARS.len() - 1) as f32) as usize; // maps brightness as an index
        let ch = CHARS.chars().nth(idx).unwrap();

        let screen_x = ((point[0] + horizontal_shift)*SCALE + width as f32 / 2.0) as isize; //projects it
        let screen_y = ((point[1] + vertical_shift)*SCALE*0.5 + height as f32 / 2.0) as isize; //projects it

        if screen_x < 0 || screen_x >= width as isize || screen_y < 0 || screen_y >= height as isize {continue;} // bount checks
        if point[2] < -100.0 || point[2] > 100.0 {
            continue;
        } //removes arifacts

        let idx = (screen_y as usize) * width + (screen_x as usize); //converts to an index

        if point[2] > zbuffer[idx]{ //depth test
            zbuffer[idx] = point[2];
            screen[idx] = ch;
        }
    }

    for y in 0..height{
        for x in 0..width{
            print!("{}", screen[y*width + x]);
        }
        println!();
    }
}
