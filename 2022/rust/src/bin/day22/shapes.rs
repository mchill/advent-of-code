use crate::*;
use std::collections::HashMap;

pub fn get_sample_p1() -> HashMap<char, Face> {
    return HashMap::from([
        (
            'A',
            Face {
                map: Vec::new(),
                offset: (8, 0),
                sides: HashMap::from([
                    (Right, Edge { face: 'A', edge: Left }),
                    (Bottom, Edge { face: 'D', edge: Top }),
                    (Left, Edge { face: 'A', edge: Right }),
                    (Top, Edge { face: 'E', edge: Bottom }),
                ]),
            },
        ),
        (
            'B',
            Face {
                map: Vec::new(),
                offset: (0, 4),
                sides: HashMap::from([
                    (Right, Edge { face: 'C', edge: Left }),
                    (Bottom, Edge { face: 'B', edge: Top }),
                    (Left, Edge { face: 'D', edge: Right }),
                    (Top, Edge { face: 'B', edge: Bottom }),
                ]),
            },
        ),
        (
            'C',
            Face {
                map: Vec::new(),
                offset: (4, 4),
                sides: HashMap::from([
                    (Right, Edge { face: 'D', edge: Left }),
                    (Bottom, Edge { face: 'C', edge: Top }),
                    (Left, Edge { face: 'B', edge: Right }),
                    (Top, Edge { face: 'C', edge: Bottom }),
                ]),
            },
        ),
        (
            'D',
            Face {
                map: Vec::new(),
                offset: (8, 4),
                sides: HashMap::from([
                    (Right, Edge { face: 'B', edge: Left }),
                    (Bottom, Edge { face: 'E', edge: Top }),
                    (Left, Edge { face: 'C', edge: Right }),
                    (Top, Edge { face: 'A', edge: Bottom }),
                ]),
            },
        ),
        (
            'E',
            Face {
                map: Vec::new(),
                offset: (8, 8),
                sides: HashMap::from([
                    (Right, Edge { face: 'F', edge: Left }),
                    (Bottom, Edge { face: 'A', edge: Top }),
                    (Left, Edge { face: 'F', edge: Right }),
                    (Top, Edge { face: 'D', edge: Bottom }),
                ]),
            },
        ),
        (
            'F',
            Face {
                map: Vec::new(),
                offset: (12, 8),
                sides: HashMap::from([
                    (Right, Edge { face: 'E', edge: Left }),
                    (Bottom, Edge { face: 'F', edge: Top }),
                    (Left, Edge { face: 'E', edge: Right }),
                    (Top, Edge { face: 'F', edge: Bottom }),
                ]),
            },
        ),
    ]);
}

pub fn get_sample_p2() -> HashMap<char, Face> {
    return HashMap::from([
        (
            'A',
            Face {
                map: Vec::new(),
                offset: (8, 0),
                sides: HashMap::from([
                    (Right, Edge { face: 'F', edge: Right }),
                    (Bottom, Edge { face: 'D', edge: Top }),
                    (Left, Edge { face: 'C', edge: Top }),
                    (Top, Edge { face: 'B', edge: Top }),
                ]),
            },
        ),
        (
            'B',
            Face {
                map: Vec::new(),
                offset: (0, 4),
                sides: HashMap::from([
                    (Right, Edge { face: 'C', edge: Left }),
                    (Bottom, Edge { face: 'E', edge: Bottom }),
                    (Left, Edge { face: 'F', edge: Bottom }),
                    (Top, Edge { face: 'A', edge: Top }),
                ]),
            },
        ),
        (
            'C',
            Face {
                map: Vec::new(),
                offset: (4, 4),
                sides: HashMap::from([
                    (Right, Edge { face: 'D', edge: Left }),
                    (Bottom, Edge { face: 'E', edge: Left }),
                    (Left, Edge { face: 'B', edge: Right }),
                    (Top, Edge { face: 'A', edge: Left }),
                ]),
            },
        ),
        (
            'D',
            Face {
                map: Vec::new(),
                offset: (8, 4),
                sides: HashMap::from([
                    (Right, Edge { face: 'F', edge: Top }),
                    (Bottom, Edge { face: 'E', edge: Top }),
                    (Left, Edge { face: 'C', edge: Right }),
                    (Top, Edge { face: 'A', edge: Bottom }),
                ]),
            },
        ),
        (
            'E',
            Face {
                map: Vec::new(),
                offset: (8, 8),
                sides: HashMap::from([
                    (Right, Edge { face: 'F', edge: Left }),
                    (Bottom, Edge { face: 'B', edge: Bottom }),
                    (Left, Edge { face: 'C', edge: Bottom }),
                    (Top, Edge { face: 'D', edge: Bottom }),
                ]),
            },
        ),
        (
            'F',
            Face {
                map: Vec::new(),
                offset: (12, 8),
                sides: HashMap::from([
                    (Right, Edge { face: 'A', edge: Right }),
                    (Bottom, Edge { face: 'B', edge: Left }),
                    (Left, Edge { face: 'E', edge: Right }),
                    (Top, Edge { face: 'D', edge: Right }),
                ]),
            },
        ),
    ]);
}

pub fn get_input_p1() -> HashMap<char, Face> {
    return HashMap::from([
        (
            'A',
            Face {
                map: Vec::new(),
                offset: (50, 0),
                sides: HashMap::from([
                    (Right, Edge { face: 'B', edge: Left }),
                    (Bottom, Edge { face: 'C', edge: Top }),
                    (Left, Edge { face: 'B', edge: Right }),
                    (Top, Edge { face: 'E', edge: Bottom }),
                ]),
            },
        ),
        (
            'B',
            Face {
                map: Vec::new(),
                offset: (100, 0),
                sides: HashMap::from([
                    (Right, Edge { face: 'A', edge: Left }),
                    (Bottom, Edge { face: 'B', edge: Top }),
                    (Left, Edge { face: 'A', edge: Right }),
                    (Top, Edge { face: 'B', edge: Bottom }),
                ]),
            },
        ),
        (
            'C',
            Face {
                map: Vec::new(),
                offset: (50, 50),
                sides: HashMap::from([
                    (Right, Edge { face: 'C', edge: Left }),
                    (Bottom, Edge { face: 'E', edge: Top }),
                    (Left, Edge { face: 'C', edge: Right }),
                    (Top, Edge { face: 'A', edge: Bottom }),
                ]),
            },
        ),
        (
            'D',
            Face {
                map: Vec::new(),
                offset: (0, 100),
                sides: HashMap::from([
                    (Right, Edge { face: 'E', edge: Left }),
                    (Bottom, Edge { face: 'F', edge: Top }),
                    (Left, Edge { face: 'E', edge: Right }),
                    (Top, Edge { face: 'F', edge: Bottom }),
                ]),
            },
        ),
        (
            'E',
            Face {
                map: Vec::new(),
                offset: (50, 100),
                sides: HashMap::from([
                    (Right, Edge { face: 'D', edge: Left }),
                    (Bottom, Edge { face: 'A', edge: Top }),
                    (Left, Edge { face: 'D', edge: Right }),
                    (Top, Edge { face: 'C', edge: Bottom }),
                ]),
            },
        ),
        (
            'F',
            Face {
                map: Vec::new(),
                offset: (0, 150),
                sides: HashMap::from([
                    (Right, Edge { face: 'F', edge: Left }),
                    (Bottom, Edge { face: 'D', edge: Top }),
                    (Left, Edge { face: 'F', edge: Right }),
                    (Top, Edge { face: 'D', edge: Bottom }),
                ]),
            },
        ),
    ]);
}

pub fn get_input_p2() -> HashMap<char, Face> {
    return HashMap::from([
        (
            'A',
            Face {
                map: Vec::new(),
                offset: (50, 0),
                sides: HashMap::from([
                    (Right, Edge { face: 'B', edge: Left }),
                    (Bottom, Edge { face: 'C', edge: Top }),
                    (Left, Edge { face: 'D', edge: Left }),
                    (Top, Edge { face: 'F', edge: Left }),
                ]),
            },
        ),
        (
            'B',
            Face {
                map: Vec::new(),
                offset: (100, 0),
                sides: HashMap::from([
                    (Right, Edge { face: 'E', edge: Right }),
                    (Bottom, Edge { face: 'C', edge: Right }),
                    (Left, Edge { face: 'A', edge: Right }),
                    (Top, Edge { face: 'F', edge: Bottom }),
                ]),
            },
        ),
        (
            'C',
            Face {
                map: Vec::new(),
                offset: (50, 50),
                sides: HashMap::from([
                    (Right, Edge { face: 'B', edge: Bottom }),
                    (Bottom, Edge { face: 'E', edge: Top }),
                    (Left, Edge { face: 'D', edge: Top }),
                    (Top, Edge { face: 'A', edge: Bottom }),
                ]),
            },
        ),
        (
            'D',
            Face {
                map: Vec::new(),
                offset: (0, 100),
                sides: HashMap::from([
                    (Right, Edge { face: 'E', edge: Left }),
                    (Bottom, Edge { face: 'F', edge: Top }),
                    (Left, Edge { face: 'A', edge: Left }),
                    (Top, Edge { face: 'C', edge: Left }),
                ]),
            },
        ),
        (
            'E',
            Face {
                map: Vec::new(),
                offset: (50, 100),
                sides: HashMap::from([
                    (Right, Edge { face: 'B', edge: Right }),
                    (Bottom, Edge { face: 'F', edge: Right }),
                    (Left, Edge { face: 'D', edge: Right }),
                    (Top, Edge { face: 'C', edge: Bottom }),
                ]),
            },
        ),
        (
            'F',
            Face {
                map: Vec::new(),
                offset: (0, 150),
                sides: HashMap::from([
                    (Right, Edge { face: 'E', edge: Bottom }),
                    (Bottom, Edge { face: 'B', edge: Top }),
                    (Left, Edge { face: 'A', edge: Top }),
                    (Top, Edge { face: 'D', edge: Bottom }),
                ]),
            },
        ),
    ]);
}
