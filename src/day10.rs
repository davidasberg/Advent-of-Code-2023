use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
    str::FromStr,
};

fn main() {
    let input = fs::read_to_string("input/day10/day10.txt").expect("Unable to read file");
    let mut map = read_input(&input);
    let starting_point = map.starting_point;
    // let part1 = map.farthest_distance_from_starting_point(starting_point);
    // println!("Part 1: {}", part1);

    let part2 = map.interior_area(starting_point);
    println!("Part 2: {}", part2 - 1);
}

fn read_input(input: &str) -> Map {
    let tiles = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<Tile>().unwrap())
                .collect()
        })
        .collect();

    let starting_point = input
        .lines()
        .enumerate()
        .find_map(|(y, line)| {
            line.chars()
                .enumerate()
                .find_map(|(x, c)| if c == 'S' { Some((x, y)) } else { None })
        })
        .unwrap();

    Map {
        tiles,
        starting_point,
    }
}

#[derive(Debug)]
struct Map {
    tiles: Vec<Vec<Tile>>,
    starting_point: (usize, usize),
}

impl Map {
    fn get_all_pipe_distances(
        &mut self,
        starting_point: (usize, usize),
    ) -> HashMap<(usize, usize), i32> {
        // determine type of tile starting point is on
        let starting_tile = determine_tile(
            self.get_tile(starting_point.0 as isize, starting_point.1 as isize - 1),
            self.get_tile(starting_point.0 as isize, starting_point.1 as isize + 1),
            self.get_tile(starting_point.0 as isize - 1, starting_point.1 as isize),
            self.get_tile(starting_point.0 as isize + 1, starting_point.1 as isize),
        );

        self.set_tile(starting_point.0, starting_point.1, starting_tile);

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut distances = HashMap::new();

        visited.insert(starting_point);
        queue.push_back((starting_point, 0));
        distances.insert(starting_point, 0);

        while let Some(((x, y), distance)) = queue.pop_front() {
            let tile = self.get_tile(x as isize, y as isize);
            let possible_neighbours = tile.get_possible_neighbours((x, y));

            for (x, y) in possible_neighbours {
                let neighbour = (x, y);
                let neighbour_tile = self.get_tile(neighbour.0 as isize, neighbour.1 as isize);

                if neighbour_tile == Tile::Ground {
                    continue;
                }

                if visited.contains(&neighbour) {
                    continue;
                }

                visited.insert(neighbour);
                distances.insert(neighbour, distance + 1);
                queue.push_back((neighbour, distance + 1));
            }
        }
        distances
    }

    fn farthest_distance_from_starting_point(&mut self, starting_point: (usize, usize)) -> i32 {
        let pipes = self.get_all_pipe_distances(starting_point);
        *pipes.values().max().unwrap()
    }

    fn get_tile(&self, x: isize, y: isize) -> Tile {
        // if x or y is out of bounds, return ground
        if x < 0 || y < 0 || x >= self.tiles[0].len() as isize || y >= self.tiles.len() as isize {
            return Tile::Ground;
        }

        self.tiles[y as usize][x as usize]
    }

    fn set_tile(&mut self, x: usize, y: usize, tile: Tile) {
        self.tiles[y][x] = tile;
    }

    fn interior_area(&mut self, starting_point: (usize, usize)) -> u32 {
        let pipes = self
            .get_all_pipe_distances(starting_point)
            .iter()
            .map(|(p, _)| *p)
            .collect::<HashSet<_>>();

        // each row should have a multiple of 2 pipes
        // split this into pairs, and take the distance between them
        let mut area = 0;
        for row in 0..self.tiles.len() {
            let mut pipes_in_row = pipes
                .iter()
                .filter(|(_, y)| *y == row)
                .map(|(x, y)| (*x, *y))
                .collect::<Vec<_>>();

            // now filter those that have at most 1 of the pipes next to them (left or right)
            // pipes_in_row = pipes_in_row
            //     .into_iter()
            //     .filter(|p| {
            //         let left = usize::try_from(p.0 as isize - 1);
            //         let right = usize::try_from(p.0 as isize + 1);

            //         let left_is_pipe = left.map(|l| pipes.contains(&(l, p.1))).unwrap_or(false);
            //         let right_is_pipe = right.map(|r| pipes.contains(&(r, p.1))).unwrap_or(false);

            //         if left_is_pipe && right_is_pipe {
            //             return false;
            //         }
            //         true
            //     })
            //     .collect::<Vec<_>>();

            if pipes_in_row.len() == 0 {
                continue;
            }

            if pipes_in_row.len() % 2 != 0 {
                // remove the middle pipe
                let middle = pipes_in_row.len() / 2;
                pipes_in_row.remove(middle);
            }

            let mut pipes_x = pipes_in_row.iter().map(|(x, _)| *x).collect::<Vec<_>>();

            pipes_x.sort();
            // dbg!(&pipes_x);

            for pair in pipes_x.chunks(2) {
                area += pair[1] as u32 - pair[0] as u32 - 1;
            }
        }

        area
    }
}

fn determine_tile(up: Tile, down: Tile, left: Tile, right: Tile) -> Tile {
    // either up and down both face tile,
    // or left and right both face tile
    // or up and right face tile
    // or down and right face tile
    // or down and left face tile
    // or up and left face tile
    use Tile::*;

    // if up and down face tile
    if (up == Tile::Vertical || up == Tile::SouthEast || up == Tile::SouthWest)
        && (down == Tile::Vertical || down == Tile::NorthEast || down == Tile::NorthWest)
    {
        return Vertical;
    }

    // if left and right face tile
    if (left == Tile::Horizontal || left == Tile::NorthEast || left == Tile::SouthEast)
        && (right == Tile::Horizontal || right == Tile::NorthWest || right == Tile::SouthWest)
    {
        return Horizontal;
    }

    // if up and right face tile
    if (up == Tile::Vertical || up == Tile::SouthEast || up == Tile::SouthWest)
        && (right == Tile::Horizontal || right == Tile::NorthWest || right == Tile::SouthWest)
    {
        return NorthEast;
    }

    // if down and right face tile
    if (down == Tile::Vertical || down == Tile::NorthEast || down == Tile::NorthWest)
        && (right == Tile::Horizontal || right == Tile::NorthWest || right == Tile::SouthWest)
    {
        return SouthEast;
    }

    // if down and left face tile
    if (down == Tile::Vertical || down == Tile::NorthEast || down == Tile::NorthWest)
        && (left == Tile::Horizontal || left == Tile::NorthEast || left == Tile::SouthEast)
    {
        return SouthWest;
    }

    // if up and left face tile
    if (up == Tile::Vertical || up == Tile::SouthEast || up == Tile::SouthWest)
        && (left == Tile::Horizontal || left == Tile::NorthEast || left == Tile::SouthEast)
    {
        return NorthWest;
    }

    panic!("Unable to determine tile type")
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
enum Tile {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Ground,
    StartingPoint,
}

impl Tile {
    fn get_possible_neighbours(&self, p: (usize, usize)) -> Vec<(usize, usize)> {
        match self {
            Tile::Vertical => vec![(0, -1), (0, 1)],
            Tile::Horizontal => vec![(-1, 0), (1, 0)],
            Tile::NorthEast => vec![(0, -1), (1, 0)],
            Tile::NorthWest => vec![(0, -1), (-1, 0)],
            Tile::SouthEast => vec![(0, 1), (1, 0)],
            Tile::SouthWest => vec![(0, 1), (-1, 0)],
            Tile::Ground => vec![],
            Tile::StartingPoint => vec![],
        }
        .into_iter()
        // with checked add, we don't have to worry about overflow
        .map(|(dx, dy): (isize, isize)| {
            (
                usize::try_from(p.0 as isize + dx).unwrap_or(0),
                usize::try_from(p.1 as isize + dy).unwrap_or(0),
            )
        })
        .collect()
    }
}

impl FromStr for Tile {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "|" => Ok(Tile::Vertical),
            "-" => Ok(Tile::Horizontal),
            "L" => Ok(Tile::NorthEast),
            "J" => Ok(Tile::NorthWest),
            "7" => Ok(Tile::SouthWest),
            "F" => Ok(Tile::SouthEast),
            "." => Ok(Tile::Ground),
            "S" => Ok(Tile::StartingPoint),
            _ => Err(()),
        }
    }
}
