import React, { useMemo, useState } from "react";
import { Blueprint as BlueprintType } from "../types/blueprint";
import { Tile } from "../types/tile";
import { ResizeType } from "phomo-wasm";
import TileModal from "./TileModal";

interface BlueprintProps {
  blueprint: BlueprintType;
  tileImages: Tile[];
  tileSizingMethod: ResizeType;
}

const MosaicBlueprint: React.FC<BlueprintProps> = ({
  blueprint,
  tileImages,
  tileSizingMethod,
}) => {
  const [showTileModal, setShowTileModal] = useState<Tile | null>(null);

  const memoizedCells = useMemo(() => {
    const { cells, cell_width, cell_height, grid_width } = blueprint;
    const scaleFactor = 100 / (grid_width * cell_width);

    let backgroundSize: string;
    if (tileSizingMethod === ResizeType.Resize) {
      backgroundSize = "100% 100%";
    } else if (tileSizingMethod === ResizeType.Crop) {
      backgroundSize = "cover";
    } else {
      backgroundSize = "cover"; // Default fallback
    }
    return cells.map((cell, index) => (
      <div
        key={index}
        className="absolute transition-all duration-100 ease-in-out hover:z-10 hover:scale-110 hover:brightness-110 cursor-pointer"
        style={{
          width: `${cell_width * scaleFactor}%`,
          height: `${cell_height * scaleFactor}%`,
          left: `${cell.x * scaleFactor}%`,
          top: `${cell.y * scaleFactor}%`,
          backgroundImage: `url(${tileImages[cell.tile_index].url})`,
          backgroundSize: backgroundSize,
          backgroundPosition: "center",
        }}
        onClick={() => setShowTileModal(tileImages[cell.tile_index])}
      />
    ));
  }, [blueprint, tileImages]);

  const aspectRatio =
    (blueprint.grid_height * blueprint.cell_height) /
    (blueprint.grid_width * blueprint.cell_width);

  return (
    <div
      className="relative w-full overflow-hidden"
      style={{
        paddingBottom: `${aspectRatio * 100}%`,
      }}
    >
      <div className="absolute inset-0">{memoizedCells}</div>
      {showTileModal && (
        <TileModal
          tile={showTileModal}
          onClose={() => setShowTileModal(null)}
        />
      )}
    </div>
  );
};

export default MosaicBlueprint;
