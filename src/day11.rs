use std::{collections::HashMap, fmt::Display, fs};

type GalaxyId = u32;
type Pos = (usize, usize);

fn main() {
    let input = fs::read_to_string("input/day11/day11.txt").expect("Unable to read file");
    let mut space_map = read_input(&input);
    println!("{}", space_map);
    for i in 0..space_map.tiles.len() {
        println!("{:?}", space_map.get_row(i));
    }
    space_map.expand();
    println!("{}", space_map);

    let distances = space_map.get_distances();
    // sum all distances
    let part1 = distances.iter().map(|(_, distance)| distance).sum::<u32>() / 2;
    println!("Part 1: {}", part1);
}

fn read_input(input: &str) -> SpaceMap {
    let mut galaxy_id = 0;
    let tiles = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    let tile = match c {
                        '.' => SpaceTile::Empty,
                        _ => {
                            galaxy_id += 1;
                            SpaceTile::GalaxyId(galaxy_id)
                        }
                    };
                    tile
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let id_map = tiles
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(j, tile)| match tile {
                    SpaceTile::GalaxyId(id) => Some((*id, (i, j))),
                    _ => None,
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<HashMap<_, _>>();

    SpaceMap { tiles, id_map }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum SpaceTile {
    Empty,
    GalaxyId(GalaxyId),
}

#[derive(Debug)]
struct SpaceMap {
    tiles: Vec<Vec<SpaceTile>>,
    id_map: HashMap<GalaxyId, Pos>,
}

impl SpaceMap {
    // returns the distance between all pairs of galaxies
    // (galaxy_id1, galaxy_id2) and (galaxy_id2, galaxy_id1) are the same distance
    fn get_distances(&self) -> HashMap<(GalaxyId, GalaxyId), u32> {
        let mut distances = HashMap::new();
        for (id1, pos1) in self.id_map.iter() {
            for (id2, pos2) in self.id_map.iter() {
                if id1 != id2 {
                    let distance = self.distance(*pos1, *pos2);
                    distances.insert((*id1, *id2), distance);
                }
            }
        }
        distances
    }

    // manhattan distance (no diagonals)
    fn distance(&self, pos1: Pos, pos2: Pos) -> u32 {
        let x1 = pos1.0 as isize;
        let y1 = pos1.1 as isize;
        let x2 = pos2.0 as isize;
        let y2 = pos2.1 as isize;
        ((x1 - x2).abs() + (y1 - y2).abs()) as u32
    }

    // duplicate rows and cols
    // that have no galaxies
    fn expand(&mut self) {
        // duplicate rows
        let mut row_index = 0;
        while row_index < self.tiles.len() {
            let row = self.get_row(row_index);
            if row.iter().all(|tile| *tile == SpaceTile::Empty) {
                // println!("inserting row {}", row_index);
                // println!("{:?}", row);
                self.insert_row(row_index, row);
                row_index += 2;
            } else {
                row_index += 1;
            }
        }

        // duplicate cols
        let mut col_index = 0;
        while col_index < self.tiles[0].len() {
            let col = self.get_col(col_index);
            if col.iter().all(|tile| *tile == SpaceTile::Empty) {
                // println!("inserting col {}", col_index);
                // println!("{:?}", col);
                self.insert_col(col_index, col);
                col_index += 2;
            } else {
                col_index += 1;
            }
        }

        // update ids
        self.id_map = self
            .tiles
            .iter()
            .enumerate()
            .map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(|(j, tile)| match tile {
                        SpaceTile::GalaxyId(id) => Some((*id, (i, j))),
                        _ => None,
                    })
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect::<HashMap<_, _>>();
    }

    fn insert_row(&mut self, row_index: usize, row: Vec<SpaceTile>) {
        self.tiles.insert(row_index, row.to_vec());
    }

    fn insert_col(&mut self, col_index: usize, col: Vec<SpaceTile>) {
        self.tiles.iter_mut().enumerate().for_each(|(i, row)| {
            row.insert(col_index, col[i]);
        });
    }

    fn get_row(&self, row: usize) -> Vec<SpaceTile> {
        self.tiles[row].to_vec()
    }

    fn get_col(&self, col: usize) -> Vec<SpaceTile> {
        self.tiles.iter().map(|row| row[col]).collect::<Vec<_>>()
    }
}

impl Display for SpaceMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        self.tiles.iter().for_each(|row| {
            row.iter().for_each(|tile| {
                output.push(match tile {
                    SpaceTile::Empty => '.',
                    SpaceTile::GalaxyId(id) => '#',
                });
            });
            output.push('\n');
        });
        write!(f, "{}", output)
    }
}
