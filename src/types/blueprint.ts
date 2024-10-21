export interface Cell {
  tile_index: number;
  x: number;
  y: number;
}

export interface Blueprint {
  cells: Cell[];
  cell_width: number;
  cell_height: number;
  grid_width: number;
  grid_height: number;
}
