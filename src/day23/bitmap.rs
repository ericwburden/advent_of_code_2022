use anyhow::{bail, Error};
use itertools::Itertools;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl};
use std::str::FromStr;

/// These can be adjusted to adjust the size of the grid
type ChunkType = u64;
const CHUNKS: usize = 3;

trait BitTricks {
    fn low_bit() -> Self;
    fn high_bit() -> Self;
}

impl BitTricks for u8 {
    fn low_bit() -> Self {
        1
    }

    fn high_bit() -> Self {
        128
    }
}

impl BitTricks for u64 {
    fn low_bit() -> Self {
        1
    }

    fn high_bit() -> Self {
        9223372036854775808
    }
}

impl BitTricks for u128 {
    fn low_bit() -> Self {
        1
    }

    fn high_bit() -> Self {
        170141183460469231731687303715884105728
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct GridRow([ChunkType; CHUNKS]);

impl Default for GridRow {
    fn default() -> Self {
        let inner = [<ChunkType>::default(); CHUNKS];
        Self(inner)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct GridRowIdx {
    chunk: usize,
    set_bit: ChunkType,
}

impl From<usize> for GridRowIdx {
    fn from(index: usize) -> Self {
        let chunk = index / <ChunkType>::BITS as usize;
        let offset = index % <ChunkType>::BITS as usize;
        let set_bit = 1 << offset;
        GridRowIdx { chunk, set_bit }
    }
}

impl GridRow {
    fn set(&mut self, GridRowIdx { chunk, set_bit }: GridRowIdx) {
        self.0[chunk] |= set_bit;
    }

    fn is_set(&self, GridRowIdx { chunk, set_bit }: GridRowIdx) -> bool {
        self.0[chunk] & set_bit != 0
    }

    fn indices(&self) -> impl Iterator<Item = GridRowIdx> {
        (0..CHUNKS)
            .cartesian_product(0..(<ChunkType>::BITS as usize))
            .map(|(c, b)| c + b)
            .map(GridRowIdx::from)
    }

    fn offset_right(&mut self) {
        let mut carry = 0;

        // This is a 'bit' complicated by the twin facts that (a) bit indices
        // run from right to left and column indices run from left to right,
        // and (b) we need to account for moving set bits across chunk
        // boundaries. The bitwise operations below take care of (a) by rotating
        // the chunk in the opposite direction from what is intuitive. (b) is
        // taken care of by determining whether the bit that wrapped around is
        // set or not, unsetting it if it is, and carrying that bit forward to
        // the next chunk in sequence.
        for chunk in self.0.iter_mut() {
            *chunk = chunk.rotate_left(1);
            let wrapped_bit = *chunk & ChunkType::low_bit();
            *chunk ^= wrapped_bit;
            *chunk |= carry;
            carry = wrapped_bit;
        }
    }

    fn offset_left(&mut self) {
        let mut carry = 0;

        for chunk in self.0.iter_mut().rev() {
            *chunk = chunk.rotate_right(1);
            let wrapped_bit = *chunk & ChunkType::high_bit();
            *chunk ^= wrapped_bit;
            *chunk |= carry;
            carry = wrapped_bit;
        }
    }
}

impl Display for GridRow {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        for idx in self.indices() {
            let glyph = if self.is_set(idx) { '#' } else { '.' };
            write!(f, "{glyph}")?;
        }
        write!(f, "")
    }
}

impl BitAnd<GridRow> for GridRow {
    type Output = GridRow;

    fn bitand(self, other: GridRow) -> Self::Output {
        let GridRow(mut lhs) = self;
        let GridRow(rhs) = other;
        lhs.iter_mut()
            .zip(rhs.into_iter())
            .for_each(|(l, r)| *l &= r);
        GridRow(lhs)
    }
}

impl BitAndAssign for GridRow {
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}

impl BitOr<GridRow> for GridRow {
    type Output = GridRow;

    fn bitor(self, other: GridRow) -> Self::Output {
        let GridRow(mut lhs) = self;
        let GridRow(rhs) = other;
        lhs.iter_mut()
            .zip(rhs.into_iter())
            .for_each(|(l, r)| *l |= r);
        GridRow(lhs)
    }
}

impl BitOrAssign for GridRow {
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}

impl BitXor<GridRow> for GridRow {
    type Output = GridRow;

    fn bitxor(self, other: GridRow) -> Self::Output {
        let GridRow(mut lhs) = self;
        let GridRow(rhs) = other;
        lhs.iter_mut()
            .zip(rhs.into_iter())
            .for_each(|(l, r)| *l ^= r);
        GridRow(lhs)
    }
}

impl BitXorAssign<GridRow> for GridRow {
    fn bitxor_assign(&mut self, other: GridRow) {
        *self = *self ^ other;
    }
}

impl Not for GridRow {
    type Output = GridRow;

    fn not(self) -> Self::Output {
        let GridRow(mut chunks) = self;
        chunks.iter_mut().for_each(|c| *c = !*c);
        GridRow(chunks)
    }
}

impl FromStr for GridRow {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bit_map_row = GridRow::default();
        for (idx, ch) in s.chars().enumerate() {
            let bit_map_row_idx = GridRowIdx::from(idx);
            match ch {
                '#' => bit_map_row.set(bit_map_row_idx),
                '.' => continue,
                _ => bail!("Input string should only contain '#', '.'!"),
            }
        }
        Ok(bit_map_row)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Grid {
    dims: usize,
    rows: [GridRow; CHUNKS * <ChunkType>::BITS as usize],
}

impl Default for Grid {
    fn default() -> Self {
        let dims = CHUNKS * <ChunkType>::BITS as usize;
        let rows = [GridRow::default(); CHUNKS * <ChunkType>::BITS as usize];
        Self { dims, rows }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct GridIdx {
    row: usize,
    bit: GridRowIdx,
}

impl From<(usize, usize)> for GridIdx {
    fn from(value: (usize, usize)) -> Self {
        let row = value.0;
        let bit = GridRowIdx::from(value.1);
        GridIdx { row, bit }
    }
}

impl Grid {
    fn set(&mut self, GridIdx { row, bit }: GridIdx) {
        self.rows[row].set(bit);
    }

    fn is_set(&self, GridIdx { row, bit }: GridIdx) -> bool {
        self.rows[row].is_set(bit)
    }

    fn indices(&self) -> impl Iterator<Item = GridIdx> {
        (0..self.dims).cartesian_product(0..self.dims).map(GridIdx::from)
    }

    fn offset_right(&mut self) -> Self {
        self.rows.iter_mut().for_each(|r| r.offset_right());
        *self
    }

    fn offset_left(&mut self) -> Self {
        self.rows.iter_mut().for_each(|r| r.offset_left());
        *self
    }

    fn offset_up(&mut self) -> Self {
        self.rows.rotate_left(1);
        if let Some(row) = self.rows.last_mut() {
            *row = GridRow::default();
        }
        *self
    }

    fn offset_down(&mut self) -> Self {
        self.rows.rotate_right(1);
        if let Some(row) = self.rows.first_mut() {
            *row = GridRow::default();
        }
        *self
    }

    fn bounds(&self) -> ((usize, usize), (usize, usize)) {
        let mut min_row = usize::MAX;
        let mut min_col = usize::MAX;
        let mut max_row = usize::MIN;
        let mut max_col = usize::MIN;

        for (row, col) in (0..self.dims).cartesian_product(0..self.dims) {
            let idx = GridIdx::from((row, col));
            if self.is_set(idx) {
                min_row = min_row.min(row);
                min_col = min_col.min(col);
                max_row = max_row.max(row);
                max_col = max_col.max(col);
            }
        }

        ((min_row, min_col), (max_row, max_col))
    }

    fn count_empty_spaces(&self) -> u32 {
        let mut empty_spaces = 0;
        let ((min_row, min_col), (max_row, max_col)) = self.bounds();
        for (row, col) in (0..self.dims).cartesian_product(0..self.dims) {
            if row < min_row || row > max_row || col < min_col || col > max_col {
                continue;
            }
            let idx = GridIdx::from((row, col));
            if !self.is_set(idx) {
                empty_spaces += 1;
            }
        }
        empty_spaces
    }
}

impl FromStr for Grid {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows = s.lines().count();
        let cols = s.lines().next().map(|l| l.len()).unwrap_or_default();
        let mut bit_map = Grid::default();
        let row_offset = (bit_map.dims / 2) - (rows / 2);
        let col_offset = (bit_map.dims / 2) - (cols / 2);

        for (row, line) in s.lines().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                let idx = GridIdx::from((row + row_offset, col + col_offset));
                match ch {
                    '#' => bit_map.set(idx),
                    '.' => continue,
                    _ => bail!("Input string should only contain '#', '.', or newline!"),
                }
            }
        }

        Ok(bit_map)
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let mut current_row = 0;
        for idx in self.indices() {
            if idx.row > current_row {
                writeln!(f)?;
                current_row = idx.row;
            }

            let glyph = if self.is_set(idx) { '#' } else { '.' };
            write!(f, "{glyph}")?;
        }
        write!(f, "")
    }
}

impl BitAnd<Grid> for Grid {
    type Output = Grid;

    fn bitand(self, other: Grid) -> Self::Output {
        let Grid {
            dims: lhs_dims,
            rows: mut lhs_rows,
        } = self;
        let Grid {
            dims: rhs_dims,
            rows: rhs_rows,
        } = other;
        if lhs_dims != rhs_dims {
            panic!("Cannot perform bitwise operations on Grids of different sizes.");
        }
        lhs_rows
            .iter_mut()
            .zip(rhs_rows.into_iter())
            .for_each(|(l, r)| *l &= r);
        Grid {
            dims: lhs_dims,
            rows: lhs_rows,
        }
    }
}

impl BitOr<Grid> for Grid {
    type Output = Grid;

    fn bitor(self, other: Grid) -> Self::Output {
        let Grid {
            dims: lhs_dims,
            rows: mut lhs_rows,
        } = self;
        let Grid {
            dims: rhs_dims,
            rows: rhs_rows,
        } = other;
        if lhs_dims != rhs_dims {
            panic!("Cannot perform bitwise operations on Grids of different sizes.");
        }
        lhs_rows
            .iter_mut()
            .zip(rhs_rows.into_iter())
            .for_each(|(l, r)| *l |= r);
        Grid {
            dims: lhs_dims,
            rows: lhs_rows,
        }
    }
}

impl BitXor<Grid> for Grid {
    type Output = Grid;

    fn bitxor(self, other: Grid) -> Self::Output {
        let Grid {
            dims: lhs_dims,
            rows: mut lhs_rows,
        } = self;
        let Grid {
            dims: rhs_dims,
            rows: rhs_rows,
        } = other;
        if lhs_dims != rhs_dims {
            panic!("Cannot perform bitwise operations on Grids of different sizes.");
        }
        lhs_rows
            .iter_mut()
            .zip(rhs_rows.into_iter())
            .for_each(|(l, r)| *l ^= r);
        Grid {
            dims: lhs_dims,
            rows: lhs_rows,
        }
    }
}

impl Not for Grid {
    type Output = Grid;

    fn not(self) -> Self::Output {
        let Grid { dims, mut rows } = self;
        rows.iter_mut().for_each(|r| *r = !*r);
        Grid { dims, rows }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, Copy)]
struct Rules([Direction; 4]);

impl Rules {
    fn rotate(&mut self) {
        self.0.rotate_left(1);
    }
}

impl Default for Rules {
    fn default() -> Self {
        use Direction::*;
        Self([North, South, West, East])
    }
}

impl IntoIterator for Rules {
    type Item = Direction;
    type IntoIter = std::array::IntoIter<Self::Item, 4>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(Debug, Default)]
struct GridBuilder {
    base: Grid,
    propose_order: Rules,
    north_south_blocked: Option<Grid>,
    east_west_blocked: Option<Grid>,
    stationary: Option<Grid>,
    willing_to_move: Option<Grid>,
    proposed_north: Option<Grid>,
    proposed_south: Option<Grid>,
    proposed_east: Option<Grid>,
    proposed_west: Option<Grid>,
}

impl GridBuilder {
    fn init(base: Grid, propose_order: Rules) -> Self {
        GridBuilder {
            base,
            propose_order,
            ..Default::default()
        }
    }

    fn identify_movers(mut self) -> Self {
        // Generate masks for the spaces around the elves. These spaces will overlap
        // with elves who have neighbors.
        let east_mask = self.base.clone().offset_right();
        let west_mask = self.base.clone().offset_left();
        let north_mask = self.base.clone().offset_up();
        let south_mask = self.base.clone().offset_down();
        let east_west_mask = east_mask | west_mask;
        let north_south_mask = north_mask | south_mask;
        let north_corner_mask = east_west_mask.clone().offset_up();
        let south_corner_mask = east_west_mask.clone().offset_down();
        let corner_mask = north_corner_mask | south_corner_mask;
        let neighbors = east_west_mask | north_south_mask | corner_mask;

        // Retain masks that can be used to determine whether an elf can propose
        // a move to the north/south or east/west, based on the presence of a
        // neighbor in that "general" direction. For example:
        //
        //   base      north_south_blocked  east_west_blocked
        //    .....          .....               ...1.
        //    ...1.          ..111               ..21.
        //    ..2..          .222.               .321.
        //    .3...          333..               .32..
        //    .....          .....               .3...
        //
        // We can determine that `2` cannot move north or south because
        // `north_south_blockeed` will occupy the space to the north and south of
        // the location of `2` in `base`. We can also determine that `2` cannot
        // move east or west because `east_west_blocked` occupies the spaces to
        // the east and west of the location of `2` in `base`. In this example,
        // `1` could move north or east, and `3` could move west or south. All
        // other proposals for `1` and `3` are blocked by `2`.
        self.north_south_blocked = Some(east_west_mask | self.base);
        self.east_west_blocked = Some(north_south_mask | self.base);

        // The Elves with no neighbors will stay put this round, don't make
        // proposals for them.
        self.stationary = Some(self.base & !neighbors);

        // The Elves who _do_ have neighbors want to move. We'll propose moving them
        // in order.
        self.willing_to_move = Some(self.base & neighbors);

        // Return self to keep building
        self
    }

    fn propose_north(mut self) -> Self {
        // Check the current builder state to move forward.
        let Some(willing_to_move) = self.willing_to_move else {
            panic!("No elves willing to move have been identified!");
        };
        let Some(north_south_blocked) = self.north_south_blocked else {
            panic!("Northern neighbors have not been identified!");
        };

        // For all elves willing to move, observe the space to the north
        let check_north = willing_to_move.clone().offset_up();

        // For any elf that doesn't encounter a neighbor, propose that it moves
        let propose_north = check_north & !north_south_blocked;
        self.proposed_north = Some(propose_north);

        // Identify the elves that have proposed to move and remove
        // them from the map of elves who are still willing to move.
        let will_move_north = (check_north & propose_north).offset_down();
        self.willing_to_move = Some(willing_to_move & !will_move_north);

        self
    }

    fn propose_south(mut self) -> Self {
        // Check the current builder state to move forward.
        let Some(willing_to_move) = self.willing_to_move else {
            panic!("No elves willing to move have been identified!");
        };
        let Some(north_south_blocked) = self.north_south_blocked else {
            panic!("Southern neighbors have not been identified!");
        };

        // For all elves willing to move, observe the space to the south
        let check_south = willing_to_move.clone().offset_down();

        // For any elf that doesn't encounter a neighbor, propose that it moves
        let propose_south = check_south & !north_south_blocked;
        self.proposed_south = Some(propose_south);

        // Identify the elves that have proposed to move and remove
        // them from the map of elves who are still willing to move.
        let will_move_south = (check_south & propose_south).offset_up();
        self.willing_to_move = Some(willing_to_move & !will_move_south);

        self
    }

    fn propose_east(mut self) -> Self {
        // Check the current builder state to move forward.
        let Some(willing_to_move) = self.willing_to_move else {
            panic!("No elves willing to move have been identified!");
        };
        let Some(east_west_blocked) = self.east_west_blocked else {
            panic!("Eastern neighbors have not been identified!");
        };

        // For all elves willing to move, observe the space to the east
        let check_east = willing_to_move.clone().offset_right();

        // For any elf that doesn't encounter a neighbor, propose that it moves
        let propose_east = check_east & !east_west_blocked;
        self.proposed_east = Some(propose_east);

        // Identify the elves that have proposed to move and remove
        // them from the map of elves who are still willing to move.
        let will_move_east = (check_east & propose_east).offset_left();
        self.willing_to_move = Some(willing_to_move & !will_move_east);

        self
    }

    fn propose_west(mut self) -> Self {
        // Check the current builder state to move forward.
        let Some(willing_to_move) = self.willing_to_move else {
            panic!("No elves willing to move have been identified!");
        };
        let Some(east_west_blocked) = self.east_west_blocked else {
            panic!("Western neighbors have not been identified!");
        };

        // For all elves willing to move, observe the space to the west
        let check_west = willing_to_move.clone().offset_left();

        // For any elf that doesn't encounter a neighbor, propose that it moves
        let propose_west = check_west & !east_west_blocked;
        self.proposed_west = Some(propose_west);

        // Identify the elves that have proposed to move and remove
        // them from the map of elves who are still willing to move.
        let will_move_west = (check_west & propose_west).offset_right();
        self.willing_to_move = Some(willing_to_move & !will_move_west);

        self
    }

    fn make_proposals(mut self) -> Self {
        for direction in self.propose_order {
            self = match direction {
                Direction::North => self.propose_north(),
                Direction::South => self.propose_south(),
                Direction::East => self.propose_east(),
                Direction::West => self.propose_west(),
            };
        }

        self
    }

    fn resolve_conflicts(mut self) -> Self {
        // Check the current builder state to move forward.
        let Some(stationary) = self.stationary else {
            panic!("The elves who will not try to move have not been identified!");
        };
        let Some(proposed_north) = self.proposed_north else {
            panic!("Proposals to move north have not been evaluated!");
        };
        let Some(proposed_south) = self.proposed_south else {
            panic!("Proposals to move south have not been evaluated!");
        };
        let Some(proposed_east) = self.proposed_east else {
            panic!("Proposals to move east have not been evaluated!");
        };
        let Some(proposed_west) = self.proposed_west else {
            panic!("Proposals to move west have not been evaluated!");
        };

        // Whenever two elves have proposed to move to the same space from
        // the north and south, we need to remove those occupied spaces from
        // the north and south proposals.
        let north_south_conflict = proposed_north & proposed_south;
        self.proposed_north = Some(proposed_north & !north_south_conflict);
        self.proposed_south = Some(proposed_south & !north_south_conflict);

        // We also need to resolve proposal conflicts for elves that propose to
        // move to the same space from the east and west.
        let east_west_conflict = proposed_east & proposed_west;
        self.proposed_east = Some(proposed_east & !east_west_conflict);
        self.proposed_west = Some(proposed_west & !east_west_conflict);

        // Update the mapping of stationary elves to include the elves whose
        // proposals were blocked. Because the `*_conflict` maps indicate the
        // spaces that were blocked, we can get the elves original locations
        // by offsetting the `*_conflict` maps appropriately.
        self.stationary = Some(
            stationary
                | north_south_conflict.clone().offset_up()
                | north_south_conflict.clone().offset_down()
                | east_west_conflict.clone().offset_left()
                | east_west_conflict.clone().offset_right(),
        );

        self
    }

    fn finalize(mut self) -> Grid {
        // Check the current builder state to move forward.
        let Some(stationary) = self.stationary else {
            panic!("The elves who will not try to move have not been identified!");
        };
        let Some(willing_to_move) = self.willing_to_move else {
            panic!("No elves willing to move have been identified!");
        };
        let Some(proposed_north) = self.proposed_north else {
            panic!("Proposals to move north have not been evaluated!");
        };
        let Some(proposed_south) = self.proposed_south else {
            panic!("Proposals to move south have not been evaluated!");
        };
        let Some(proposed_east) = self.proposed_east else {
            panic!("Proposals to move east have not been evaluated!");
        };
        let Some(proposed_west) = self.proposed_west else {
            panic!("Proposals to move west have not been evaluated!");
        };

        // The new state will be a combination of: the elves who didn't try to move,
        // the elves who successfully proposed (and moved) in each direction, the
        // elves who proposed a move and were blocked by a conflicting proposal,
        // and the elves who wanted to move but couldn't make a valid proposal.
        stationary
            | willing_to_move
            | proposed_north
            | proposed_south
            | proposed_east
            | proposed_west
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("../../input/23/input.txt");

    #[test]
    fn part_one() {
        let mut state: Grid = Grid::from_str(INPUT).unwrap();
        let start = std::time::Instant::now();
        let mut propose_order = Rules::default();
        for _ in 0..10 {
            state = GridBuilder::init(state, propose_order)
                .identify_movers()
                .make_proposals()
                .resolve_conflicts()
                .finalize();
            propose_order.rotate();
        }
        println!("Empty Spaces: {}", state.count_empty_spaces());
        println!("Calculated in: {:?}", start.elapsed());
    }

    #[test]
    fn part_two() {
        let mut state: Grid = Grid::from_str(INPUT).unwrap();
        let start = std::time::Instant::now();
        let mut last_state = Grid::default();
        let mut propose_order = Rules::default();
        let mut rounds = 0;
        while state != last_state {
            last_state = state;
            state = GridBuilder::init(state, propose_order)
                .identify_movers()
                .make_proposals()
                .resolve_conflicts()
                .finalize();
            propose_order.rotate();
            rounds += 1;
        }
        println!("Rounds Taken: {}", rounds);
        println!("Calculated in: {:?}", start.elapsed());
    }

}
