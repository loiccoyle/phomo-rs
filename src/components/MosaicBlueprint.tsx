import React, { useMemo, useState } from "react";
import { Blueprint as BlueprintType } from "../types/blueprint";
import { Tile } from "../types/tile";
import TileModal from "./TileModal";

interface BlueprintProps {
  blueprint: BlueprintType;
  tileImages: Tile[];
  originalTileImages: Tile[];
}

const MosaicBlueprint: React.FC<BlueprintProps> = ({
  blueprint,
  tileImages,
  originalTileImages,
}) => {
  const [showTileModal, setShowTileModal] = useState<{
    mosaicTile: Tile;
    originalTile: Tile;
  } | null>(null);

  const memoizedCells = useMemo(() => {
    const { cells, cell_width, cell_height, grid_width, grid_height } =
      blueprint;
    const scaleFactor = 100 / (grid_width * cell_width);
    const aspectRatio = (cell_height * grid_height) / (cell_width * grid_width);

    return cells.map((cell, index) => (
      <div
        key={index}
        className="absolute transition-all duration-100 ease-in-out hover:z-10 hover:scale-110 hover:brightness-110 cursor-pointer"
        style={{
          width: `${cell_width * scaleFactor}%`,
          height: `${(cell_height * scaleFactor) / aspectRatio}%`,
          left: `${cell.x * scaleFactor}%`,
          top: `${(cell.y * scaleFactor) / aspectRatio}%`,
          backgroundImage: `url(${tileImages[cell.tile_index].url})`,
          backgroundSize: "100% 100%",
          backgroundPosition: "center",
        }}
        onClick={() =>
          setShowTileModal({
            mosaicTile: tileImages[cell.tile_index],
            originalTile: originalTileImages[cell.tile_index],
          })
        }
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
          mosaicTile={showTileModal.mosaicTile}
          originalTile={showTileModal.originalTile}
          onClose={() => setShowTileModal(null)}
        />
      )}
    </div>
  );
};

export default MosaicBlueprint;
