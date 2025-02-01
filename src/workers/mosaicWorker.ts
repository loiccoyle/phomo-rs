import { Mosaic } from "phomo-wasm";
import { fetchImageAsBytes } from "../utils/imageUtils";
import { ColorMatchingMethod } from "../types/colorMatchingMethods";

const fetchImagesAsBytes = async (urls: string[]): Promise<Uint8Array[]> => {
  return Promise.all(urls.map((url) => fetchImageAsBytes(url)));
};

self.onmessage = async (event) => {
  console.log(event);
  const {
    masterImageUrl,
    tileImagesUrls,
    gridWidth,
    gridHeight,
    tileSizingMethod,
    tileRepeats,
    solver,
    metric,
    colorMatchingMethod,
    mosaicImageSize,
  } = event.data;

  console.log(event.data);
  try {
    const masterImageBytes = await fetchImageAsBytes(masterImageUrl);
    const tileImageBytes = await fetchImagesAsBytes(tileImagesUrls);
    const mosaic = new Mosaic(
      masterImageBytes,
      tileImageBytes,
      gridWidth,
      gridHeight,
      tileRepeats,
      tileSizingMethod,
      mosaicImageSize ? Uint32Array.from(mosaicImageSize) : undefined,
    );
    self.postMessage({ type: "log", message: "Mosaic initialized" });

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

    const blueprint = mosaic.buildBlueprintWithSolver(metric, solver);
    self.postMessage({ type: "log", message: "Built blueprint" });

    const mosaicBase64 = mosaic.renderBlueprint(blueprint);
    self.postMessage({ type: "log", message: "Rendered blueprint" });

    self.postMessage({
      mosaicTiles: mosaic.getTiles(),
      mosaicBlueprint: blueprint,
      mosaicImage: mosaicBase64,
      type: "mosaic",
    });
  } catch (error) {
    let errorMessage = "An unknown error occurred.";
    if (error instanceof Error) {
      errorMessage = error.message;
    } else if (typeof error === "string") {
      errorMessage = error;
    }

    self.postMessage({ message: errorMessage, type: "error" });
  }
};
