use crate::day20::{Input, Output};
use std::collections::HashMap;
use std::ops::{Add, AddAssign, Index, IndexMut};

/// Solve Day 20, Part 1
pub fn solve(input: &Input) -> Output {
    // Make a decryptor!
    let mut decryptor = MixingDecryptor::from(input.clone());
    decryptor.mix();

    // Just like that!
    decryptor.grove_coordinates_sum().into()
}

/// Really just a convenient struct for bundling together the three vectors
/// we'll use to solve this puzzle. Who needs an actual linked list when you
/// can just simulate one! The three vectors here will serve as a
/// pseudo-doubly-linked-list, with the values in `nodes`, the pointers to
/// next values in `forward_links`, and the pointers ot previous values
/// in `backward_links`.
#[derive(Debug)]
pub struct MixingDecryptor {
    nodes: Vec<i64>,
    forward_links: Vec<usize>,
    backward_links: Vec<usize>,
}

impl From<Vec<i64>> for MixingDecryptor {
    fn from(nodes: Vec<i64>) -> Self {
        // The forward links are the indices of the nodes that come after the
        // node at the same index as the link. So, `forward_links[node[0]]`
        // in the unmixed list would be `1`.
        let forward_links: Vec<_> = (1..nodes.len())
            .into_iter()
            .chain(std::iter::once(0))
            .collect();

        // The backward links are the indices of the nodes that come before the
        // node at the same index as the link.
        let backward_links: Vec<_> = std::iter::once(nodes.len() - 1)
            .chain(0..(nodes.len() - 1))
            .collect();

        MixingDecryptor {
            nodes,
            forward_links,
            backward_links,
        }
    }
}

impl MixingDecryptor {
    /// Mix up that list!
    pub fn mix(&mut self) {
        let Self {
            nodes,
            forward_links,
            backward_links,
        } = self;
        for (moved_node_idx, moved_node) in nodes.iter().copied().enumerate() {
            // If the value of the node is zero, don't move anything.
            if moved_node == 0 {
                continue;
            }

            // We calculate the number of skips such that we never
            // wrap fully around the list of nodes. We do this by taking
            // the absolute value of the node mod the length of the list
            // minus one. Why minus one? Because we don't count the node
            // we're moving.
            let skips = (moved_node.unsigned_abs() as usize) % (nodes.len() - 1);

            // Find the index of the node we'll displace by following the links from
            // the current node a number of times indicated by the value of the node.
            // Follow forward links for positive numbers and backward links for
            // negative numbers.
            let mut displaced_node_idx = moved_node_idx;
            for _ in (0..skips) {
                if moved_node > 0 {
                    displaced_node_idx = forward_links[displaced_node_idx];
                }
                if moved_node < 0 {
                    displaced_node_idx = backward_links[displaced_node_idx];
                }
            }

            // Move backwards one more time if the value of the node is negative,
            // since the displaced node is always displaced to the left.
            if moved_node < 0 {
                displaced_node_idx = backward_links[displaced_node_idx];
            }

            // Get the indices of other nodes we'll need to adjust links for
            let moved_node_next = forward_links[moved_node_idx];
            let moved_node_prev = backward_links[moved_node_idx];
            let displaced_node_next = forward_links[displaced_node_idx];

            // Close the gap left by the moved node
            forward_links[moved_node_prev] = moved_node_next;
            backward_links[moved_node_next] = moved_node_prev;

            // Insert the moved node after the displaced node
            forward_links[moved_node_idx] = displaced_node_next;
            backward_links[moved_node_idx] = displaced_node_idx;

            // Update the links for the displaced node
            forward_links[displaced_node_idx] = moved_node_idx;
            backward_links[displaced_node_next] = moved_node_idx;
        }
    }

    /// Get the index of the zero value.
    fn find_zero_index(&self) -> usize {
        // This is safe because the puzzle tells us that there's a zero in the
        // input. Otherwise, unwrapping here would be a bad idea.
        self.nodes.iter().position(|x| *x == 0).unwrap()
    }

    /// Find and sum the 1000th, 2000th, and 3000th values in order starting
    /// from the zero value, wrapping around the list as necessary.
    pub fn grove_coordinates_sum(&self) -> i64 {
        let mut current_index = self.find_zero_index();
        let mut total = 0;

        // Similar to how we mixed the lists, we can just follow links through
        // the `forward_links` vector to move forward in the list.
        for positions_after in 1..=3000 {
            current_index = self.forward_links[current_index];
            if positions_after % 1000 == 0 {
                total += self.nodes[current_index];
            }
        }
        total
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn playground() {
        let input = vec![1, 2, -3, 3, -2, 0, 4];
        let mut decryptor = MixingDecryptor::from(input);
        decryptor.mix();
        println!("{}", decryptor.grove_coordinates_sum());
    }
}
