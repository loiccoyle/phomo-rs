import { useState, useEffect } from "react";
import { overlayGrid } from "phomo-wasm";
import { fetchImageAsBytes } from "../utils/imageUtils";

export const useImageSelection = () => {
  const [masterImage, setMasterImage] = useState<string | null>(null);
  const [gridOverlay, setGridOverlay] = useState<string | null>(null);
  const [tileImages, setTileImages] = useState<{ url: string; name: string }[]>(
    [],
  );
  const [gridWidth, setGridWidth] = useState(20);
  const [gridHeight, setGridHeight] = useState(20);

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

  useEffect(() => {
    async function updateGridOverlay() {
      if (masterImage === null) return;
      const masterImageBytes = await fetchImageAsBytes(masterImage);
      setGridOverlay(
        `data:image/png;base64,${overlayGrid(masterImageBytes, gridWidth, gridHeight)}`,
      );
    }
    updateGridOverlay();
  }, [masterImage, gridWidth, gridHeight]);

  return {
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
  };
};
