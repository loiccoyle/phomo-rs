use crate::error::PhomoError;
use crate::macros::maybe_progress_bar;
use crate::solvers::error::HungarianError;
use crate::solvers::error::SolverError;
use crate::solvers::Solve;
use crate::solvers::SolverConfig;
use crate::DistanceMatrix;

const UNASSIGNED: usize = usize::MAX;

#[derive(Debug)]
struct HungarianState {
    dual_row_values: Vec<i64>,
    dual_column_values: Vec<i64>,
    shortest_path_costs: Vec<i64>,
    augmentation_path: Vec<usize>,
    column_assigned_to_row: Vec<usize>,
    row_assigned_to_column: Vec<usize>,
    selected_rows: Vec<bool>,
    selected_columns: Vec<bool>,
    remaining_columns: Vec<usize>,
}

#[derive(Debug, Default)]
pub struct Hungarian {
    config: SolverConfig,
}

impl Hungarian {
    /// Creates a new instance of the solver with the given distance matrix.
    pub fn new(config: SolverConfig) -> Self {
        Self { config }
    }
}

/// Finds an augmenting path starting from the given row.
/// Returns the sink column and the minimum value found.
fn find_augmenting_path(
    mut current_row: usize,
    distance_matrix: &DistanceMatrix,
    state: &mut HungarianState,
) -> Result<(usize, i64), SolverError> {
    let nc = distance_matrix.columns;
    let cost = &distance_matrix.data;
    let mut min_val = 0;

    let mut num_remaining = nc;
    // Initialize remaining columns with their indices
    state
        .remaining_columns
        .iter_mut()
        .enumerate()
        .rev()
        .for_each(|(it, item)| {
            *item = it;
        });

    // Reset selected rows, columns, and shortest path costs
    state.selected_rows.fill(false);
    state.selected_columns.fill(false);
    state.shortest_path_costs.fill(i64::MAX);

    let mut sink = usize::MAX;
    while sink == usize::MAX {
        let mut index = usize::MAX;
        let mut lowest = i64::MAX;
        state.selected_rows[current_row] = true;

        // Iterate over remaining columns to find the shortest path
        state
            .remaining_columns
            .iter()
            .take(num_remaining)
            .enumerate()
            .for_each(|(it, &j)| {
                let r: i64 = min_val + cost[current_row * nc + j]
                    - state.dual_row_values[current_row]
                    - state.dual_column_values[j];
                if r < state.shortest_path_costs[j] {
                    state.augmentation_path[j] = current_row;
                    state.shortest_path_costs[j] = r;
                }

                // Update the lowest cost and index
                if state.shortest_path_costs[j] < lowest
                    || (state.shortest_path_costs[j] == lowest
                        && state.row_assigned_to_column[j] == usize::MAX)
                {
                    lowest = state.shortest_path_costs[j];
                    index = it;
                }
            });

        if lowest == i64::MAX {
            return Err(HungarianError::Infeasible.into());
        }
        min_val = lowest;

        let j = state.remaining_columns[index];
        if state.row_assigned_to_column[j] == usize::MAX {
            sink = j;
        } else {
            current_row = state.row_assigned_to_column[j];
        }

        state.selected_columns[j] = true;
        num_remaining -= 1;
        state.remaining_columns.swap(index, num_remaining);
    }

    Ok((sink, min_val))
}

/// Updates the dual variables based on the current row and minimum value found.
fn update_dual_variables(current_row: usize, min_value: i64, state: &mut HungarianState) {
    state.dual_row_values[current_row] += min_value;

    // Update dual values for selected rows
    for (row_idx, row_dual) in state.dual_row_values.iter_mut().enumerate() {
        if state.selected_rows[row_idx] && row_idx != current_row {
            *row_dual +=
                min_value - state.shortest_path_costs[state.column_assigned_to_row[row_idx]];
        }
    }

    // Update dual values for selected columns
    for (col_idx, col_dual) in state.dual_column_values.iter_mut().enumerate() {
        if state.selected_columns[col_idx] {
            *col_dual -= min_value - state.shortest_path_costs[col_idx];
        }
    }
}
/// Augments the solution by updating the assignments based on the found augmenting path.
fn augment_solution(current_row: usize, sink_column: usize, state: &mut HungarianState) {
    let mut column = sink_column;
    loop {
        let row = state.augmentation_path[column];
        state.row_assigned_to_column[column] = row;

        // Swap the column assigned to the row with the current column
        std::mem::swap(&mut state.column_assigned_to_row[row], &mut column);

        if row == current_row {
            break;
        }
    }
}

impl Solve for Hungarian {
    fn solve(&mut self, distance_matrix: &DistanceMatrix) -> Result<Vec<usize>, PhomoError> {
        let d_matrix = if self.config.max_tile_occurrences > 1 {
            &distance_matrix.tile(self.config.max_tile_occurrences)
        } else {
            distance_matrix
        };
        if d_matrix.columns < d_matrix.rows {
            return Err(SolverError::TooFewColumns {
                rows: d_matrix.rows,
                columns: d_matrix.columns,
            }
            .into());
        }
        let mut state = HungarianState {
            dual_row_values: vec![0; d_matrix.rows],
            dual_column_values: vec![0; d_matrix.columns],
            shortest_path_costs: vec![i64::MAX; d_matrix.columns],
            augmentation_path: vec![UNASSIGNED; d_matrix.columns],
            column_assigned_to_row: vec![UNASSIGNED; d_matrix.rows],
            row_assigned_to_column: vec![UNASSIGNED; d_matrix.columns],
            selected_rows: vec![false; d_matrix.rows],
            selected_columns: vec![false; d_matrix.columns],
            remaining_columns: (0..d_matrix.columns).collect(),
        };

        for current_row in maybe_progress_bar!(0..d_matrix.rows, "Computing assignments") {
            let (sink_column, min_value) = find_augmenting_path(current_row, d_matrix, &mut state)?;
            update_dual_variables(current_row, min_value, &mut state);
            augment_solution(current_row, sink_column, &mut state);
        }

        Ok(state.column_assigned_to_row)
    }
}
