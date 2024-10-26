use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

#[derive(Debug)]
pub struct Vertex {
    pub position: [f64; 3],
}

#[derive(Debug)]
pub struct Face {
    pub indices: Vec<usize>,
    pub color: [u8; 4],
}

#[derive(Debug)]
pub struct Fobj {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<Face>,
    pub rotation: [f64; 3],
    pub position: [f64; 3],
    pub visible: bool,
    pub id: String,
    /// Width, height
    pub bounding_box: [f64; 2],
}

impl Fobj {
    pub fn parse_obj(strpath: &str, mtpath: &str, pos: [f64; 3], id: String) -> Self {
        let path = Path::new(strpath);
        let file = File::open(&path).expect("Could not open file!");
        let reader = BufReader::new(file);
        let mtl_path = Path::new(mtpath);
        let mtl_file = File::open(&mtl_path).expect("Could not open file!");
        let mtl_reader = BufReader::new(mtl_file);
        let mut colors = Vec::new();
        for line in mtl_reader.lines() {
            let l = line.unwrap();
            let segs: Vec<&str> = l.split_whitespace().collect();
            if segs.len() == 0 {
                continue;
            }
            match segs[0] {
                "Kd" => {
                    let r = (segs[1].parse::<f64>().unwrap() * 255.0) as u8;
                    let g = (segs[2].parse::<f64>().unwrap() * 255.0) as u8;
                    let b = (segs[3].parse::<f64>().unwrap() * 255.0) as u8;
                    colors.push([r, g, b, 255]);
                }
                _ => {}
            }
        }
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices = Vec::new();
        let mut max_x = 0.0;
        let mut max_y = 0.0;
        let mut cur_color = [255; 4];
        for line in reader.lines() {
            let l = line.unwrap();
            let segs: Vec<&str> = l.split_whitespace().collect();
            match segs[0] {
                "v" => {
                    let position: [f64; 3] = [
                        segs[1].parse::<f64>().unwrap() + pos[0],
                        segs[2].parse::<f64>().unwrap() + pos[1],
                        segs[3].parse::<f64>().unwrap() + pos[2],
                    ];
                    let cur_x = (position[0] - pos[0]).abs();
                    let cur_y = (position[1] - pos[1]).abs();
                    let cur_z = (position[2] - pos[2]).abs();
                    if cur_x > max_x {
                        max_x = cur_x;
                    } else if cur_y > max_x {
                        max_x = cur_y;
                    }
                    if cur_z > max_y {
                        max_y = cur_y;
                    }
                    vertices.push(Vertex { position });
                }
                "f" => {
                    let mut ind = Vec::new();
                    for index in segs.iter().skip(1) {
                        ind.push(index.split("//").next().unwrap().parse::<usize>().unwrap());
                    }
                    ind.push(ind[0]);
                    indices.push(Face {
                        indices: ind,
                        color: cur_color,
                    });
                }
                "usemtl" => {
                    let mtl: Vec<&str> = segs[1].split('t').collect();
                    let index = mtl[1].parse::<usize>().unwrap();
                    if index < colors.len() {
                        cur_color = colors[index];
                    }
                }
                _ => {}
            }
        }
        return Self {
            vertices,
            indices,
            rotation: [0.0; 3],
            position: pos,
            visible: true,
            id,
            bounding_box: [max_x, max_y],
        };
    }
}
