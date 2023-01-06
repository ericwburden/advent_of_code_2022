use anyhow::{bail, Error};
use itertools::Itertools;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl};
use std::str::FromStr;

// These are handy to have named, especially the high bit.
const U64_LOW_BIT: u64 = 1;
const U64_HIGH_BIT: u64 = 9223372036854775808;

/// Represents one row in the Grid. Each row is composed of a number 
/// of unsigned integers. Each bit in the integers represents one column
/// in the overall grid. For the rest of this implementation, it's
/// important to note that the 'indices' of the bits in an integer are
/// arrayed from right to left, and normal array indices are read from
/// left to right. This means that the rightmost bit in each integer
/// represents the leftmost column in that "chunk" of bits in the row.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct GridRow<const CHUNKS: usize>([u64; CHUNKS]);

/// Default a row where each chunk is 0
impl<const CHUNKS: usize> Default for GridRow<CHUNKS> {
    fn default() -> Self {
        let inner = [<u64>::default(); CHUNKS];
        Self(inner)
    }
}

/// This struct specifies a particular bit in a GridRow in more familiar,
/// array-like terms. These are mostly used for putting information into
/// and getting information out of a GridRow. Other operations work on the
/// bits directly.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct GridRowIdx {
    chunk: usize,
    set_bit: u64,
}

/// Convert a column index (usize) into a GridRowIdx, allowing you to access
/// the `usize` column more easily.
impl From<usize> for GridRowIdx {
    fn from(index: usize) -> Self {
        let chunk = index / <u64>::BITS as usize;
        let offset = index % <u64>::BITS as usize;
        let set_bit = 1 << offset;
        GridRowIdx { chunk, set_bit }
    }
}

impl<const CHUNKS: usize> GridRow<CHUNKS> {
    /// Set a particular bit in the GridRow
    fn set(&mut self, GridRowIdx { chunk, set_bit }: GridRowIdx) {
        self.0[chunk] |= set_bit;
    }

    /// Check if a particular bit in the GridRow is set
    fn is_set(&self, GridRowIdx { chunk, set_bit }: GridRowIdx) -> bool {
        self.0[chunk] & set_bit != 0
    }

    /// Produce all the GridRowIdx indices present in this GridRow, from
    /// left to right. These indices can be used to iterate through the
    /// bits in this GridRow.
    fn indices(&self) -> impl Iterator<Item = GridRowIdx> {
        (0..CHUNKS)
            .cartesian_product(0..(<u64>::BITS as usize))
            .map(|(c, b)| c + b)
            .map(GridRowIdx::from)
    }

    /// Shift the bits in this row one space to the right. Accounts for the 
    /// fact that the rows aren't really contiguous but may be separated into
    /// multiple "chunks" of bits (integers).
    fn offset_right(&mut self) {
        let mut carry = 0; // Carry overflowing bits from one chunk to the next

        // For each chunk of bits...
        for chunk in self.0.iter_mut() {
            // Rotate in the "opposite" direction of the offset because bits
            // are read from right to left and the row is read from left to right.
            *chunk = chunk.rotate_left(1);

            // Get just the bit that was wrapped around the end of the "chunk"
            let wrapped_bit = *chunk & U64_LOW_BIT;

            // Unset that wrapped bit if it was set.
            *chunk ^= wrapped_bit;

            // Set the current carried bit. This carries the wrapped bit from
            // one chunk to the next.
            *chunk |= carry;

            // Carry the wrapped bit forward.
            carry = wrapped_bit;
        }
    }

    /// Shift the bits in this row one space to the left. Accounts for the 
    /// fact that the rows aren't really contiguous but may be separated into
    /// multiple "chunks" of bits (integers).
    fn offset_left(&mut self) {
        let mut carry = 0; // Carry overflowing bits from one chunk to the next

        // For each chunk of bits, in reverse order...
        for chunk in self.0.iter_mut().rev() {
            // Rotate in the "opposite" direction of the offset because bits
            // are read from right to left and the row is read from left to right.
            *chunk = chunk.rotate_right(1);

            // Get just the bit that was wrapped around the end of the "chunk". Since
            // we're rotating right, this is the highest bit.
            let wrapped_bit = *chunk & U64_HIGH_BIT;

            // Unset that wrapped bit if it was set.
            *chunk ^= wrapped_bit;

            // Set the current carried bit. This carries the wrapped bit from
            // one chunk to the next.
            *chunk |= carry;

            // Carry the wrapped bit forward.
            carry = wrapped_bit;
        }
    }
}

/**************************************************************************************
* Bitwise operations for GridRow! GridRow bitwise operations are performed chunk
* by chunk, which means they need to be the same length. This is handled by the
* const generic CHUNKS, so Rust won't let you perform GridRow<1> & GridRow<2>.
**************************************************************************************/

impl<const CHUNKS: usize> BitAnd<GridRow<CHUNKS>> for GridRow<CHUNKS> {
    type Output = GridRow<CHUNKS>;

    fn bitand(self, other: GridRow<CHUNKS>) -> Self::Output {
        let GridRow(mut lhs) = self;
        let GridRow(rhs) = other;
        lhs.iter_mut()
            .zip(rhs.into_iter())
            .for_each(|(l, r)| *l &= r);
        GridRow(lhs)
    }
}

impl<const CHUNKS: usize> BitAndAssign for GridRow<CHUNKS> {
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}

impl<const CHUNKS: usize> BitOr<GridRow<CHUNKS>> for GridRow<CHUNKS> {
    type Output = GridRow<CHUNKS>;

    fn bitor(self, other: GridRow<CHUNKS>) -> Self::Output {
        let GridRow(mut lhs) = self;
        let GridRow(rhs) = other;
        lhs.iter_mut()
            .zip(rhs.into_iter())
            .for_each(|(l, r)| *l |= r);
        GridRow(lhs)
    }
}

impl<const CHUNKS: usize> BitOrAssign for GridRow<CHUNKS> {
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}

impl<const CHUNKS: usize> BitXor<GridRow<CHUNKS>> for GridRow<CHUNKS> {
    type Output = GridRow<CHUNKS>;

    fn bitxor(self, other: GridRow<CHUNKS>) -> Self::Output {
        let GridRow(mut lhs) = self;
        let GridRow(rhs) = other;
        lhs.iter_mut()
            .zip(rhs.into_iter())
            .for_each(|(l, r)| *l ^= r);
        GridRow(lhs)
    }
}

impl<const CHUNKS: usize> BitXorAssign<GridRow<CHUNKS>> for GridRow<CHUNKS> {
    fn bitxor_assign(&mut self, other: GridRow<CHUNKS>) {
        *self = *self ^ other;
    }
}

impl<const CHUNKS: usize> Not for GridRow<CHUNKS> {
    type Output = GridRow<CHUNKS>;

    fn not(self) -> Self::Output {
        let GridRow(mut chunks) = self;
        chunks.iter_mut().for_each(|c| *c = !*c);
        GridRow(chunks)
    }
}

/**************************************************************************************
* End of GridRow bitwise operations. Yay!
* This wraps up the GridRow and GridRowIdx definitions. Now, on to the Grid itself!
**************************************************************************************/


/// Represents a Grid of bits, where set bits indicate an elf is present in that
/// space. A collection of GridRows. 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Grid<const CHUNKS: usize, const ROWS: usize> {
    rows: [GridRow<CHUNKS>; ROWS],
}

/// The default Grid of the appropriate size with all spaces empty.
impl<const CHUNKS: usize, const ROWS: usize> Default for Grid<CHUNKS, ROWS> {
    fn default() -> Self {
        let rows = [GridRow::default(); ROWS];
        Self { rows }
    }
}

/// This struct specifies a particular bit in a Grid in more familiar,
/// array-like terms. These are mostly used for putting information into
/// and getting information out of a Grid. Other operations work on the
/// bits directly.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct GridIdx {
    row: usize,
    bit: GridRowIdx,
}

/// Convert a (<row>, <col>) index into a GridIdx that serves the purpose
/// of a (<row>, <col>) index into your Grid.
impl From<(usize, usize)> for GridIdx {
    fn from(value: (usize, usize)) -> Self {
        let row = value.0;
        let bit = GridRowIdx::from(value.1);
        GridIdx { row, bit }
    }
}

impl<const CHUNKS: usize, const ROWS: usize> Grid<CHUNKS, ROWS> {
    /// Set a bit in the Grid
    fn set(&mut self, GridIdx { row, bit }: GridIdx) {
        self.rows[row].set(bit);
    }

    /// Check to see if a bit in the Grid is set
    fn is_set(&self, GridIdx { row, bit }: GridIdx) -> bool {
        self.rows[row].is_set(bit)
    }

    /// Iterate through the indices in this Grid. Allows for iterating over
    /// the spaces in the Grid indirectly.
    fn indices(&self) -> impl Iterator<Item = GridIdx> {
        (0..ROWS).cartesian_product(0..(CHUNKS * 64)).map(GridIdx::from)
    }

    /// Shift all the bits in the Grid one space to the right.
    fn offset_right(&mut self) -> Self {
        self.rows.iter_mut().for_each(|r| r.offset_right());
        *self
    }

    /// Shift all the bits in the Grid one space to the left.
    fn offset_left(&mut self) -> Self {
        self.rows.iter_mut().for_each(|r| r.offset_left());
        *self
    }

    /// Shift all the bits in the Grid one space up. Essentially just rotates the
    /// GridRows in the Grid and replaces the wrapped Row with an empty Row.
    fn offset_up(&mut self) -> Self {
        self.rows.rotate_left(1);
        if let Some(row) = self.rows.last_mut() {
            *row = GridRow::default();
        }
        *self
    }

    /// Shift all the bits in the Grid one space down. Essentially just rotates the
    /// GridRows in the Grid and replaces the wrapped Row with an empty Row.
    fn offset_down(&mut self) -> Self {
        self.rows.rotate_right(1);
        if let Some(row) = self.rows.first_mut() {
            *row = GridRow::default();
        }
        *self
    }

    /// Identify the bounds of the populated rectangle in the Grid, returning
    /// ((<min_row>, <min_col>), (<max_row>, <max_col>))
    fn bounds(&self) -> ((usize, usize), (usize, usize)) {
        let mut min_row = usize::MAX;
        let mut min_col = usize::MAX;
        let mut max_row = usize::MIN;
        let mut max_col = usize::MIN;

        for (row, col) in (0..ROWS).cartesian_product(0..(CHUNKS * 64)) {
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

    /// Count the empty spaces in the populated rectangle in the Grid. Iterate over
    /// the spaces in the grid and count the empties that are inside the bounds of
    /// the populated rectangle.
    pub fn count_empty_spaces(&self) -> u32 {
        let mut empty_spaces = 0;
        let ((min_row, min_col), (max_row, max_col)) = self.bounds();
        for (row, col) in (0..ROWS).cartesian_product(0..(CHUNKS * 64)) {
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

/// This is the input parsing for today's puzzle. Iterates over the lines and
/// characters of the input, setting bits in an empty Grid. Shifts the bits
/// such that the set bits are centered around the center of the Grid.
impl<const CHUNKS: usize, const ROWS: usize> FromStr for Grid<CHUNKS, ROWS> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows = s.lines().count();
        let cols = s.lines().next().map(|l| l.len()).unwrap_or_default();
        let mut grid = Grid::default();
        let row_offset = (ROWS / 2) - (rows / 2);
        let col_offset = ((CHUNKS * 64) / 2) - (cols / 2);

        for (row, line) in s.lines().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                let idx = GridIdx::from((row + row_offset, col + col_offset));
                match ch {
                    '#' => grid.set(idx),
                    '.' => continue,
                    _ => bail!("Input string should only contain '#', '.', or newline!"),
                }
            }
        }

        Ok(grid)
    }
}


/**************************************************************************************
* Bitwise operations for Grid! Grid bitwise operations are performed row
* by row, which means they need to be the same length. This is handled by the
* const generics CHUNKS and ROWS, so Rust won't let you perform
* Grid<1, 1> & GridRow<1, 2> or Grid<1, 1> & Grid<2, 1>.
**************************************************************************************/

impl<const CHUNKS: usize, const ROWS: usize> BitAnd<Grid<CHUNKS, ROWS>> for Grid<CHUNKS, ROWS> {
    type Output = Grid<CHUNKS, ROWS>;

    fn bitand(self, other: Grid<CHUNKS, ROWS>) -> Self::Output {
        let Grid { rows: mut lhs_rows } = self;
        let Grid { rows: rhs_rows } = other;
        lhs_rows
            .iter_mut()
            .zip(rhs_rows.into_iter())
            .for_each(|(l, r)| *l &= r);
        Grid { rows: lhs_rows }
    }
}

impl<const CHUNKS: usize, const ROWS: usize> BitOr<Grid<CHUNKS, ROWS>> for Grid<CHUNKS, ROWS> {
    type Output = Grid<CHUNKS, ROWS>;

    fn bitor(self, other: Grid<CHUNKS, ROWS>) -> Self::Output {
        let Grid { rows: mut lhs_rows } = self;
        let Grid { rows: rhs_rows } = other;
        lhs_rows
            .iter_mut()
            .zip(rhs_rows.into_iter())
            .for_each(|(l, r)| *l |= r);
        Grid { rows: lhs_rows }
    }
}

impl<const CHUNKS: usize, const ROWS: usize> BitXor<Grid<CHUNKS, ROWS>> for Grid<CHUNKS, ROWS> {
    type Output = Grid<CHUNKS, ROWS>;

    fn bitxor(self, other: Grid<CHUNKS, ROWS>) -> Self::Output {
        let Grid { rows: mut lhs_rows } = self;
        let Grid { rows: rhs_rows } = other;
        lhs_rows
            .iter_mut()
            .zip(rhs_rows.into_iter())
            .for_each(|(l, r)| *l ^= r);
        Grid { rows: lhs_rows }
    }
}

impl<const CHUNKS: usize, const ROWS: usize> Not for Grid<CHUNKS, ROWS> {
    type Output = Grid<CHUNKS, ROWS>;

    fn not(self) -> Self::Output {
        let Grid { mut rows } = self;
        rows.iter_mut().for_each(|r| *r = !*r);
        Grid { rows }
    }
}

/**************************************************************************************
* And that's it for bitwise operations on Grid.
**************************************************************************************/



/// Represents one of the four cardinal directions.
#[derive(Debug, Clone, Copy)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

/// Represents the list of rules for determining which directions the elves
/// propose to move, in order.
#[derive(Debug, Clone, Copy)]
pub struct Rules([Direction; 4]);

impl Rules {
    /// Rotate the first rule to the back of the list.
    pub fn rotate(&mut self) {
        self.0.rotate_left(1);
    }
}

/// Default the list of rules to the order specified by the puzzle text.
impl Default for Rules {
    fn default() -> Self {
        use Direction::*;
        Self([North, South, West, East])
    }
}

/// Iterate over the Directions in Rules
impl IntoIterator for Rules {
    type Item = Direction;
    type IntoIter = std::array::IntoIter<Self::Item, 4>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}


/// Represents an interim struct used to construct a new Grid state out of an 
/// existing Grid. I'm not convinced this is 100% a great use of the builder
/// pattern, but it's good practice.
#[derive(Debug, Default)]
pub struct GridBuilder<const CHUNKS: usize, const ROWS: usize> {
    base: Grid<CHUNKS, ROWS>,
    propose_order: Rules,
    north_south_blocked: Option<Grid<CHUNKS, ROWS>>,
    east_west_blocked: Option<Grid<CHUNKS, ROWS>>,
    stationary: Option<Grid<CHUNKS, ROWS>>,
    willing_to_move: Option<Grid<CHUNKS, ROWS>>,
    proposed_north: Option<Grid<CHUNKS, ROWS>>,
    proposed_south: Option<Grid<CHUNKS, ROWS>>,
    proposed_east: Option<Grid<CHUNKS, ROWS>>,
    proposed_west: Option<Grid<CHUNKS, ROWS>>,
}

impl<const CHUNKS: usize, const ROWS: usize> GridBuilder<CHUNKS, ROWS> {
    /// Start up a new GridBuilder based on an existing Grid and set of proposal rules.
    pub fn init(base: Grid<CHUNKS, ROWS>, propose_order: Rules) -> Self {
        GridBuilder {
            base,
            propose_order,
            ..Default::default()
        }
    }

    /// Split the initial Grid into spaces that contain elves with neighbors (the
    /// elves who will try to move) and spaces that contain elves who do not
    /// have neighbors (they'll stay put).
    pub fn identify_movers(mut self) -> Self {
        // Generate masks for the spaces around the elves. These spaces will overlap
        // with elves who have neighbors. Make sure we don't accidentally mutate the
        // base Grid.
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

    /// For elves that can move north (no northerly neighbors), propose they do so
    /// and remove those elves from the mapping of elves who still want to move.
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

    /// For elves that can move south (no southerly neighbors), propose they do so
    /// and remove those elves from the mapping of elves who still want to move.
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

    /// For elves that can move east (no easterly neighbors), propose they do so
    /// and remove those elves from the mapping of elves who still want to move.
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

    /// For elves that can move west (no westerly neighbors), propose they do so
    /// and remove those elves from the mapping of elves who still want to move.
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

    /// Step through the list of rules in `propose_order`, and propose elves move
    /// in each direction in order.
    pub fn make_proposals(mut self) -> Self {
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

    /// When two elves propose to move to the same space, revert the proposed movement
    /// of those elves. Note that the only opportunities for two elves to conflict is
    /// if they are in the same row or column with one empty space between them.
    pub fn resolve_conflicts(mut self) -> Self {
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

    /// Produce a Grid from the GridBuilder by combining all the different elf states.
    pub fn finalize(mut self) -> Grid<CHUNKS, ROWS> {
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

    /// Pretty printing is really helpful for debugging!
    impl<const CHUNKS: usize> Display for GridRow<CHUNKS> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            for idx in self.indices() {
                let glyph = if self.is_set(idx) { '#' } else { '.' };
                write!(f, "{glyph}")?;
            }
            write!(f, "")
        }
    }

    /// Pretty printing is really helpful for debugging!
    impl<const CHUNKS: usize, const ROWS: usize> Display for Grid<CHUNKS, ROWS> {
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

    const INPUT: &str = include_str!("../../input/23/input.txt");

    #[test]
    fn playground() {
        println!("{}", 1u32.rotate_right(1));
    }

    #[test]
    fn part_one() {
        let mut state: Grid<3, 192> = Grid::from_str(INPUT).unwrap();
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
        let mut state: Grid<3, 192> = Grid::from_str(INPUT).unwrap();
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
