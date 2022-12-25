use crate::*;
use itertools::Itertools;
use std::collections::HashMap;

pub fn construct(map: &Vec<Vec<char>>, size: usize) -> Vec<Rc<RefCell<Face>>> {
    let mut faces = Vec::new();

    for y in (0..map.len()).step_by(size) {
        for x in (0..map[y].len()).step_by(size) {
            if map[y][x] == ' ' {
                continue;
            }

            let mut face = Face {
                map: Vec::new(),
                offset: (x, y),
                sides: HashMap::new(),
            };
            for y in face.offset.1..face.offset.1 + size {
                face.map.push(map[y][face.offset.0..face.offset.0 + size].to_vec());
            }

            faces.push(Rc::new(RefCell::new(face)));
        }
    }

    for face in faces.iter() {
        for side in [Right, Bottom, Left, Top] {
            let (adjacent_offset, overflowed) = forward(face.borrow().offset, side, size, usize::MAX);
            if overflowed {
                continue;
            }

            let adjacent = faces.iter().find(|other| other.clone().borrow().offset == adjacent_offset);
            if let Some(adjacent) = adjacent {
                let mut face = face.borrow_mut();
                let edge = rotate(side, true, 2);
                face.sides.insert(side, Edge { face: adjacent.clone(), edge });
            }
        }
    }

    return faces;
}

pub fn glue_planar(faces: &mut Vec<Rc<RefCell<Face>>>) {
    for face in faces.iter() {
        let vertically_aligned_faces: Vec<Rc<RefCell<Face>>> = faces
            .iter()
            .filter(|other| other.borrow().offset.0 == face.borrow().offset.0)
            .sorted_by(|a, b| a.borrow().offset.1.cmp(&b.borrow().offset.1))
            .map(|face| face.clone())
            .collect();
        let horizontally_aligned_faces: Vec<Rc<RefCell<Face>>> = faces
            .iter()
            .filter(|other| other.borrow().offset.1 == face.borrow().offset.1)
            .sorted_by(|a, b| a.borrow().offset.0.cmp(&b.borrow().offset.0))
            .map(|face| face.clone())
            .collect();

        let vertical_position = vertically_aligned_faces.iter().position(|other| other == face).unwrap();
        let horizontal_position = horizontally_aligned_faces.iter().position(|other| other == face).unwrap();

        let to_right = horizontally_aligned_faces[(horizontal_position as i8 + 1).rem_euclid(horizontally_aligned_faces.len() as i8) as usize].clone();
        let to_bottom = vertically_aligned_faces[(vertical_position as i8 + 1).rem_euclid(vertically_aligned_faces.len() as i8) as usize].clone();
        let to_left = horizontally_aligned_faces[(horizontal_position as i8 - 1).rem_euclid(horizontally_aligned_faces.len() as i8) as usize].clone();
        let to_top = vertically_aligned_faces[(vertical_position as i8 - 1).rem_euclid(vertically_aligned_faces.len() as i8) as usize].clone();

        let mut face = face.borrow_mut();

        face.sides.insert(Right, Edge { face: to_right, edge: Left });
        face.sides.insert(Bottom, Edge { face: to_bottom, edge: Top });
        face.sides.insert(Left, Edge { face: to_left, edge: Right });
        face.sides.insert(Top, Edge { face: to_top, edge: Bottom });
    }
}

pub fn glue_cubic(faces: &mut Vec<Rc<RefCell<Face>>>) {
    while !faces.iter().all(|face| face.borrow().sides.len() == 4) {
        for face in faces.iter() {
            for left_side in [Right, Bottom, Left, Top] {
                let right_side = rotate(left_side, true, 1);
                let face = face.borrow();

                if !face.sides.contains_key(&left_side) || !face.sides.contains_key(&right_side) {
                    continue;
                }

                let left_edge = face.sides.get(&left_side).unwrap().clone();
                let right_edge = face.sides.get(&right_side).unwrap().clone();

                let left_side = rotate(left_edge.edge, false, 1);
                let right_side = rotate(right_edge.edge, true, 1);

                let face = right_edge.face.clone();
                let edge = right_side;
                left_edge.face.borrow_mut().sides.insert(left_side, Edge { face, edge });

                let face = left_edge.face.clone();
                let edge = left_side;
                right_edge.face.borrow_mut().sides.insert(right_side, Edge { face, edge });
            }
        }
    }
}
