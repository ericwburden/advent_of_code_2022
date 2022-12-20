use super::input::{Blueprint, Recipe, Resource, ResourceCountArray};
use crate::day19::{Input, Output};
use rayon::prelude::*;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::iter::zip;
use std::ops::{Index, IndexMut};
use Resource::*;

/// Solve Day 19, Part 1
///
/// I finally found a good an legitimate use for `rayon`! I probably could have used
/// parallel processing on Day 16, too. May try that later. Here, though, we can
/// process each blueprint in parallel, really helping with speed.
pub fn solve(input: &Input) -> Output {
    input
        .par_iter()
        .map(|blueprint| Factory::new(*blueprint, 24))
        .map(|factory| factory.quality_level())
        .sum::<u32>()
        .into()
}

/// It's a Factory that produces Factories! Represents each state of resource
/// production and includes the original blueprint, the number of turns remaining,
/// the current bot count, the current stockpile of resources, and the total number
/// of each type of resource produced.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Factory {
    blueprint: Blueprint,
    remaining: u32,
    bots: ResourceCountArray,
    stockpile: ResourceCountArray,
    produced: ResourceCountArray,
}

/// Sorting for Factory, so that the state closest to completion floats to
/// the top of the Binary Heap we'll use for the graph search algorithm. "Greater"
/// values for Factory are those where the most geodes can possibly be produced
/// using the best-guess estimate.
impl Ord for Factory {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_estimate = self.best_estimate(Geode);
        let other_estimate = other.best_estimate(Geode);
        self_estimate.cmp(&other_estimate)
    }
}

impl PartialOrd for Factory {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Factory {
    /// Create a new Factory!
    pub fn new(blueprint: Blueprint, time: u32) -> Self {
        Factory {
            blueprint,
            remaining: time,
            bots: ResourceCountArray([1, 0, 0, 0]),
            stockpile: Default::default(),
            produced: Default::default(),
        }
    }

    /// Try to produce a bot from the given recipe. Attempts to fast-forward the
    /// state until the given bot is produced, if it can be. There are a few
    /// different guard clauses that aim to prevent creating new states that won't
    /// lead to the optimal solution.
    fn produce_recipe(&self, recipe: Recipe) -> Option<Factory> {
        let Recipe { bot, cost } = recipe;

        // Don't produce this recipe if the factory can't produce enough
        // resources to build this bot
        let max_production = (self.bots * self.remaining) + self.stockpile;
        if zip(cost, max_production).any(|(lhs, rhs)| lhs > rhs) {
            return None;
        }

        // If there's only one turn left, then skip it. This factory won't
        // produce anything else.
        if self.remaining == 1 {
            return None;
        }

        // If this factory could have produced the current recipe in its last
        // incarnation, then it should have produced that bot back then. Too
        // late, now!
        let last_turn_resources = self.stockpile.saturating_sub(self.bots);
        if zip(cost, last_turn_resources)
            .filter(|(lhs, _)| *lhs > 0)
            .all(|(lhs, rhs)| lhs < rhs)
        {
            return None;
        }

        /// If we're actually going to produce this bot, we need to advance the
        /// current Factory state minute-by-minute until we've gathered enough
        /// resources to produce the bot. Then we pay the price, produce the bot,
        /// and return the state.
        let mut new_state = *self;
        while new_state.remaining > 0 && self.bots == new_state.bots {
            let available = new_state.stockpile;
            new_state.remaining -= 1;
            new_state.stockpile += new_state.bots;
            new_state.produced += new_state.bots;

            if zip(available, cost).all(|(lhs, rhs)| lhs >= rhs) {
                new_state.bots[bot] += 1;
                new_state.stockpile -= cost;
            }
        }

        Some(new_state)
    }

    /// Identify all the next states that can be reached from the current Factory.
    /// Tries to produce one of each bot and includes a "wait" state where the
    /// Factory just lets time run out. This is for cases when not enough resources
    /// will be generated to produce any more bots before time runs out.
    fn next_states(&self) -> impl Iterator<Item = Factory> + '_ {
        let mut wait_state = *self;
        wait_state.stockpile += self.bots * self.remaining;
        wait_state.produced += self.bots * self.remaining;
        wait_state.remaining = 0;

        let wait_state_iter = std::iter::once(wait_state);

        self.blueprint
            .recipes
            .into_iter()
            .flat_map(|recipe| self.produce_recipe(recipe))
            .chain(wait_state_iter)
    }

    /// Identify the most possible resources of a given type that could be
    /// produced under ideal circumstances.
    fn best_estimate(&self, resource: Resource) -> u32 {
        // Assume that we can make one new bot per minute for
        // the remaining time. In that perfect scenario, how many
        // `resource` would we have a the end of time?
        // This is also our A* heuristic, which will always _overestimate_
        // how close we are to the goal. We're using a max heap,
        // so we need to overestimate to get an admissible heuristic.
        let resource_bots = self.bots[resource] + self.remaining;
        let new_resources = resource_bots * self.remaining;
        new_resources + self.produced[resource]
    }

    /// This is our unique identifier for a given Factory. It's the simplest
    /// value that identifies a Factory uniquely _enough_ to find the right
    /// solution.
    fn key(&self) -> ResourceCountArray {
        self.bots + self.produced
    }

    /// Performs a modified A* search through the possible Factory states,
    /// seeking a state that produces the most possible geodes.
    pub fn geodes_produced(&self) -> u32 {
        let mut heap = BinaryHeap::from([*self]);
        let mut seen = HashSet::new();
        let mut most_geodes = 0; // Used for optimization

        // So long as the heap still has items on it...
        while let Some(state) = heap.pop() {
            // If we reached a state where time runs out, we've identified
            // the state producing the most geodes, assuming we've implemented
            // the ordering of Factories correctly.
            if state.remaining == 0 {
                return state.stockpile[Geode];
            }

            // If we've seen this state already, skip it.
            if seen.contains(&state.key()) {
                continue;
            }

            // Otherwise, mark it as seen. Update the most geodes produced
            // by any state seen so far.
            seen.insert(state.key());
            most_geodes = most_geodes.max(state.produced[Geode]);

            for next_state in state.next_states() {
                // If we've seen this `next_state` before, skip it.
                if seen.contains(&next_state.key()) {
                    continue;
                }

                // If the best possible geode production for this state is still
                // less than the most geodes we've actually seen in a state so
                // far, skip it. The best estimate is an overestimate by design.
                if next_state.best_estimate(Geode) < most_geodes {
                    continue;
                }

                // Add this state to the heap to be checked.
                heap.push(next_state);
            }
        }

        // This should never happen
        unreachable!()
    }

    /// Calcualate the quality level of this Factory
    fn quality_level(&self) -> u32 {
        self.blueprint.id * self.geodes_produced()
    }
}
