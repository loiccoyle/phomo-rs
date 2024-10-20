import React from "react";
import { Grid } from "lucide-react";

interface MosaicPreviewProps {
  mosaicImage: string;
  gridWidth: number;
  gridHeight: number;
}

const MosaicPreview: React.FC<MosaicPreviewProps> = ({
  mosaicImage,
  gridWidth,
  gridHeight,
}) => {
  return (
    <div className="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6 mt-8">
      <div className="flex items-center justify-between mb-4">
        <h2 className="text-2xl font-semibold text-gray-700 dark:text-gray-200">
          Mosaic Preview
        </h2>
        <div className="flex items-center space-x-2">
          <Grid className="text-gray-500 dark:text-gray-400" />
          <span className="text-sm text-gray-500 dark:text-gray-400">
            {gridWidth} x {gridHeight} grid
          </span>
        </div>
      </div>
      <img src={mosaicImage} alt="Generated mosaic" width="100%" />
    </div>
  );
};

export default MosaicPreview;
