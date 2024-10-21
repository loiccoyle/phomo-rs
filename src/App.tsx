import React from "react";
import { ThemeProvider } from "./contexts/ThemeContext";
import Header from "./components/Header";
import MosaicControls from "./components/MosaicControls";
import { useMosaicCreation } from "./hooks/useMosaicCreation";
import { useImageSelection } from "./hooks/useImageSelection";
import MosaicBlueprint from "./components/MosaicBlueprint";
import { Download } from "lucide-react";

const App: React.FC = () => {
  const {
    masterImage,
    gridOverlay,
    tileImages,
    gridWidth,
    gridHeight,
    handleMasterImageSelect,
    handleTileImagesSelect,
    handleRemoveMasterImage,
    handleClearTileImages,
    handleRemoveTileImage,
    setGridWidth,
    setGridHeight,
  } = useImageSelection();

  const {
    colorMatchingMethod,
    setColorMatchingMethod,
    tileSizingMethod,
    setTileSizingMethod,
    mosaic,
    mosaicImage,
    mosaicBlueprint,
    handleCreateMosaic,
  } = useMosaicCreation(masterImage, tileImages, gridWidth, gridHeight);

  return (
    <ThemeProvider>
      <div className="min-h-screen bg-gray-100 dark:bg-gray-900 px-4 sm:px-8 pb-8 pt-4 transition-colors duration-200">
        <main className="container mx-auto">
          <Header />
          <MosaicControls
            onMasterImageSelect={handleMasterImageSelect}
            onTileImagesSelect={handleTileImagesSelect}
            onGridWidthChange={setGridWidth}
            onGridHeightChange={setGridHeight}
            onCreateMosaic={handleCreateMosaic}
            gridWidth={gridWidth}
            gridHeight={gridHeight}
            tileImages={tileImages}
            masterImage={masterImage}
            gridOverlay={gridOverlay}
            onRemoveMasterImage={handleRemoveMasterImage}
            onRemoveTileImage={handleRemoveTileImage}
            onClearTileImages={handleClearTileImages}
            colorMatchingMethod={colorMatchingMethod}
            setColorMatchingMethod={setColorMatchingMethod}
            tileSizingMethod={tileSizingMethod}
            setTileSizingMethod={setTileSizingMethod}
          />
          {mosaicBlueprint && mosaic && (
            <div className="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6 mt-8">
              <div className="flex items-center align-center justify-center">
                <MosaicBlueprint
                  blueprint={mosaicBlueprint}
                  originalTileImages={tileImages}
                  tileImages={mosaic.getTiles().map((url, i) => {
                    return {
                      url: `data:image/png;base64,${url}`,
                      name: tileImages[i].name,
                    };
                  })}
                />
              </div>
              <div className="mt-8 flex justify-center">
                <button
                  className="flex items-center justify-center px-6 py-3 rounded-lg transition-colors text-lg font-semibold bg-green-500 hover:bg-green-600 text-white"
                  onClick={() => {
                    if (!mosaicImage) return;
                    const link = document.createElement("a");
                    link.href = mosaicImage;
                    link.download = "mosaic.png";
                    link.click();
                  }}
                >
                  <Download className="w-6 h-6 mr-2" />
                  Download
                </button>
              </div>
            </div>
          )}
        </main>
      </div>
    </ThemeProvider>
  );
};

export default App;
