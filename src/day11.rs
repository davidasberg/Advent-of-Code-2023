use std::{collections::HashMap, fmt::Display, fs};

type GalaxyId = u32;
type Pos = (usize, usize);

fn main() {
    let input = fs::read_to_string("input/day11/day11.txt").expect("Unable to read file");
    let mut space_map = read_input(&input);
    space_map.expand(1);
    let distances = space_map.get_distances();
    // sum all distances
    let part1 = distances.iter().map(|(_, distance)| distance).sum::<u64>();
    println!("Part 1: {}", part1);

    let mut large_space_map = read_input(&input);
    large_space_map.expand(999_999);
    let distances = large_space_map.get_distances();
    // sum all distances
    let part2 = distances.iter().map(|(_, distance)| distance).sum::<u64>();
    println!("Part 2: {}", part2);
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
                    SpaceTile::GalaxyId(id) => Some(((i, j), *id)),
                    _ => None,
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<HashMap<_, _>>();

    SpaceMap {
        height: tiles.len(),
        width: tiles[0].len(),
        id_map,
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum SpaceTile {
    Empty,
    GalaxyId(GalaxyId),
}

#[derive(Debug)]
struct SpaceMap {
    height: usize,
    width: usize,
    id_map: HashMap<Pos, GalaxyId>,
}

impl SpaceMap {
    // returns the distance between all pairs of galaxies
    // (galaxy_id1, galaxy_id2) and (galaxy_id2, galaxy_id1) are the same distance
    fn get_distances(&self) -> HashMap<(GalaxyId, GalaxyId), u64> {
        let mut distances = HashMap::new();
        for (pos1, galaxy_id1) in self.id_map.iter() {
            for (pos2, galaxy_id2) in self.id_map.iter() {
                if pos1 != pos2 {
                    let key = if galaxy_id1 < galaxy_id2 {
                        (*galaxy_id1, *galaxy_id2)
                    } else {
                        (*galaxy_id2, *galaxy_id1)
                    };

                    if distances.contains_key(&key) {
                        continue;
                    }

                    let distance = self.distance(*pos1, *pos2);
                    distances.insert(key, distance);
                }
            }
        }
        distances
    }

    // manhattan distance (no diagonals)
    fn distance(&self, pos1: Pos, pos2: Pos) -> u64 {
        let x1 = pos1.0 as isize;
        let y1 = pos1.1 as isize;
        let x2 = pos2.0 as isize;
        let y2 = pos2.1 as isize;
        ((x1 - x2).abs() + (y1 - y2).abs()) as u64
    }

    // duplicate rows and cols
    // that have no galaxies
    fn expand(&mut self, expansion_size: usize) {
        let mut added_rows: Vec<(usize, usize)> = Vec::new();
        let mut added_cols: Vec<(usize, usize)> = Vec::new();

        for row_id in 0..self.height {
            let row = self.get_row(row_id);
            if row.iter().all(|tile| *tile == SpaceTile::Empty) {
                added_rows.push((row_id, expansion_size));
            }
        }

        for col_id in 0..self.width {
            let col = self.get_col(col_id);
            if col.iter().all(|tile| *tile == SpaceTile::Empty) {
                added_cols.push((col_id, expansion_size));
            }
        }

        // offset each galaxy by the number of rows/cols added before it
        let mut new_map = HashMap::new();
        for (galaxy_pos, galaxy_id) in self.id_map.iter_mut() {
            let mut offset = (0, 0);
            for row_id in &added_rows {
                let (row_id, expansion_size) = row_id;
                if galaxy_pos.0 >= *row_id {
                    offset.0 += expansion_size;
                }
            }
            for col_id in &added_cols {
                let (col_id, expansion_size) = col_id;
                if galaxy_pos.1 >= *col_id {
                    offset.1 += expansion_size;
                }
            }
            let new_pos = (galaxy_pos.0 + offset.0, galaxy_pos.1 + offset.1);
            new_map.insert(new_pos, *galaxy_id);
        }
        self.id_map = new_map;
    }

    fn get_row(&self, row: usize) -> Vec<SpaceTile> {
        (0..self.width)
            .map(|col| {
                let tile = self.id_map.get(&(row, col));
                match tile {
                    Some(id) => SpaceTile::GalaxyId(*id),
                    None => SpaceTile::Empty,
                }
            })
            .collect::<Vec<_>>()
    }

    fn get_col(&self, col: usize) -> Vec<SpaceTile> {
        (0..self.height)
            .map(|row| {
                let tile = self.id_map.get(&(row, col));
                match tile {
                    Some(id) => SpaceTile::GalaxyId(*id),
                    None => SpaceTile::Empty,
                }
            })
            .collect::<Vec<_>>()
    }
}

impl Display for SpaceMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.height {
            for col in 0..self.width {
                let tile = self.id_map.get(&(row, col));
                let c = match tile {
                    Some(id) => "#".to_string(),
                    None => ".".to_string(),
                };
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
