use crate::solution::Solution;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    Empty,
    Paper,
}

#[derive(Clone, Debug)]
struct Grid<T> {
    grid: Vec<T>,
    w: usize,
    h: usize,
}

impl Grid<Tile> {
    fn from_input(input: &str) -> Self {
        let (first_line, _) = input.split_once("\n").unwrap();
        let w = first_line.len();

        let cap = input.len() - w + 1;
        let mut grid = Vec::with_capacity(cap);
        grid.extend(input.chars().filter_map(|c| match c {
            '.' => Some(Tile::Empty),
            '@' => Some(Tile::Paper),
            '\n' => None,
            _ => panic!("unexpected char: {c}"),
        }));

        let h = grid.len() / w;

        Self { grid, w, h }
    }

    fn tile_is_accessible(&self, coords: Coords) -> bool {
        let adjacent_paper = self
            .surroundings(coords)
            .filter(|s| *s == Tile::Paper)
            .count();
        adjacent_paper < 4
    }

    fn count_accessible(&self) -> usize {
        self.tiles()
            .filter(|(coords, t)| *t == Tile::Paper && self.tile_is_accessible(*coords))
            .count()
    }

    fn remove_accessible(&mut self) -> usize {
        let mut removed = 0;

        for i in 0..self.grid.len() {
            let coords = self.i_to_coords(i);

            // SAFETY: `i` is less than `self.grid.len()`
            let t = unsafe { self.grid.get_unchecked(i) };

            if *t == Tile::Paper && self.tile_is_accessible(coords) {
                // SAFETY: `i` is elss than `self.grid.len()`
                unsafe { *self.grid.get_unchecked_mut(i) = Tile::Empty };

                removed += 1;
            }
        }

        removed
    }
}

impl<T> Grid<T> {
    /// SAFETY: the coordinates must be in-bounds
    unsafe fn get_unchecked(&self, coords: Coords) -> &T {
        let i = self.coords_to_i(coords);
        unsafe { self.grid.get_unchecked(i) }
    }

    fn tiles(&self) -> impl Iterator<Item = (Coords, T)>
    where
        T: Copy,
    {
        self.grid.iter().copied().enumerate().map(|(i, t)| {
            let coords = self.i_to_coords(i);
            (coords, t)
        })
    }

    fn surroundings_coords(&self, coords: Coords) -> impl Iterator<Item = Coords> {
        assert!(self.coords_to_i(coords) < self.grid.len());
        [
            self.row_sub(coords).and_then(|coords| self.col_sub(coords)), // NW
            self.row_sub(coords),                                         // N
            self.row_sub(coords).and_then(|coords| self.col_add(coords)), // NE
            self.col_add(coords),                                         // E
            self.col_sub(coords),                                         // W
            self.row_add(coords).and_then(|coords| self.col_sub(coords)), // SW
            self.row_add(coords),                                         // S
            self.row_add(coords).and_then(|coords| self.col_add(coords)), // SE
        ]
        .into_iter()
        .flatten()
    }

    fn surroundings(&self, coords: Coords) -> impl Iterator<Item = T>
    where
        T: Copy,
    {
        self.surroundings_coords(coords).map(|c| {
            // SAFETY: the coords are guaranteed to be in-bounds
            unsafe { *self.get_unchecked(c) }
        })
    }

    fn i_to_coords(&self, i: usize) -> Coords {
        debug_assert!(i < self.grid.len());

        let r = i / self.w;
        let c = i - r * self.w;

        Coords(r, c)
    }

    fn coords_to_i(&self, Coords(r, c): Coords) -> usize {
        r * self.w + c
    }

    fn row_sub(&self, Coords(r, c): Coords) -> Option<Coords> {
        debug_assert!(r < self.h);
        r.checked_sub(1).map(|r| Coords(r, c))
    }

    fn row_add(&self, Coords(r, c): Coords) -> Option<Coords> {
        debug_assert!(r < self.h);
        (r < self.h - 1).then_some(Coords(r + 1, c))
    }

    fn col_sub(&self, Coords(r, c): Coords) -> Option<Coords> {
        debug_assert!(c < self.w);
        c.checked_sub(1).map(|c| Coords(r, c))
    }

    fn col_add(&self, Coords(r, c): Coords) -> Option<Coords> {
        debug_assert!(c < self.w);
        (c < self.w - 1).then_some(Coords(r, c + 1))
    }
}

#[derive(Clone, Copy, Debug)]
struct Coords(usize, usize);

#[derive(Debug)]
pub struct Day04 {
    grid: Grid<Tile>,
}

impl Solution for Day04 {
    fn with_input(input: String) -> Self {
        let grid = Grid::from_input(input.trim());
        Self { grid }
    }

    fn part1(&self) -> String {
        self.grid.count_accessible().to_string()
    }

    fn part2(&self) -> String {
        let mut total_removed = 0;
        let mut grid = self.grid.clone();

        loop {
            let removed = grid.remove_accessible();
            if removed == 0 {
                break;
            } else {
                total_removed += removed;
            }
        }

        total_removed.to_string()
    }
}
