import { useEffect, useState } from "react";
import { MetricType, ResizeType, Solver } from "phomo-wasm";
import { ColorMatchingMethod } from "../types/colorMatchingMethods";
import { Blueprint } from "../types/blueprint";
import { UserImage } from "../types/userImage";

// Worker creation function
const createWorker = () => {
  console.log("Creating new worker");
  return new Worker(new URL("../workers/mosaicWorker.ts", import.meta.url), {
    type: "module",
  });
};

export const useMosaicCreation = (
  masterImage: UserImage | null,
  tileImages: UserImage[],
  tileRepeats: number,
  gridWidth: number,
  gridHeight: number,
  mosaicImageSize: [number, number] | null,
) => {
  const [worker, setWorker] = useState<Worker>(() => createWorker());
  const [buildingMosaic, setBuildingMosaic] = useState(false);
  const [mosaicTiles, setMosaicTiles] = useState<string[] | null>(null);
  const [mosaicImage, setMosaicImage] = useState<string | null>(null);
  const [mosaicBlueprint, setMosaicBlueprint] = useState<Blueprint | null>(
    null,
  );
  const [colorMatchingMethod, setColorMatchingMethod] = useState(
    ColorMatchingMethod.None,
  );
  const [tileSizingMethod, setTileSizingMethod] = useState(ResizeType.Crop);
  const [metric, setMetric] = useState(MetricType.NormL1);
  const [solver, setSolver] = useState(Solver.Hungarian);

  // Attach event listeners to the worker.
  // We create a function so we can reattach these to a new worker when needed.
  const setupWorkerListeners = (w: Worker) => {
    w.onmessage = (event) => {
      console.log("Message received from worker:", event);
      setBuildingMosaic(false);

      // If the worker reports an error (e.g. during initialization), we can recreate it.
      if (event.data.error) {
        alert(`Error while building mosaic: ${event.data.error}`);
        // Optionally, recreate the worker here if needed.
        const newWorker = createWorker();
        setupWorkerListeners(newWorker);
        setWorker(newWorker);
        return;
      }

      // Process a valid result
      setMosaicImage(event.data.mosaicImage);
      setMosaicBlueprint(event.data.mosaicBlueprint);
      setMosaicTiles(event.data.mosaicTiles);
    };

    w.onerror = (event) => {
      setBuildingMosaic(false);
      console.error("Worker error:", event);
      alert(`Error while building the mosaic: ${event.error}`);

      // Terminate the worker that encountered an error.
      w.terminate();

      // Create a new worker, attach listeners, and update state.
      const newWorker = createWorker();
      setupWorkerListeners(newWorker);
      setWorker(newWorker);
    };
  };

  // Set up listeners on first render.
  useEffect(() => {
    setupWorkerListeners(worker);
    // Cleanup when the component unmounts
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

    // Send parameters to the worker
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
