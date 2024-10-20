import React, { useEffect, useState } from "react";
import { Grid } from "lucide-react";
import MosaicControls from "./components/MosaicControls";
import Header from "./components/Header";
import { ThemeProvider } from "./contexts/ThemeContext";
import { ColorMatchingMethod } from "./components/colorMatchingMethods";

import { Mosaic, ResizeType, overlayGrid } from "phomo-wasm";

const App: React.FC = () => {
  const [masterImage, setMasterImage] = useState<string | null>(null);
  const [gridOverlay, setGridOverlay] = useState<string | null>(null);
  const [tileImages, setTileImages] = useState<{ url: string; name: string }[]>(
    [],
  );
  const [gridWidth, setGridWidth] = useState(20);
  const [gridHeight, setGridHeight] = useState(20);
  const [colorMatchingMethod, setColorMatchingMethod] = useState(
    ColorMatchingMethod.None,
  );
  const [tileSizingMethod, setTileSizingMethod] = useState(ResizeType.Resize);
  const [mosaicImage, setMosaicImage] = useState<string | null>(null);

  const handleMasterImageSelect = (file: File) => {
    const imageUrl = URL.createObjectURL(file);
    setMasterImage(imageUrl);
  };

  const handleTileImagesSelect = (files: FileList) => {
    const newTileImages = Array.from(files).map((file) => ({
      url: URL.createObjectURL(file),
      name: file.name,
    }));
    setTileImages((prevImages) => [...prevImages, ...newTileImages]);
  };

  const handleRemoveMasterImage = () => {
    setMasterImage(null);
  };

  const handleClearTileImages = () => {
    setTileImages([]);
  };

  const handleRemoveTileImage = (index: number) => {
    setTileImages((prevImages) => prevImages.filter((_, i) => i !== index));
  };

  const handleCreateMosaic = async () => {
    if (tileImages.length < gridWidth * gridHeight) {
      alert(`Please select at least ${gridHeight * gridWidth} tile images.`);
      return;
    }

    if (!masterImage) {
      alert("Please select a master image.");
      return;
    }

    try {
      // Convert master image and tile images to byte arrays for WASM
      const masterImageBytes = await fetchImageAsBytes(masterImage);
      const tileImageBytes = await Promise.all(
        tileImages.map((tile) => fetchImageAsBytes(tile.url)),
      );
      // Create the mosaic using WASM
      const mosaic = new Mosaic(
        masterImageBytes,
        tileImageBytes,
        gridWidth,
        gridHeight,
        tileSizingMethod,
      );

      if (colorMatchingMethod === ColorMatchingMethod.MasterToTile) {
        mosaic.transferMasterToTiles();
      } else if (colorMatchingMethod === ColorMatchingMethod.TileToMaster) {
        mosaic.transferTilesToMaster();
      } else if (colorMatchingMethod === ColorMatchingMethod.Equalize) {
        mosaic.equalize();
      }

      // Generate the mosaic and retrieve the final image as base64
      const blueprint = mosaic.buildBlueprint("NormL1");
      const mosaicBase64 = mosaic.renderBlueprint(blueprint);
      // console.log("blueprint:", blueprint);
      // const mosaicBase64_ = mosaic.build("NormL1");

      setMosaicImage(`data:image/png;base64,${mosaicBase64}`);
      if (!mosaicImage) {
        // smooth scroll to bottom of the page
        setTimeout(() => {
          window.scrollTo({
            top: document.body.scrollHeight,
            behavior: "smooth",
          });
        }, 0.5);
      }
    } catch (error) {
      console.error("Error creating mosaic:", error);
      alert("An error occurred while creating the mosaic.");
    }
  };

  // Utility to fetch image as byte array for WASM
  const fetchImageAsBytes = async (url: string): Promise<Uint8Array> => {
    const response = await fetch(url);
    const blob = await response.blob();
    const arrayBuffer = await blob.arrayBuffer();
    return new Uint8Array(arrayBuffer);
  };

  useEffect(() => {
    async function run() {
      if (masterImage === null) return;
      const masterImageBytes = await fetchImageAsBytes(masterImage);
      setGridOverlay(
        `data:image/png;base64,${overlayGrid(masterImageBytes, gridWidth, gridHeight)}`,
      );
    }
    run();
  }, [masterImage, gridWidth, gridHeight]);

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
          )}
        </main>
      </div>
    </ThemeProvider>
  );
};

export default App;
