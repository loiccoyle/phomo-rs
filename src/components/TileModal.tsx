import React, { useState } from "react";
import { X, FileImage } from "lucide-react";
import { Tile } from "../types/tile";

interface TileModalProps {
  mosaicTile: Tile;
  originalTile: Tile;
  onClose: () => void;
}

const TileModal: React.FC<TileModalProps> = ({
  mosaicTile,
  originalTile,
  onClose,
}) => {
  const [showOriginalTile, setShowOriginalTile] = useState(false);
  return (
    <div
      className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50"
      onClick={onClose}
    >
      <div
        className="bg-white dark:bg-gray-800 rounded-lg p-4 max-w-2xl w-full mx-4"
        onClick={(e) => e.stopPropagation()}
      >
        <div className="text-right">
          <button
            onClick={onClose}
            className="text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-200 transition-colors text-right"
          >
            <X size={24} />
          </button>
        </div>
        <div className="relative">
          <img
            src={showOriginalTile ? originalTile.url : mosaicTile.url}
            alt={`Full size tile ${mosaicTile.name}`}
            className="w-full h-auto rounded-lg"
          />
          <div className="absolute bottom-0 left-0 right-0 bg-black bg-opacity-50 text-white p-1 text-xs truncate rounded-b-lg">
            {mosaicTile.name}{" "}
            {showOriginalTile ? "(original)" : "(mosaic tile)"}
          </div>
          <button
            className="absolute top-1 right-1 p-1 bg-gray-700 rounded-md text-gray-400 hover:text-gray-300"
            onClick={() => setShowOriginalTile(!showOriginalTile)}
          >
            <FileImage />
          </button>
        </div>
      </div>
    </div>
  );
};

export default TileModal;
