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

/// Finds the two maximum values in an iterator of i64 values.
///
/// # Arguments
/// - `iter`: An iterator of i64 values.
fn find_two_max_values<I>(iter: I) -> Option<(i64, i64)>
where
    I: IntoIterator<Item = i64>,
{
    let mut iter = iter.into_iter();

    // Get the first two values to initialize the two max values
    let first = iter.next()?;
    let second = iter.next()?;

    // Initialize the two max values
    let (mut max1, mut max2) = if first > second {
        (first, second)
    } else {
        (second, first)
    };

    // Iterate through the rest of the values
    for value in iter {
        if value > max1 {
            max2 = max1;
            max1 = value;
        } else if value > max2 {
            max2 = value;
        }
    }

    Some((max1, max2))
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
    let mut values = Vec::new();

    for (task, &assigned_count) in task_assigned.iter().enumerate() {
        if assigned_count < max_tile_occurrences {
            let value = -distance_matrix.get(agent, task) - prices[task];
            values.push((task, value));
        }
    }

    if values.len() < 2 {
        return None; // Not enough tasks to compare
    }

    // Find the two maximum values
    let (max1, max2) = find_two_max_values(values.iter().map(|&(_, value)| value))?;

    // Find the tasks corresponding to the two maximum values
    let best_task = values.iter().find(|&&(_, value)| value == max1)?.0;

    Some((best_task, max1, max2))
}

impl Solve for Auction {
    fn solve(&mut self, distance_matrix: &DistanceMatrix) -> Result<Vec<usize>, PhomoError> {
        // Check if the number of columns is less than the number of rows
        if distance_matrix.columns < distance_matrix.rows {
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
