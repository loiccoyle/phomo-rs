import { useState } from "react";
import { UserImage } from "../types/userImage";

export const useImageSelection = () => {
  const [masterImage, setMasterImage] = useState<UserImage | null>(null);
  const [mosaicImageSize, setMosaicImageSize] = useState<
    [number, number] | null
  >(null);
  const [tileImages, setTileImages] = useState<UserImage[]>([]);
  const [gridWidth, setGridWidth] = useState(20);
  const [gridHeight, setGridHeight] = useState(20);
  const [tileRepeats, setTileRepeats] = useState(1);

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

  return {
    masterImage,
    mosaicImageSize,
    tileImages,
    tileRepeats,
    gridWidth,
    gridHeight,
    handleMasterImageSelect,
    setMosaicImageSize,
    handleTileImagesSelect,
    setTileRepeats,
    handleRemoveMasterImage,
    handleClearTileImages,
    handleRemoveTileImage,
    setGridWidth,
    setGridHeight,
  };
};
