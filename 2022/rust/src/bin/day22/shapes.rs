use crate::*;
use itertools::Itertools;
use std::collections::HashMap;

pub fn construct(map: &Vec<Vec<char>>, size: usize) -> HashMap<char, Face> {
    let mut faces = HashMap::new();
    let mut name = 'A';

    for y in (0..map.len()).step_by(size) {
        for x in (0..map[y].len()).step_by(size) {
            if map[y][x] == ' ' {
                continue;
            }

            let mut face = Face {
                name,
                map: Vec::new(),
                offset: (x, y),
                sides: HashMap::new(),
            };
            for y in face.offset.1..face.offset.1 + size {
                face.map.push(map[y][face.offset.0..face.offset.0 + size].to_vec());
            }

            faces.insert(name, face);
            name = char::from_u32(name as u32 + 1).unwrap();
        }
    }

    for f in ['A', 'B', 'C', 'D', 'E', 'F'] {
        for side in [Right, Bottom, Left, Top] {
            let (adjacent_offset, overflowed) = forward(faces.get(&f).unwrap().offset, side, size, usize::MAX);
            if overflowed {
                continue;
            }

            let adjacent = faces.values().find(|other| other.offset == adjacent_offset).map(|other| other.name);
            if let Some(adjacent) = adjacent {
                faces.entry(f).and_modify(|face| {
                    let edge = rotate(side, true, 2);
                    face.sides.insert(side, Edge { face: adjacent, edge });
                });
            }
        }
    }

    return faces;
}

pub fn glue_planar(faces: &mut HashMap<char, Face>) {
    for f in ['A', 'B', 'C', 'D', 'E', 'F'] {
        let face = faces.get(&f).unwrap();

        let vertically_aligned_faces: Vec<&Face> = faces
            .values()
            .filter(|other| other.offset.0 == face.offset.0)
            .sorted_by(|a, b| a.offset.1.cmp(&b.offset.1))
            .collect();
        let horizontally_aligned_faces: Vec<&Face> = faces
            .values()
            .filter(|other| other.offset.1 == face.offset.1)
            .sorted_by(|a, b| a.offset.0.cmp(&b.offset.0))
            .collect();

        let vertical_position = vertically_aligned_faces.iter().position(|other| *other == face).unwrap();
        let horizontal_position = horizontally_aligned_faces.iter().position(|other| *other == face).unwrap();

        let to_right = horizontally_aligned_faces[(horizontal_position as i8 + 1).rem_euclid(horizontally_aligned_faces.len() as i8) as usize].name;
        let to_bottom = vertically_aligned_faces[(vertical_position as i8 + 1).rem_euclid(vertically_aligned_faces.len() as i8) as usize].name;
        let to_left = horizontally_aligned_faces[(horizontal_position as i8 - 1).rem_euclid(horizontally_aligned_faces.len() as i8) as usize].name;
        let to_top = vertically_aligned_faces[(vertical_position as i8 - 1).rem_euclid(vertically_aligned_faces.len() as i8) as usize].name;

        let face = faces.get_mut(&f).unwrap();

        face.sides.insert(Right, Edge { face: to_right, edge: Left });
        face.sides.insert(Bottom, Edge { face: to_bottom, edge: Top });
        face.sides.insert(Left, Edge { face: to_left, edge: Right });
        face.sides.insert(Top, Edge { face: to_top, edge: Bottom });
    }
}

pub fn glue_cubic(faces: &mut HashMap<char, Face>) {
    while !faces.iter().all(|(_, face)| face.sides.len() == 4) {
        for face in ['A', 'B', 'C', 'D', 'E', 'F'] {
            for left_side in [Right, Bottom, Left, Top] {
                let right_side = rotate(left_side, true, 1);
                let face = faces.get(&face).unwrap();

                if !face.sides.contains_key(&left_side) || !face.sides.contains_key(&right_side) {
                    continue;
                }

                let left_edge = face.sides.get(&left_side).unwrap().clone();
                let right_edge = face.sides.get(&right_side).unwrap().clone();

                let left_side = rotate(left_edge.edge, false, 1);
                let right_side = rotate(right_edge.edge, true, 1);

                faces.entry(left_edge.face).and_modify(|left_face| {
                    let face = right_edge.face;
                    let edge = right_side;
                    left_face.sides.insert(left_side, Edge { face, edge });
                });

                faces.entry(right_edge.face).and_modify(|right_face| {
                    let face = left_edge.face;
                    let edge = left_side;
                    right_face.sides.insert(right_side, Edge { face, edge });
                });
            }
        }
    }
}
