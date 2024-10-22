use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

#[derive(Debug)]
struct Vertex {
    position: [f64; 3],
    indices: Vec<usize>,
}

#[derive(Debug)]
pub struct Fobj {
    vertices: Vec<Vertex>,
    rotation: [f64; 3],
    position: [f64; 3],
    visible: bool,
    id: String,
}

impl Fobj {
    pub fn parse_obj(strpath: &str, pos: [f64; 3], id: String) -> Self {
        let path = Path::new(strpath);
        let file = File::open(&path).expect("Could not open file!");
        let reader = BufReader::new(file);
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indexi = 0;
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
                    vertices.push(Vertex {
                        position,
                        indices: Vec::new(),
                    });
                }
                "f" => {
                    for index in segs.iter().skip(1) {
                        vertices[indexi]
                            .indices
                            .push(index.split("/").next().unwrap().parse().unwrap());
                    }
                    indexi += 1;
                }
                _ => {}
            }
        }
        return Self {
            vertices,
            rotation: [0.0; 3],
            position: pos,
            visible: true,
            id,
        };
    }
}
