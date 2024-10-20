import React from 'react'
import { X } from 'lucide-react'

interface TileModalProps {
  imageUrl: string
  tileIndex: number
  onClose: () => void
}

const TileModal: React.FC<TileModalProps> = ({ imageUrl, tileIndex, onClose }) => {
  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div className="bg-white dark:bg-gray-800 rounded-lg p-4 max-w-2xl w-full mx-4">
        <div className="flex justify-between items-center mb-4">
          <h3 className="text-xl font-semibold text-gray-800 dark:text-gray-200">Tile Preview (Index: {tileIndex})</h3>
          <button
            onClick={onClose}
            className="text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-200 transition-colors"
          >
            <X size={24} />
          </button>
        </div>
        <img
          src={imageUrl}
          alt={`Full size tile ${tileIndex}`}
          className="w-full h-auto rounded-lg"
        />
      </div>
    </div>
  )
}

export default TileModal