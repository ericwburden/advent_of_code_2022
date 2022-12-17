use crate::day16::{Input, Output, ValveMap};

/// Solve Day 16, Part 1
///
/// I'm not satisfied with this solution. We're essentially performing a depth-first
/// search starting from "AA" and identifying _every_ path through the notable valves
/// then returning the maximum pressure released by _any_ path. I'm quite certain
/// that there's an A* implementation that could do this much more efficiently, but
/// I'm struggling to develop an appropriate penalty function. I'll come back to this
/// one.
pub fn solve(input: &Input) -> Output {
    // Start at valve "AA"
    let state = TravelState::default();
    let mut open = vec![state];

    // Depth-first seach starting with "AA". Every time we hit a state where
    // time has run out, we check the total pressure released and update
    // the maximum pressure if necessary.
    let mut max_released = 0;

    // While the search stack has items remaining...
    while let Some(state) = open.pop() {
        // Check the state on the top of the stack. If time is up, try to update
        // the max pressure released and move on to the next item in the stack.
        if state.remaining == 0 {
            max_released = max_released.max(state.released);
            continue;
        }

        // If the state is an intermediate state (time still remaining), then add
        // each possible next state to the stack.
        for next_state in state.next_states(input) {
            open.push(next_state);
        }
    }

    // Return the maximum pressure released by any path through the valves
    max_released.into()
}

/// Represents the state of a path through the valves. Indicates current location
/// as a node index, the set of open valves, the amount of time remaining, and
/// the total pressure that will be released once time runs out given the valves
/// open.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TravelState {
    pub location: usize,
    pub valves_open: u64,
    pub remaining: u32,
    pub released: u32,
}

impl Default for TravelState {
    fn default() -> Self {
        TravelState {
            location: Default::default(),
            valves_open: Default::default(),
            remaining: 30, // Default to 30 minutes remaining
            released: Default::default(),
        }
    }
}

impl TravelState {
    /// Return a list of possible next states that can be reached from the
    /// current state.
    pub fn next_states(&self, map: &ValveMap) -> Vec<TravelState> {
        let mut out = Vec::new();

        // For each neighboring valve (and the distance to get to it)...
        for (neighbor, distance) in map.edges[self.location].iter() {
            // If the distance to the neighbor is to far to reach in the
            // time remaining or that valve is already open, then skip that
            // neighbor as a possible next state.
            if *distance >= self.remaining {
                continue;
            }
            let valve = map.nodes[*neighbor];
            if valve.id & self.valves_open > 0 {
                continue;
            }

            // Update the next state by subtracting the distance traveled from the
            // time remaining, with one extra for opening the destination valve.
            // Set the location of the next state to the neighbor index and add
            // the set bit for the valve ID to the value keeping track of which
            // valves are already open. When opening a new valve, add the flow rate
            // of that valve times the remaining time to the total pressure released
            // by this state, since we already know that valve will remain open
            // for the rest of the time, releasing its flow rate each minute.
            let mut new_state = *self;
            new_state.location = *neighbor;
            new_state.valves_open |= valve.id;
            new_state.remaining -= (distance + 1);
            new_state.released += (new_state.remaining * valve.flow);
            out.push(new_state);
        }

        // If there aren't any neighbors to travel to, either because all the valves
        // are already open or the travel time is more than time remaining, then we
        // can just "wait it out", staying put until time runs out.
        if out.is_empty() {
            let mut wait_state = *self;
            wait_state.remaining = 0;
            out.push(wait_state)
        }

        out
    }
}
