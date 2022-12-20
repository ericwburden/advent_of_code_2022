use crate::day19::Input;
use std::ops::{Add, AddAssign, Index, IndexMut, Mul, Sub, SubAssign};

/// Represents one of the resource types we're dealing with today.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

/// This little newtype right here is the star of today's show, as
/// if you couldn't tell by all the basic arithmetic trait implementations
/// down below. This fella is being used to represent the number of
/// robots we have active, the number of resources we've gathered, and
/// the recipe costs for making new robots. Who knew one little array
/// could be so versatile!
#[derive(Debug, Default, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ResourceCountArray(pub [u32; 4]);

/*----------------------------------------------------------------------------------
* This section contains a whole bunch of trait implementations for our hero, the
* ResourceCountArray. We can index the array by resource type, add them together,
* subtract them, multiply them by a scalar, and iterate over them. Truly a marvel!
*/

impl Index<Resource> for ResourceCountArray {
    type Output = u32;

    fn index(&self, index: Resource) -> &Self::Output {
        match index {
            Resource::Ore => &self.0[0],
            Resource::Clay => &self.0[1],
            Resource::Obsidian => &self.0[2],
            Resource::Geode => &self.0[3],
        }
    }
}

impl IndexMut<Resource> for ResourceCountArray {
    fn index_mut(&mut self, index: Resource) -> &mut Self::Output {
        match index {
            Resource::Ore => &mut self.0[0],
            Resource::Clay => &mut self.0[1],
            Resource::Obsidian => &mut self.0[2],
            Resource::Geode => &mut self.0[3],
        }
    }
}

impl Index<usize> for ResourceCountArray {
    type Output = u32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for ResourceCountArray {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Add<ResourceCountArray> for ResourceCountArray {
    type Output = ResourceCountArray;

    fn add(self, other: ResourceCountArray) -> Self::Output {
        let mut sum: ResourceCountArray = Default::default();
        for (idx, (lhs, rhs)) in self.into_iter().zip(other.into_iter()).enumerate() {
            sum[idx] = lhs + rhs;
        }
        sum
    }
}

impl AddAssign<ResourceCountArray> for ResourceCountArray {
    fn add_assign(&mut self, rhs: ResourceCountArray) {
        for (idx, value) in rhs.into_iter().enumerate() {
            self[idx] += value;
        }
    }
}

impl SubAssign<ResourceCountArray> for ResourceCountArray {
    fn sub_assign(&mut self, rhs: ResourceCountArray) {
        for (idx, value) in rhs.into_iter().enumerate() {
            self[idx] -= value;
        }
    }
}

impl Mul<u32> for ResourceCountArray {
    type Output = ResourceCountArray;

    fn mul(self, rhs: u32) -> Self::Output {
        let mut product: ResourceCountArray = Default::default();
        for (idx, value) in self.into_iter().enumerate() {
            product[idx] = self[idx] * rhs;
        }
        product
    }
}

impl IntoIterator for ResourceCountArray {
    type Item = u32;
    type IntoIter = std::array::IntoIter<Self::Item, 4>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

/* -----------------------------------------------------------------------------------
*  Ok, back to normal non-trait stuff.
*/

impl ResourceCountArray {
    /// But only because _saturating_sub()_ isn't from a trait. We needed this one
    /// for a _very_ important performance optimization, though. You'll see that in
    /// the code for part one.
    pub fn saturating_sub(&self, other: ResourceCountArray) -> ResourceCountArray {
        let mut difference: ResourceCountArray = Default::default();
        for (idx, (lhs, rhs)) in self.into_iter().zip(other.into_iter()).enumerate() {
            difference[idx] = lhs.saturating_sub(rhs);
        }
        difference
    }
}

/// Represents a recipe for making a new robot
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Recipe {
    pub bot: Resource,
    pub cost: ResourceCountArray, // See, everywere.
}

/// Represents an entire blueprint, with ID and recipe for each bot type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Blueprint {
    pub id: u32,
    pub recipes: [Recipe; 4],
}

/// The usual module wrapping the parsers for today's input. I'll be honest, the whole
/// inner module thing still seems a little odd to me, but it's the best way I could
/// come up with to namespace the parsing functions so far. So, I'm keeping it until
/// I can think of (or steal) a better idea.
pub mod parser {
    use super::*;
    use anyhow::{anyhow, Result};
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{newline, space1, u32},
        combinator::{opt, value},
        multi::separated_list0,
        sequence::{delimited, pair, preceded, separated_pair, tuple},
        Finish, IResult,
    };

    /// Nom parser for "ore" -> Resource::Ore
    fn ore(s: &str) -> IResult<&str, Resource> {
        value(Resource::Ore, tag("ore"))(s)
    }

    /// Nom parser for "clay" -> Resource::Clay
    fn clay(s: &str) -> IResult<&str, Resource> {
        value(Resource::Clay, tag("clay"))(s)
    }

    /// Nom parser for "obsidian" -> Resource::Obsidian
    fn obsidian(s: &str) -> IResult<&str, Resource> {
        value(Resource::Obsidian, tag("obsidian"))(s)
    }

    /// Nom parser for "geode" -> Resource::Geode
    fn geode(s: &str) -> IResult<&str, Resource> {
        value(Resource::Geode, tag("geode"))(s)
    }

    /// Parses any resource name into the relevant `Resource`
    fn resource(s: &str) -> IResult<&str, Resource> {
        alt((ore, clay, obsidian, geode))(s)
    }

    /// Nom parser for "4 ore" -> ResourceCountArray([4, 0, 0, 0])
    fn cost(s: &str) -> IResult<&str, ResourceCountArray> {
        let (s, (amt, resource)) = separated_pair(u32, space1, resource)(s)?;
        let mut cost: ResourceCountArray = Default::default();
        cost[resource] += amt;
        Ok((s, cost))
    }

    /// Nom parser for "3 ore and 14 clay" -> ResourceCountArray([3, 14, 0, 0])
    fn cost2(s: &str) -> IResult<&str, ResourceCountArray> {
        let mut resources: ResourceCountArray = Default::default();
        let (s, (cost1, cost2)) = separated_pair(cost, tag(" and "), cost)(s)?;
        Ok((s, cost1 + cost2))
    }

    /// Nom parser for
    ///   "Each ore robot costs 4 ore"
    ///     -> Recipe { bot: Resource::Ore, cost: ResourceCountArray([4, 0, 0, 0]) }
    fn recipe(s: &str) -> IResult<&str, Recipe> {
        let (s, _) = tag("Each ")(s)?;
        let (s, bot) = resource(s)?;
        let (s, _) = tag(" robot costs ")(s)?;
        let (s, cost) = alt((cost2, cost))(s)?;
        let (s, _) = tag(".")(s)?;
        Ok((s, Recipe { bot, cost }))
    }

    /// Nom parser for all four recipes from a line
    fn recipes(s: &str) -> IResult<&str, [Recipe; 4]> {
        let (s, (r1, r2, r3, r4)) = tuple((
            preceded(space1, recipe),
            preceded(space1, recipe),
            preceded(space1, recipe),
            preceded(space1, recipe),
        ))(s)?;
        Ok((s, [r1, r2, r3, r4]))
    }

    /// Nom parser for a single line of the input, producing a Blueprint
    fn blueprint(s: &str) -> IResult<&str, Blueprint> {
        let (s, id) = delimited(tag("Blueprint "), u32, tag(":"))(s)?;
        let (s, recipes) = recipes(s)?;
        Ok((s, Blueprint { id, recipes }))
    }

    /// Parses each line of the input into a Blueprint and return the list
    fn blueprints(s: &str) -> IResult<&str, Vec<Blueprint>> {
        separated_list0(newline, blueprint)(s)
    }

    /// Entrypoint for the parsing functions
    pub fn parse(s: &str) -> Result<Vec<Blueprint>> {
        let (_, result) = blueprints(s).finish().map_err(|e| anyhow!("{e}"))?;
        Ok(result)
    }
}

const INPUT: &str = include_str!("../../input/19/input.txt");

/// Parse that input
pub fn read() -> Input {
    parser::parse(INPUT).unwrap()
}

#[cfg(test)]
mod test {
    use super::parser::*;
    use super::*;

    #[test]
    fn playground() {
        let input = parser::parse(INPUT).unwrap();
        println!("{:?}", input);
    }
}
