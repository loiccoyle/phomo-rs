import { useState, useEffect } from "react";
import { overlayGrid } from "phomo-wasm";
import { fetchImageAsBytes } from "../utils/imageUtils";
import { UserImage } from "../types/userImage";

export const useImageSelection = () => {
  const [masterImage, setMasterImage] = useState<UserImage | null>(null);
  const [mosaicImageSize, setMosaicImageSize] = useState<
    [number, number] | null
  >(null);
  const [gridOverlay, setGridOverlay] = useState<string | null>(null);
  const [tileImages, setTileImages] = useState<UserImage[]>([]);
  const [gridWidth, setGridWidth] = useState(20);
  const [gridHeight, setGridHeight] = useState(20);

  const handleMasterImageSelect = (file: File) => {
    const imageUrl = URL.createObjectURL(file);
    setMasterImage({ url: imageUrl, name: file.name });
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
      const masterImageBytes = await fetchImageAsBytes(masterImage.url);
      setGridOverlay(
        `data:image/png;base64,${overlayGrid(masterImageBytes, gridWidth, gridHeight)}`,
      );
    }
    updateGridOverlay();
  }, [masterImage, gridWidth, gridHeight]);

  return {
    masterImage,
    mosaicImageSize,
    gridOverlay,
    tileImages,
    gridWidth,
    gridHeight,
    handleMasterImageSelect,
    setMosaicImageSize,
    handleTileImagesSelect,
    handleRemoveMasterImage,
    handleClearTileImages,
    handleRemoveTileImage,
    setGridWidth,
    setGridHeight,
  };
};
