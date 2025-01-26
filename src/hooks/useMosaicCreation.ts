import { useMemo, useState, useEffect } from "react";
import { MetricType, ResizeType, Solver } from "phomo-wasm";
import { ColorMatchingMethod } from "../types/colorMatchingMethods";
import { Blueprint } from "../types/blueprint";
import { UserImage } from "../types/userImage";

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
  masterImage: UserImage | null,
  tileImages: UserImage[],
  tileRepeats: number,
  gridWidth: number,
  gridHeight: number,
  mosaicImageSize: [number, number] | null,
) => {
  const [buildingMosaic, setBuildingMosaic] = useState(false);
  const [colorMatchingMethod, setColorMatchingMethod] = useState(
    ColorMatchingMethod.None,
  );
  const [mosaicTiles, setMosaicTiles] = useState<string[] | null>(null);
  const [tileSizingMethod, setTileSizingMethod] = useState(ResizeType.Crop);
  const [mosaicImage, setMosaicImage] = useState<string | null>(null);
  const [mosaicBlueprint, setMosaicBlueprint] = useState<Blueprint | null>(
    null,
  );
  const [metric, setMetric] = useState(MetricType.NormL1);
  const [solver, setSolver] = useState(Solver.Hungarian);

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
    const requiredTileImages = Math.ceil(
      (gridWidth * gridHeight) / tileRepeats,
    );
    if (tileImages.length < requiredTileImages) {
      alert(`Please select at least ${requiredTileImages} tile images.`);
      return;
    }

    if (!masterImage) {
      alert("Please select a master image.");
      return;
    }

    setBuildingMosaic(true);
    worker.postMessage({
      masterImageUrl: masterImage.url,
      tileImagesUrls: tileImages.map((tile) => tile.url),
      gridWidth,
      gridHeight,
      tileSizingMethod,
      tileRepeats,
      solver,
      metric,
      colorMatchingMethod,
      mosaicImageSize,
    });
  };

  return {
    colorMatchingMethod,
    setColorMatchingMethod,
    tileSizingMethod,
    setTileSizingMethod,
    metric,
    setMetric,
    solver,
    setSolver,
    buildingMosaic,
    mosaicTiles,
    mosaicImage,
    mosaicBlueprint,
    handleCreateMosaic,
    mosaicImageSize,
  };
};
