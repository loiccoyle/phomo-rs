import React, { useState, useEffect, useRef } from "react";

interface MosaicGridProps {
  masterImage: string | null;
  gridWidth: number;
  gridHeight: number;
}

const MosaicGrid: React.FC<MosaicGridProps> = ({
  masterImage,
  gridWidth,
  gridHeight,
}) => {
  const [imageDimensions, setImageDimensions] = useState({
    width: 0,
    height: 0,
  });
  const imageRef = useRef<HTMLImageElement>(null);

  useEffect(() => {
    const updateDimensions = () => {
      if (imageRef.current) {
        setImageDimensions({
          width: imageRef.current.offsetWidth,
          height: imageRef.current.offsetHeight,
        });
      }
    };

    window.addEventListener("resize", updateDimensions);
    updateDimensions();

    return () => window.removeEventListener("resize", updateDimensions);
  }, []);

  const cellWidth = Math.floor(imageDimensions.width / gridWidth);
  const cellHeight = Math.floor(imageDimensions.height / gridHeight);

  const gridStyle = {
    display: "grid",
    gridTemplateColumns: `repeat(${gridWidth}, ${cellWidth}px)`,
    gridTemplateRows: `repeat(${gridHeight}, ${cellHeight}px)`,
    width: `${cellWidth * gridWidth}px`,
    height: `${cellHeight * gridHeight}px`,
  };

  if (masterImage === null) {
    return <div></div>;
  }

  return (
    <div className="relative inline-block w-full">
      <img
        ref={imageRef}
        src={masterImage}
        alt="Image with grid overlay"
        width="100%"
      />
      <div
        className="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 pointer-events-none"
        style={gridStyle}
      >
        {Array.from({ length: gridWidth * gridHeight }).map((_, index) => (
          <div key={index} className="border border-white bg-non" />
        ))}
      </div>
    </div>
  );
};

export default MosaicGrid;
