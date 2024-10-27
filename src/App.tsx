import React from "react";
import { ThemeProvider } from "./contexts/ThemeContext";
import Header from "./components/Header";
import MosaicControls from "./components/MosaicControls";
import { useMosaicCreation } from "./hooks/useMosaicCreation";
import { useImageSelection } from "./hooks/useImageSelection";
import MosaicBlueprint from "./components/MosaicBlueprint";
import { Download, Loader } from "lucide-react";
import Footer from "./components/Footer";

const App: React.FC = () => {
  const {
    masterImage,
    mosaicImageSize,
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
    setMosaicImageSize,
  } = useImageSelection();

  const {
    colorMatchingMethod,
    setColorMatchingMethod,
    tileSizingMethod,
    setTileSizingMethod,
    buildingMosaic,
    mosaicTiles,
    mosaicImage,
    mosaicBlueprint,
    handleCreateMosaic,
  } = useMosaicCreation(
    masterImage,
    tileImages,
    gridWidth,
    gridHeight,
    mosaicImageSize,
  );

  return (
    <ThemeProvider>
      <div className="min-h-screen bg-gray-100 dark:bg-gray-900 px-4 sm:px-8 pb-8 pt-4 transition-colors duration-200">
        <main className="container mx-auto">
          <Header />
          <MosaicControls
            gridWidth={gridWidth}
            gridHeight={gridHeight}
            tileImages={tileImages}
            masterImage={masterImage}
            gridOverlay={gridOverlay}
            colorMatchingMethod={colorMatchingMethod}
            tileSizingMethod={tileSizingMethod}
            onMasterImageSelect={handleMasterImageSelect}
            onTileImagesSelect={handleTileImagesSelect}
            onGridWidthChange={setGridWidth}
            onGridHeightChange={setGridHeight}
            onCreateMosaic={handleCreateMosaic}
            onRemoveMasterImage={handleRemoveMasterImage}
            onRemoveTileImage={handleRemoveTileImage}
            onClearTileImages={handleClearTileImages}
            onColorMatchingMethodChange={setColorMatchingMethod}
            onTileSizingMethodChange={setTileSizingMethod}
            onMosaicSizeChange={setMosaicImageSize}
          />
          <div className="mt-8">
            {buildingMosaic ? (
              <div className="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6 flex items-center justify-center">
                <Loader className="w-8 h-8 text-blue-500 animate-spin mr-2" />
                <span className="text-lg font-semibold dark:text-white">
                  Building Mosaic...
                </span>
              </div>
            ) : mosaicBlueprint && mosaicTiles && tileImages ? (
              <div className="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6">
                <div className="flex items-center align-center justify-center">
                  <MosaicBlueprint
                    blueprint={mosaicBlueprint}
                    originalTileImages={tileImages}
                    tileImages={mosaicTiles.map((url, i) => ({
                      url: `data:image/png;base64,${url}`,
                      name: tileImages[i].name,
                    }))}
                  />
                </div>
                <div className="mt-8 flex justify-center">
                  <button
                    className="flex items-center justify-center px-6 py-3 rounded-lg transition-colors text-lg font-semibold bg-green-500 hover:bg-green-600 text-white"
                    onClick={() => {
                      if (!mosaicImage) return;
                      // Convert raw base64 string to a Blob
                      const byteString = atob(mosaicImage);
                      const ab = new ArrayBuffer(byteString.length);
                      const ia = new Uint8Array(ab);
                      for (let i = 0; i < byteString.length; i++) {
                        ia[i] = byteString.charCodeAt(i);
                      }
                      // Define the MIME type of your image (e.g., PNG)
                      const blob = new Blob([ia], { type: "image/png" });
                      // Create a temporary URL for downloading the Blob
                      const link = document.createElement("a");
                      link.href = URL.createObjectURL(blob);
                      link.download = "mosaic.png";
                      link.click();
                      // Clean up the object URL after download
                      URL.revokeObjectURL(link.href);
                    }}
                  >
                    <Download className="w-6 h-6 mr-2" />
                    Download
                  </button>
                </div>
              </div>
            ) : null}
          </div>
        </main>
        <Footer />
      </div>
    </ThemeProvider>
  );
};

export default App;
