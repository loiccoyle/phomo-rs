import { useMemo, useState, useEffect } from "react";
import { ResizeType } from "phomo-wasm";
import { ColorMatchingMethod } from "../types/colorMatchingMethods";
import { Blueprint } from "../types/blueprint";

// Worker creation function
const createWorker = () => {
  console.log("new worker");
  const worker = new Worker(
    new URL("../workers/mosaicWorker.ts", import.meta.url),
    { type: "module" },
  );
  return worker;
};

export const useMosaicCreation = (
  masterImage: string | null,
  tileImages: { url: string; name: string }[],
  gridWidth: number,
  gridHeight: number,
) => {
  const [buildingMosaic, setBuildingMosaic] = useState(false);
  const [colorMatchingMethod, setColorMatchingMethod] = useState(
    ColorMatchingMethod.None,
  );
  const [mosaicTiles, setMosaicTiles] = useState<string[] | null>(null);
  const [tileSizingMethod, setTileSizingMethod] = useState(ResizeType.Resize);
  const [mosaicImage, setMosaicImage] = useState<string | null>(null);
  const [mosaicBlueprint, setMosaicBlueprint] = useState<Blueprint | null>(
    null,
  );

  // Create worker only once
  const worker = useMemo(() => createWorker(), []);

  // Attach onmessage and onerror handlers only once
  useEffect(() => {
    worker.onmessage = (event) => {
      console.log("message received from worker");
      console.log(event);
      setMosaicImage(event.data.mosaicImage);
      setMosaicBlueprint(event.data.mosaicBlueprint);
      setMosaicTiles(event.data.mosaicTiles);
      setBuildingMosaic(false);
    };

    worker.onerror = (event) => {
      setBuildingMosaic(false);
      console.error(event);
    };

    // Cleanup worker when component unmounts
    return () => {
      worker.terminate();
    };
  }, [worker]);

  const handleCreateMosaic = async () => {
    if (tileImages.length < gridWidth * gridHeight) {
      alert(`Please select at least ${gridHeight * gridWidth} tile images.`);
      return;
    }

    if (!masterImage) {
      alert("Please select a master image.");
      return;
    }

    setBuildingMosaic(true);
    worker.postMessage({
      masterImageUrl: masterImage,
      tileImagesUrls: tileImages.map((tile) => tile.url),
      gridWidth,
      gridHeight,
      tileSizingMethod: tileSizingMethod,
      colorMatchingMethod: colorMatchingMethod,
    });
  };

  return {
    colorMatchingMethod,
    setColorMatchingMethod,
    tileSizingMethod,
    setTileSizingMethod,
    buildingMosaic,
    mosaicTiles,
    mosaicImage,
    mosaicBlueprint,
    handleCreateMosaic,
  };
};
