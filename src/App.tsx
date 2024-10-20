import React from "react";
import { ThemeProvider } from "./contexts/ThemeContext";
import Header from "./components/Header";
import MosaicControls from "./components/MosaicControls";
import MosaicPreview from "./components/MosaicPreview";
import { useMosaicCreation } from "./hooks/useMosaicCreation";
import { useImageSelection } from "./hooks/useImageSelection";

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
    mosaicImage,
    handleCreateMosaic,
  } = useMosaicCreation(masterImage, tileImages, gridWidth, gridHeight);

  return (
    <ThemeProvider>
      <div className="min-h-screen bg-gray-100 dark:bg-gray-900 px-4 sm:px-8 pb-8 pt-4 transition-colors duration-200">
        <Header />
        <main className="container mx-auto">
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
          {mosaicImage && (
            <MosaicPreview
              mosaicImage={mosaicImage}
              gridWidth={gridWidth}
              gridHeight={gridHeight}
            />
          )}
        </main>
      </div>
    </ThemeProvider>
  );
};

export default App;
