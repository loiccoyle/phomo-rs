import { Mosaic } from "phomo-wasm";
import { fetchImageAsBytes } from "../utils/imageUtils";
import { ColorMatchingMethod } from "../types/colorMatchingMethods";
import { TileAssignment } from "../types/tileAssignment";

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
    tileAssignmentMethod,
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

    let blueprint;
    switch (tileAssignmentMethod) {
      case TileAssignment.Optimal:
        blueprint = mosaic.buildBlueprint("NormL1");
        break;
      case TileAssignment.Greedy:
        blueprint = mosaic.buildBlueprintGreedy("NormL1");
        break;
    }

    const mosaicBase64 = mosaic.renderBlueprint(blueprint);

    self.postMessage({
      mosaicTiles: mosaic.getTiles(),
      mosaicBlueprint: blueprint,
      mosaicImage: mosaicBase64,
    });
  } catch (error) {
    let errorMessage = "An unknown error occurred.";
    if (error instanceof Error) {
      errorMessage = error.message; // If it's a standard Error object
    } else if (typeof error === "string") {
      errorMessage = error; // If the error is a string
    }

    self.postMessage({ error: errorMessage });
  }
};
