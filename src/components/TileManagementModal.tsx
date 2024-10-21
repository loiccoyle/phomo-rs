import React from "react";
import { X } from "lucide-react";

interface TileManagementModalProps {
  isOpen: boolean;
  onClose: () => void;
  tileImages: { url: string; name: string }[];
  onRemoveTileImage: (index: number) => void;
}

const TileManagementModal: React.FC<TileManagementModalProps> = ({
  isOpen,
  onClose,
  tileImages,
  onRemoveTileImage,
}) => {
  if (!isOpen) return null;

  return (
    <div
      className="fixed inset-0 bg-black bg-opacity-50 flex flex-col items-center justify-center z-50"
      onClick={onClose}
    >
      <div
        className="bg-white dark:bg-gray-800 rounded-lg px-6 mt-1 w-full max-w-3xl max-h-[80vh] overflow-y-auto"
        onClick={(e) => e.stopPropagation()}
      >
        <div className="flex justify-between items-center mb-4 pt-2 sticky top-0 z-50 bg-white dark:bg-gray-800">
          <h3 className="text-xl font-semibold text-gray-800 dark:text-gray-200">
            Tile Images
          </h3>
          <button
            onClick={onClose}
            className="text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-200 transition-colors"
          >
            <X size={24} />
          </button>
        </div>
        <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-4">
          {tileImages.map((tile, index) => (
            <div key={index} className="relative">
              <img
                src={tile.url}
                alt={`Tile ${index}`}
                className="w-full h-32 object-cover rounded-lg"
              />
              <div className="absolute bottom-0 left-0 right-0 bg-black bg-opacity-50 text-white p-1 text-xs truncate rounded-b-lg">
                {tile.name}
              </div>
              <button
                onClick={() => onRemoveTileImage(index)}
                className="absolute top-1 right-1 p-1 bg-red-500 text-white rounded-full hover:bg-red-600 transition-colors"
              >
                <X size={16} />
              </button>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
};

export default TileManagementModal;
