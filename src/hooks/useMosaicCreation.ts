import { useState } from "react";
import { Mosaic, ResizeType } from "phomo-wasm";
import { ColorMatchingMethod } from "../types/colorMatchingMethods";
import { fetchImageAsBytes } from "../utils/imageUtils";

export const useMosaicCreation = (
  masterImage: string | null,
  tileImages: { url: string; name: string }[],
  gridWidth: number,
  gridHeight: number,
) => {
  const [colorMatchingMethod, setColorMatchingMethod] = useState(
    ColorMatchingMethod.None,
  );
  const [tileSizingMethod, setTileSizingMethod] = useState(ResizeType.Resize);
  const [mosaicImage, setMosaicImage] = useState<string | null>(null);

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
      const masterImageBytes = await fetchImageAsBytes(masterImage);
      const tileImageBytes = await Promise.all(
        tileImages.map((tile) => fetchImageAsBytes(tile.url)),
      );

      const mosaic = new Mosaic(
        masterImageBytes,
        tileImageBytes,
        gridWidth,
        gridHeight,
        tileSizingMethod,
      );

      switch (colorMatchingMethod) {
        case ColorMatchingMethod.MasterToTile:
          mosaic.transferMasterToTiles();
          break;
        case ColorMatchingMethod.TileToMaster:
          mosaic.transferTilesToMaster();
          break;
        case ColorMatchingMethod.Equalize:
          mosaic.equalize();
          break;
      }

      const blueprint = mosaic.buildBlueprint("NormL1");
      const mosaicBase64 = mosaic.renderBlueprint(blueprint);

      setMosaicImage(`data:image/png;base64,${mosaicBase64}`);

      if (!mosaicImage) {
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

  return {
    colorMatchingMethod,
    setColorMatchingMethod,
    tileSizingMethod,
    setTileSizingMethod,
    mosaicImage,
    handleCreateMosaic,
  };
};
