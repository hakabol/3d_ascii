const CHARS: &str = ".:-=+*#%@";


pub fn torus(i_radius: f32, o_radius: f32) -> (Vec<[f32; 3]>, Vec<[f32; 3]>){
    let mut matrix = make_matrix_torus(i_radius, o_radius);
    let torus = (matrix.clone(), find_normals_torus(&mut matrix, i_radius, o_radius));

    torus
}
fn make_matrix_torus(i_radius: f32, o_radius: f32) -> Vec<[f32; 3]>{
    let mut matrix: Vec<[f32; 3]> = vec![];
    for phi in 0..=360{
        let rad: f32 = (phi as f32).to_radians();
        for theta in 0..=360{
            let rad_t: f32 = (theta as f32).to_radians();

            let point = [(i_radius + o_radius*rad_t.cos())*rad.cos(), 
                        (i_radius + o_radius*rad_t.cos())*rad.sin(),
                        o_radius*rad_t.sin()
            ];

            matrix.push(point);
        }
    }
    matrix
}

fn find_normals_torus(matrix: &mut Vec<[f32; 3]> , i_radius: f32, _o_radius: f32) -> Vec<[f32; 3]>{
    let mut norm: Vec<[f32; 3]> = vec![];
    for point in matrix{
        let dist = (point[0].powi(2) + point[1].powi(2)).sqrt();

        let normal: [f32; 3] = [
            point[0] - i_radius*point[0]/dist,
            point[1] - i_radius*point[1]/dist,
            point[2] - i_radius*point[2]/dist
        ];
        let len = (
            normal[0]*normal[0] +
            normal[1]*normal[1] +
            normal[2]*normal[2]
        ).sqrt();

        let normal = [
            normal[0]/len,
            normal[1]/len,
            normal[2]/len
        ];
        
        norm.push(normal)
    }
    norm
}

pub fn cube(size: i32, displace: [f32; 3]) -> (Vec<[f32; 3]>, Vec<[f32; 3]>){
    let mut matrix: Vec<[f32; 3]> = vec![];
    let mut normals: Vec<[f32; 3]> = vec![];

    let dis_x = displace[0];
    let dis_y = displace[1];
    let dis_z = displace[2];

    for width in -size/2..=size/2{
        for height in -size/2..=size/2{
            matrix.push([width as f32 + dis_x, height as f32 + dis_y, size as f32/2.0 + dis_z]);
            normals.push([0.0, 0.0, 1.0]);

            matrix.push([width as f32 + dis_x, height as f32 + dis_y, -size as f32/2.0 + dis_z]);
            normals.push([0.0, 0.0, -1.0]);

            matrix.push([width as f32 + dis_x, size as f32/2.0 + dis_y, height as f32 + dis_z]);
            normals.push([0.0, 1.0, 0.0]);

            matrix.push([width as f32 + dis_x, -size as f32/2.0 + dis_y, height as f32 + dis_z]);
            normals.push([0.0, -1.0, 0.0]);

            matrix.push([size as f32/2.0 + dis_x, height as f32 + dis_y, width as f32 + dis_z]);
            normals.push([1.0, 0.0, 0.0]);

            matrix.push([-size as f32/2.0 + dis_x, height as f32 + dis_y, width as f32 + dis_z]);
            normals.push([-1.0, 0.0, 0.0]);

        }
    }
    (matrix, normals)

}

pub fn rotate(rx: f32, ry:f32, (matrix, normals): &(Vec<[f32; 3]>, Vec<[f32; 3]>)) -> (Vec<[f32; 3]>, Vec<[f32; 3]>) {
    let sin_a = rx.sin();
    let cos_a = rx.cos();
    let sin_b = ry.sin();
    let cos_b = ry.cos();

    let mut new_normals: Vec<[f32; 3]> = vec![];
    let mut new_matrix: Vec<[f32; 3]> = vec![];

    for i in 0..matrix.len(){
        let [x, y, z] = matrix[i];

        let y1 = y * cos_a - z * sin_a;
        let z1 = y * sin_a + z * cos_a;
        let x1 = x;

        let x2 = x1 * cos_b - y1 * sin_b;
        let y2 = x1 * sin_b + y1 * cos_b;
        let z2 = z1*1.0;
        
        new_matrix.push([x2, y2, z2]);

        let [nx, ny, nz] = normals[i];

        let ny1 = ny * cos_a - nz * sin_a;
        let nz1 = ny * sin_a - nz * cos_a;
        let nx1 = nx;

        let nx2 = nx1 * cos_b - ny1 * sin_b;
        let ny2 = nx1 * sin_b + ny1 * cos_b;
        let nz2 = nz1;

        new_normals.push([nx2, ny2, nz2]);
    }

    (new_matrix, new_normals)
}

pub fn displace([dx, dy, dz]: [f32; 3], (matrix, normals): (Vec<[f32; 3]>, Vec<[f32; 3]>)) -> (Vec<[f32; 3]>, Vec<[f32; 3]>){
    let mut new_matrix: Vec<[f32; 3]> = vec![];

    for [x, y, z] in matrix{
        new_matrix.push([x + dx, y + dy, z + dz])
    }

    (new_matrix, normals)
}

pub fn combine(shapes: &[(Vec<[f32; 3]>, Vec<[f32; 3]>)]) -> (Vec<[f32; 3]>, Vec<[f32; 3]>){
    let mut screen: (Vec<[f32; 3]>, Vec<[f32; 3]>) = (vec![], vec![]);

    for shape in shapes{
        for (p, n) in shape.0.iter().zip(shape.1.iter()){
            screen.0.push(*p);
            screen.1.push(*n);
        }
    }

    screen
}

pub fn plot((matrix, normals): &(Vec<[f32; 3]>, Vec<[f32; 3]>), a: f32, b: f32, color: [u8;3], scale: f32, light:[f32; 5]){
    let sin_a = a.sin();
    let cos_a = a.cos();
    let sin_b = b.sin();
    let cos_b = b.cos();

    let ambience = light[4];
    let contrast = light[3];

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
        let y2 = x1 * sin_b + y1 * cos_b;
        let z2 = z1*1.0;

        let nx = normal[0];
        let ny = normal[1];
        let nz = normal[2];

        let ny1 = ny * cos_a - nz * sin_a;
        let nz1 = ny * sin_a - nz * cos_a;
        let nx1 = nx;

        let nx2 = nx1 * cos_b - ny1 * sin_b;
        let ny2 = nx1 * sin_b + ny1 * cos_b;
        let nz2 = nz1;

        let brightness = ((nx2*light[0] + ny2*light[1] + nz2*light[2]) * contrast + ambience).clamp(0.0, 1.0); //calculates brightness clamp makes it 0 - 1
        
        let inv_z = 1.0/(z2 + z_offset);

        if z2 + z_offset <= 50.0{
            continue;
        }

        let idx = (brightness*(CHARS.len() - 1) as f32) as usize; // maps brightness as an index
        let ch = CHARS.chars().nth(idx).unwrap();

        let screen_x = ((x2)*inv_z*scale + width as f32 / 2.0) as isize; //projects it
        let screen_y = ((y2)*inv_z*scale*0.5 + height as f32 / 2.0) as isize; //projects it

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
            print!("\x1b[38;2;{};{};{}m{}\x1b[0m",color[0], color[1], color[2], screen[y*width + x]);
        }
        println!();
    }
}
