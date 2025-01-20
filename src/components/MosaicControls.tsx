import React, { useEffect, useMemo, useState } from "react";
import {
  Upload,
  Play,
  Palette,
  X,
  Image as ImageIcon,
  Crop,
  Maximize,
  File,
  Folder,
  Ratio,
  Grid,
  ChevronDown,
  ChevronUp,
} from "lucide-react";
import TileManagementModal from "./TileManagementModal";
import { ResizeType } from "phomo-wasm";
import { ColorMatchingMethod } from "../types/colorMatchingMethods";
import { UserImage } from "../types/userImage";

interface MosaicControlsProps {
  gridWidth: number;
  gridHeight: number;
  tileImages: UserImage[];
  tileRepeats: number;
  masterImage: UserImage | null;
  gridOverlay: string | null;
  colorMatchingMethod: string;
  tileSizingMethod: ResizeType;
  onMasterImageSelect: (file: File) => void;
  onTileImagesSelect: (files: FileList) => void;
  onTileRepeatsChange: (n: number) => void;
  onGridWidthChange: (width: number) => void;
  onGridHeightChange: (height: number) => void;
  onCreateMosaic: () => void;
  onRemoveMasterImage: () => void;
  onRemoveTileImage: (index: number) => void;
  onClearTileImages: () => void;
  onColorMatchingMethodChange: (method: ColorMatchingMethod) => void;
  onTileSizingMethodChange: (method: ResizeType) => void;
  onMosaicSizeChange: (size: [number, number] | null) => void;
}

const MosaicControls: React.FC<MosaicControlsProps> = ({
  gridWidth,
  gridHeight,
  tileImages,
  tileRepeats,
  masterImage,
  gridOverlay,
  colorMatchingMethod,
  tileSizingMethod,
  onMasterImageSelect,
  onTileImagesSelect,
  onTileRepeatsChange,
  onGridWidthChange,
  onGridHeightChange,
  onCreateMosaic,
  onRemoveMasterImage,
  onRemoveTileImage,
  onClearTileImages,
  onColorMatchingMethodChange,
  onTileSizingMethodChange,
  onMosaicSizeChange,
}) => {
  const [showGrid, setShowGrid] = useState(false);
  const [matchMasterAspectRatio, setMatchMasterAspectRatio] = useState(false);
  const [isTileModalOpen, setIsTileModalOpen] = useState(false);
  const [upscale, setUpscale] = useState(1);
  const [masterImageSize, setMasterImageSize] = useState<
    [number, number] | null
  >(null);
  const requiredTileImages = Math.ceil((gridWidth * gridHeight) / tileRepeats);
  const isTileImagesEnough = tileImages.length >= requiredTileImages;

  const colorMatchingOptions = [
    {
      value: ColorMatchingMethod.None,
      label: "No palette matching",
      description: "Leave color palette unchanged",
    },
    {
      value: ColorMatchingMethod.Equalize,
      label: "Normalize colors",
      description:
        "Adjust colors of the tiles and master image to cover color space",
    },
    {
      value: ColorMatchingMethod.MasterToTile,
      label: "Master to Tiles",
      description: "Transfer master image color palette to tiles",
    },
    {
      value: ColorMatchingMethod.TileToMaster,
      label: "Tiles to Master",
      description: "Transfer tiles' color palette to the master image",
    },
  ];

  const tileSizingOptions = [
    {
      value: ResizeType.Crop,
      label: "Crop",
      description: "Crop tiles to fit grid cells",
      icon: Crop,
    },
    {
      value: ResizeType.Resize,
      label: "Resize",
      description: "Resize tiles to fit grid cells",
      icon: Maximize,
    },
  ];

  useEffect(() => {
    if (masterImage) {
      const img = new Image();
      img.src = masterImage.url;
      img.onload = () => {
        setMasterImageSize([img.width, img.height]);
      };
    }
  }, [masterImage]);

  const handleGridWidthChange = useMemo(
    () => (width: number) => {
      onGridWidthChange(width);
      if (matchMasterAspectRatio && masterImage) {
        const masterAspectRatio = masterImageSize![0] / masterImageSize![1];
        onGridHeightChange(Math.round(width / masterAspectRatio));
      }
    },
    [masterImage, matchMasterAspectRatio, masterImageSize],
  );

  useEffect(() => {
    if (masterImageSize) {
      onMosaicSizeChange([
        masterImageSize[0] * upscale,
        masterImageSize[1] * upscale,
      ]);
    }
  }, [masterImageSize, upscale]);

  useEffect(() => {
    if (matchMasterAspectRatio) {
      // when the match aspect ratio is enabled, refresh the grid width to also set the grid height
      handleGridWidthChange(gridWidth);
    }
  }, [matchMasterAspectRatio]);

  useEffect(() => {
    if (!masterImage) {
      setMatchMasterAspectRatio(false);
    }
  }, [masterImage]);

  return (
    <div className="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6">
      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        <div>
          <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
            Master Image
          </label>
          <div className="h-44">
            {masterImage ? (
              <div className="relative h-full">
                <img
                  src={masterImage.url}
                  alt="Master"
                  className="w-full h-full object-cover rounded-lg"
                />
                {masterImageSize && (
                  <div className="absolute bottom-0 left-0 right-0 bg-black bg-opacity-50 text-white p-1 text-xs truncate rounded-b-lg">
                    {masterImage.name} {masterImageSize[0]}x{masterImageSize[1]}
                  </div>
                )}
                <button
                  onClick={onRemoveMasterImage}
                  className="absolute top-1 right-1 p-1 bg-red-500 text-white rounded-full hover:bg-red-600 transition-colors"
                >
                  <X size={16} />
                </button>
              </div>
            ) : (
              <label className="flex flex-col items-center justify-center w-full h-full border-2 border-gray-300 dark:border-gray-600 border-dashed rounded-lg cursor-pointer bg-gray-50 dark:bg-gray-700 hover:bg-gray-100 dark:hover:bg-gray-600">
                <div className="flex flex-col items-center justify-center pt-5 pb-6">
                  <Upload className="w-8 h-8 mb-2 text-gray-500 dark:text-gray-400" />
                  <p className="mb-2 text-sm text-gray-500 dark:text-gray-400">
                    <span className="font-semibold">Click to select</span> or
                    drag and drop
                  </p>
                  <p className="text-xs text-gray-500 dark:text-gray-400">
                    PNG or JPG
                  </p>
                </div>
                <input
                  type="file"
                  className="hidden"
                  accept="image/*"
                  onChange={(e) =>
                    e.target.files && onMasterImageSelect(e.target.files[0])
                  }
                />
              </label>
            )}
            <div className="flex items-center space-x-4 h-6">
              <label
                htmlFor="upscale"
                className="text-sm font-medium text-gray-700 dark:text-gray-300 w-1/3"
              >
                <span className="">Upscale x{upscale}</span>
              </label>
              <input
                id="upscale"
                type="range"
                value={upscale}
                onChange={(e) => setUpscale(parseInt(e.target.value))}
                min={1}
                max={10}
                step={1}
                className="w-5/6 h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer dark:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-opacity-75"
                style={{
                  accentColor: "#3b82f6",
                }}
              />
            </div>
          </div>
        </div>
        <div>
          <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
            Tile Images
          </label>
          <div className="flex flex-col h-48">
            <div className="relative h-full">
              <button
                onClick={onClearTileImages}
                className={
                  "absolute top-2 right-2 p-1 bg-red-500 text-white rounded-full hover:bg-red-600 transition-colors " +
                  (tileImages.length === 0 ? "hidden" : "")
                }
              >
                <X size={16} />
              </button>
            </div>
            <div className="flex flex-col items-center justify-center w-full h-full border-2 border-gray-300 dark:border-gray-600 border-dashed rounded-lg bg-gray-50 dark:bg-gray-700 mb-1">
              <div className="flex flex-col items-center justify-center pt-1 pb-2">
                <Upload className="w-8 h-8 mb-2 text-gray-500 dark:text-gray-400" />
                <p className="mb-2 text-sm text-gray-500 dark:text-gray-400">
                  <span className="font-semibold">Select</span> or drag and drop
                </p>
                <div className="flex justify-center gap-2">
                  <div>
                    <input
                      type="file"
                      className="hidden"
                      accept="image/*"
                      multiple
                      id="file-input"
                      onChange={(e) => {
                        const files = e.target.files;
                        if (files) {
                          // Filter files to include only valid images (PNG, JPG)
                          const imageFiles = Array.from(files).filter((file) =>
                            ["image/png", "image/jpeg"].includes(file.type),
                          );
                          onTileImagesSelect(imageFiles as unknown as FileList);
                        }
                      }}
                    />
                    <label
                      htmlFor="file-input"
                      className="flex cursor-pointer bg-gray-200 dark:bg-gray-600 p-2 rounded-md hover:bg-gray-300 dark:hover:bg-gray-500"
                    >
                      <File className="w-4 h-4 text-blue-600 dark:text-blue-500" />
                    </label>
                  </div>
                  <div>
                    <input
                      type="file"
                      className="hidden"
                      multiple
                      ref={(input) => {
                        if (input) {
                          // Set webkitdirectory manually for directory selection
                          input.setAttribute("webkitdirectory", "true");
                          input.setAttribute("mozdirectory", "true");
                          input.setAttribute("msdirectory", "true");
                          input.setAttribute("odirectory", "true");
                          input.setAttribute("directory", "true");
                        }
                      }}
                      id="directory-input"
                      onChange={(e) => {
                        const files = e.target.files;
                        if (files) {
                          // Filter files to include only valid images (PNG, JPG)
                          const imageFiles = Array.from(files).filter((file) =>
                            ["image/png", "image/jpeg"].includes(file.type),
                          );
                          onTileImagesSelect(imageFiles as unknown as FileList);
                        }
                      }}
                    />
                    <label
                      htmlFor="directory-input"
                      className="flex cursor-pointer bg-gray-200 dark:bg-gray-600 p-2 rounded-md hover:bg-gray-300 dark:hover:bg-gray-500"
                    >
                      <Folder className="w-4 h-4 text-blue-600 dark:text-blue-500" />
                    </label>
                  </div>
                </div>
                <p
                  className={`text-sm ${isTileImagesEnough ? "text-green-600 dark:text-green-400" : "text-red-600 dark:text-red-400"}`}
                >
                  {tileImages.length} / {requiredTileImages} images selected
                </p>
              </div>
            </div>
            <div className="flex items-center w-full justify-between pb-1">
              <label
                htmlFor="tileRepeats"
                className="text-sm font-medium text-gray-700 dark:text-gray-300 w-1/3"
              >
                <span className="">Tile Repeats</span>
              </label>
              <div className="flex items-center space-x-4 h-6 w-full">
                <input
                  id="tileRepeats"
                  type="range"
                  min="1"
                  max="10"
                  value={tileRepeats}
                  onChange={(e) =>
                    onTileRepeatsChange(parseInt(e.target.value))
                  }
                  className="w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer dark:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-opacity-75"
                  style={{
                    accentColor: "#3b82f6",
                  }}
                />
                <input
                  id="tileRepeats"
                  type="number"
                  min="2"
                  value={tileRepeats}
                  onChange={(e) => {
                    onTileRepeatsChange(Math.max(parseInt(e.target.value), 1));
                  }}
                  className="w-12 text-center rounded-md text-gray-700 bg-gray-200 dark:bg-gray-700 dark:text-gray-300"
                  disabled={matchMasterAspectRatio}
                />
              </div>
            </div>
            <button
              disabled={!tileImages.length}
              onClick={() => setIsTileModalOpen(true)}
              className={
                "flex items-center justify-center px-4 py-2 rounded-lg mt-auto" +
                (tileImages.length === 0
                  ? " bg-gray-200 dark:bg-gray-600 text-gray-400"
                  : " bg-blue-500 text-white hover:bg-blue-600 transition-colors")
              }
            >
              <ImageIcon className="w-5 h-5 mr-2" />
              Manage
            </button>
          </div>
        </div>
      </div>
      <div className="mt-6">
        <div className="flex justify-between">
          <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
            Grid Size
          </label>
          <div className="flex items-center space-x-2">
            <Grid className="text-gray-500 dark:text-gray-400" />
            <span className="font-medium text-gray-500 dark:text-gray-400">
              {gridWidth && gridHeight ? `${gridWidth}x${gridHeight}` : ""}
            </span>
          </div>
        </div>
        <div className="flex sm:flex-row flex-col items-center gap-4">
          <div
            className={`p-4 rounded-lg transition-colors sm:w-2/3 w-full ${
              masterImage
                ? matchMasterAspectRatio
                  ? "bg-blue-100 dark:bg-blue-900 border-2 border-blue-500 cursor-pointer"
                  : "bg-gray-100 dark:bg-gray-700 border-2 dark:border-gray-600 hover:bg-gray-200 dark:hover:bg-gray-600 cursor-pointer"
                : "opacity-50 cursor-default"
            }`}
            onClick={
              masterImage
                ? () => setMatchMasterAspectRatio((prev) => !prev)
                : undefined
            }
          >
            <div className="flex items-center mb-2">
              <Ratio className="w-5 h-5 mr-2 text-blue-500" />
              <h3 className="font-medium text-gray-800 dark:text-gray-200">
                Aspect ratio
              </h3>
            </div>
            <p className="text-sm text-gray-600 dark:text-gray-400">
              Match master image aspect ratio
            </p>
          </div>
          <div className="flex gap-2 flex-col w-full">
            <div className="flex items-center space-x-4 h-6">
              <label
                htmlFor="gridWidth"
                className="text-sm font-medium text-gray-700 dark:text-gray-300 w-1/6"
              >
                <span className="">Width</span>
              </label>
              <input
                id="gridWidth"
                type="range"
                min="2"
                value={gridWidth}
                onChange={(e) =>
                  handleGridWidthChange(parseInt(e.target.value))
                }
                className="w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer dark:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-opacity-75"
                style={{
                  accentColor: "#3b82f6",
                }}
              />
              <input
                id="gridWidth"
                type="number"
                min="2"
                value={gridWidth}
                onChange={(e) => {
                  handleGridWidthChange(Math.max(parseInt(e.target.value), 2));
                }}
                className="w-12 text-center rounded-md text-gray-700 bg-gray-200 dark:bg-gray-700 dark:text-gray-300"
              />
            </div>
            <div className="flex items-center space-x-4 h-6">
              <label
                htmlFor="gridHeight"
                className="text-sm font-medium text-gray-700 dark:text-gray-300 w-1/6"
              >
                <span className="">Height</span>
              </label>
              <input
                id="gridHeight"
                type="range"
                min="2"
                value={gridHeight}
                onChange={(e) => onGridHeightChange(parseInt(e.target.value))}
                className={`w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer dark:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-opacity-75 ${matchMasterAspectRatio ? "opacity-50" : ""}`}
                style={{
                  accentColor: "#3b82f6",
                }}
                disabled={matchMasterAspectRatio}
              />
              <input
                id="gridHeight"
                type="number"
                min="2"
                value={gridHeight}
                onChange={(e) => {
                  onGridHeightChange(Math.max(parseInt(e.target.value), 2));
                }}
                className={`w-12 text-center rounded-md text-gray-700 bg-gray-200 dark:bg-gray-700 dark:text-gray-300 ${matchMasterAspectRatio ? "opacity-50" : ""}`}
                disabled={matchMasterAspectRatio}
              />
            </div>
          </div>
        </div>
        <div className="flex justify-center align-middle items-center flex-col gap-2">
          <div
            className={`flex align-middle bg-gray-100 rounded-2xl dark:bg-gray-700 p-1  ${masterImage ? "cursor-pointer hover:bg-gray-200 dark:hover:bg-gray-600" : "opacity-50"}`}
            onClick={() => masterImage && setShowGrid((prev) => !prev)}
          >
            {showGrid ? (
              <ChevronUp className="text-gray-600 dark:text-gray-300" />
            ) : (
              <ChevronDown className="text-gray-600 dark:text-gray-300" />
            )}
          </div>
          {showGrid && masterImage && gridOverlay && (
            <div className="rounded-md p-2 flex flex-center flex-col justify-center align-middle items-center w-full">
              <img src={gridOverlay} className="max-full rounded-lg" />
            </div>
          )}
        </div>
      </div>
      <div className="mt-6">
        <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
          Palette Matching
        </label>
        <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
          {colorMatchingOptions.map((option) => (
            <div
              key={option.value}
              className={`p-4 rounded-lg cursor-pointer transition-colors ${
                colorMatchingMethod === option.value
                  ? "bg-blue-100 dark:bg-blue-900 border-2 border-blue-500"
                  : "bg-gray-100 dark:bg-gray-700 border-2 dark:border-gray-600 hover:bg-gray-200 dark:hover:bg-gray-600"
              }`}
              onClick={() => onColorMatchingMethodChange(option.value)}
            >
              <div className="flex items-center mb-2">
                <Palette className="w-5 h-5 mr-2 text-blue-500" />
                <h3 className="font-medium text-gray-800 dark:text-gray-200">
                  {option.label}
                </h3>
              </div>
              <p className="text-sm text-gray-600 dark:text-gray-400">
                {option.description}
              </p>
            </div>
          ))}
        </div>
      </div>
      <div className="mt-6">
        <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
          Tile Sizing Method
        </label>
        <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
          {tileSizingOptions.map((option) => (
            <div
              key={option.value}
              className={`p-4 rounded-lg cursor-pointer transition-colors ${
                tileSizingMethod === option.value
                  ? "bg-blue-100 dark:bg-blue-900 border-2 border-blue-500"
                  : "bg-gray-100 dark:bg-gray-700 border-2 dark:border-gray-600 hover:bg-gray-200 dark:hover:bg-gray-600"
              }`}
              onClick={() => onTileSizingMethodChange(option.value)}
            >
              <div className="flex items-center mb-2">
                <option.icon className="w-5 h-5 mr-2 text-blue-500" />
                <h3 className="font-medium text-gray-800 dark:text-gray-200">
                  {option.label}
                </h3>
              </div>
              <p className="text-sm text-gray-600 dark:text-gray-400">
                {option.description}
              </p>
            </div>
          ))}
        </div>
      </div>
      <div className="mt-8 flex justify-center">
        <button
          onClick={onCreateMosaic}
          className={
            "flex items-center justify-center px-6 py-3 rounded-lg transition-colors text-lg font-semibold " +
            (isTileImagesEnough
              ? "bg-green-500 hover:bg-green-600 text-white"
              : "bg-gray-300 dark:bg-gray-700 text-gray-400")
          }
          disabled={!isTileImagesEnough || !masterImage}
        >
          <Play className="w-6 h-6 mr-2" />
          Create Mosaic
        </button>
      </div>
      <TileManagementModal
        isOpen={isTileModalOpen}
        onClose={() => setIsTileModalOpen(false)}
        tileImages={tileImages}
        onRemoveTileImage={onRemoveTileImage}
      />
    </div>
  );
};

export default MosaicControls;
