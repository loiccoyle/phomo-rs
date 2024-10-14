import React from "react";

interface MosaicGridProps {
  onTileClick: (tileUrl: string, index: number) => void;
  gridSize: number;
  masterImage: string | null;
}

const MosaicGrid: React.FC<MosaicGridProps> = ({
  onTileClick,
  gridSize,
  masterImage,
}) => {
  const tileSize = 100 / gridSize;

  // Use the master image if available, otherwise use a placeholder
  const imageUrl =
    masterImage ||
    "https://images.unsplash.com/photo-1682687982501-1e58ab814714";

  return (
    <div
      className={`grid gap-px bg-gray-200 dark:bg-gray-700 rounded-lg overflow-hidden`}
      style={{ gridTemplateColumns: `repeat(${gridSize}, minmax(0, 1fr))` }}
    >
      {Array.from({ length: gridSize * gridSize }).map((_, index) => {
        const row = Math.floor(index / gridSize);
        const col = index % gridSize;
        return (
          <div
            key={index}
            className="relative aspect-square"
            // cursor-pointer hover:opacity-75 transition-opacity"
            style={{ paddingBottom: `${tileSize}%` }}
            onClick={() => onTileClick(imageUrl, index)}
          >
            <div
              className="absolute inset-0 bg-cover bg-no-repeat"
              style={{
                backgroundImage: `url(${imageUrl})`,
                backgroundPosition: `${(col / (gridSize - 1)) * 100}% ${(row / (gridSize - 1)) * 100}%`,
                backgroundSize: `${gridSize * 100}%`,
              }}
            />
          </div>
        );
      })}
    </div>
  );
};

export default MosaicGrid;

