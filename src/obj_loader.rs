use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::mesh_data::MeshData;

pub fn load_obj(path: &str) -> Result<MeshData, String> {
    let file = File::open(path).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);

    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    for line in reader.lines() {
        let line = line.map_err(|e| e.to_string())?;
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "v" => {
                if parts.len() < 4 {
                    return Err(format!("Invalid vertex line: {}", line));
                }
                let x: f32 = parts[1].parse::<f32>().map_err(|e| e.to_string())?;
                let y: f32 = parts[2].parse::<f32>().map_err(|e| e.to_string())?;
                let z: f32 = parts[3].parse::<f32>().map_err(|e| e.to_string())?;
                vertices.push(x);
                vertices.push(y);
                vertices.push(z);
            }
            "f" => {
                if parts.len() < 4 {
                    return Err(format!("Invalid face line: {}", line));
                }
                // Basic support for triangles only. 
                for i in 1..4 {
                    let vertex_data: Vec<&str> = parts[i].split('/').collect();
                    let vertex_index: usize = vertex_data[0].parse::<usize>().map_err(|e| e.to_string())?;
                    
                    if vertex_index == 0 || vertex_index > vertices.len() / 3 {
                        return Err(format!("Invalid vertex index: {}", vertex_index));
                    }
                    
                    indices.push((vertex_index - 1) as u32);
                }
            }
            _ => {}
        }
    }

    Ok(MeshData::from_data(vertices, indices))
}
