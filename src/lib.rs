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
pub struct Fobj {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<Vec<usize>>,
    pub rotation: [f64; 3],
    pub position: [f64; 3],
    pub visible: bool,
    pub id: String,
}

impl Fobj {
    pub fn parse_obj(strpath: &str, pos: [f64; 3], id: String) -> Self {
        let path = Path::new(strpath);
        let file = File::open(&path).expect("Could not open file!");
        let reader = BufReader::new(file);
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices = Vec::new();
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
                    vertices.push(Vertex { position });
                }
                "f" => {
                    let mut ind = Vec::new();
                    for index in segs.iter().skip(1) {
                        ind.push(index.split("/").next().unwrap().parse::<usize>().unwrap());
                    }
                    ind.push(ind[0]);
                    indices.push(ind);
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
        };
    }
}
