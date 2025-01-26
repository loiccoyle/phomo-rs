use crate::error::PhomoError;
use crate::solvers::Solve;
use crate::solvers::SolverConfig;
use crate::DistanceMatrix;

use super::error::AuctionError;
use super::error::SolverError;

/// The Auction struct represents the auction algorithm solver.
#[derive(Debug)]
pub struct Auction {
    epsilon: i64,
    config: SolverConfig,
}

impl Default for Auction {
    fn default() -> Self {
        Self {
            epsilon: 1,
            config: SolverConfig::default(),
        }
    }
}

impl Auction {
    /// Creates a new instance of the Auction struct with the given epsilon and configuration.
    ///
    /// # Arguments
    /// - `epsilon`: The epsilon value for the auction algorithm.
    /// - `config`: The configuration for the solver.
    pub fn new(epsilon: i64, config: SolverConfig) -> Self {
        Self { epsilon, config }
    }
}

/// Finds the best and second-best tasks for a given agent.
///
/// # Arguments
/// - `agent`: The index of the agent.
/// - `distance_matrix`: The distance matrix.
/// - `prices`: The prices for each task.
/// - `task_assigned`: The number of times each task has been assigned.
/// - `max_tile_occurrences`: The maximum number of times a task can be assigned.
///
/// # Returns
/// An option containing a tuple of the best task index, the best value, and the second-best value, or None if there are fewer than two tasks to compare.
fn find_best_and_second_best(
    agent: usize,
    distance_matrix: &DistanceMatrix,
    prices: &[i64],
    task_assigned: &[usize],
    max_tile_occurrences: usize,
) -> Option<(usize, i64, i64)> {
    let mut best_task = usize::MAX;
    let mut best_value = i64::MIN;
    let mut second_best_value = i64::MIN;

    for (task, &assigned_count) in task_assigned.iter().enumerate() {
        if assigned_count < max_tile_occurrences {
            let value = -distance_matrix.get(agent, task) - prices[task];

            if value > best_value {
                second_best_value = best_value;
                best_value = value;
                best_task = task;
            } else if value > second_best_value {
                second_best_value = value;
            }
        }
    }

    if best_task == usize::MAX || second_best_value == i64::MIN {
        return None; // Not enough tasks to compare
    }

    Some((best_task, best_value, second_best_value))
}

impl Solve for Auction {
    fn solve(&mut self, distance_matrix: &DistanceMatrix) -> Result<Vec<usize>, PhomoError> {
        // Check if the number of columns is less than the number of rows
        if distance_matrix.columns * self.config.max_tile_occurrences < distance_matrix.rows {
            return Err(SolverError::TooFewColumns {
                rows: distance_matrix.rows,
                columns: distance_matrix.columns,
            }
            .into());
        }

        let num_agents = distance_matrix.rows;
        let num_tasks = distance_matrix.columns;

        let mut prices = vec![0; num_tasks]; // Prices for each task
        let mut assignment = vec![None; num_agents]; // Assignment of tasks to agents
        let mut task_assigned = vec![0; num_tasks]; // Track how many times each task is assigned

        // Continue until all agents are assigned a task
        while assignment.iter().any(|&a| a.is_none()) {
            for (agent, assigned_task) in assignment.iter_mut().enumerate().take(num_agents) {
                if assigned_task.is_some() {
                    continue; // Skip already assigned agents
                }

                // Find the best and second-best tasks for the current agent
                if let Some((best_task, best_value, second_best_value)) = find_best_and_second_best(
                    agent,
                    distance_matrix,
                    &prices,
                    &task_assigned,
                    self.config.max_tile_occurrences,
                ) {
                    // Calculate the bid for the best task
                    let bid = best_value - second_best_value + self.epsilon;
                    prices[best_task] += bid; // Update the price for the best task
                    *assigned_task = Some(best_task); // Assign the task to the agent
                    task_assigned[best_task] += 1; // Increment the assignment count for the task
                } else {
                    // No available task for the agent
                    return Err(SolverError::from(AuctionError::UnassignedAgents).into());
                }
            }
        }

        Ok(assignment.into_iter().map(|a| a.unwrap()).collect())
    }
}
