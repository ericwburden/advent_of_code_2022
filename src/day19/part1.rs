use super::input::{Blueprint, Recipe, Resource, ResourceCountArray};
use crate::day19::{Input, Output};
use rayon::prelude::*;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::iter::zip;
use std::ops::{Index, IndexMut};
use Resource::*;

pub fn solve(input: &Input) -> Output {
    input
        .par_iter()
        .map(|blueprint| Factory::new(*blueprint, 24))
        .map(|factory| factory.quality_level())
        .sum::<u32>()
        .into()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Factory {
    blueprint: Blueprint,
    remaining: u32,
    bots: ResourceCountArray,
    stockpile: ResourceCountArray,
    produced: ResourceCountArray,
    max_costs: ResourceCountArray,
}

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
    pub fn new(blueprint: Blueprint, time: u32) -> Self {
        let max_costs = blueprint
            .recipes
            .into_iter()
            .fold(Default::default(), |acc, x| acc + x.cost);
        Factory {
            blueprint,
            remaining: time,
            bots: ResourceCountArray([1, 0, 0, 0]),
            stockpile: Default::default(),
            produced: Default::default(),
            max_costs,
        }
    }

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
        if self.remaining == 1 { return None; }

        let last_turn_resources = self.stockpile.saturating_sub(self.bots);
        if zip(cost, last_turn_resources)
            .filter(|(lhs, _)| *lhs > 0)
            .all(|(lhs, rhs)| lhs < rhs)
        {
            return None;
        }

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

    fn key(&self) -> ResourceCountArray {
        self.bots + self.produced
    }

    pub fn geodes_produced(&self) -> u32 {
        let mut heap = BinaryHeap::from([*self]);
        let mut seen = HashSet::new();
        let mut most_geodes = 0;

        while let Some(state) = heap.pop() {
            if state.remaining == 0 {
                return state.stockpile[Geode];
            }
            if seen.contains(&state.key()) {
                continue;
            }
            seen.insert(state.key());
            most_geodes = most_geodes.max(state.produced[Geode]);

            for next_state in state.next_states() {
                if seen.contains(&next_state.key()) {
                    continue;
                }
                if next_state.best_estimate(Geode) < most_geodes {
                    continue;
                }
                heap.push(next_state);
            }
        }
        unreachable!()
    }

    fn quality_level(&self) -> u32 {
        self.blueprint.id * self.geodes_produced()
    }
}
