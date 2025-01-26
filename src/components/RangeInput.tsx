interface RangeInputProps {
  label: string;
  value: number;
  min: number;
  max: number;
  onChange: (value: number) => void;
  disabled?: boolean;
}

const RangeInput: React.FC<RangeInputProps> = ({
  label,
  value,
  min,
  max,
  onChange,
  disabled = false,
}) => {
  return (
    <div className="flex items-center space-x-4 h-6">
      <label className="text-sm font-medium text-gray-700 dark:text-gray-300 w-1/6">
        {label}
      </label>
      <input
        type="range"
        min={min}
        max={max}
        value={value}
        onChange={(e) => onChange(parseInt(e.target.value))}
        className={`w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer dark:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-opacity-75 ${
          disabled ? "opacity-50" : ""
        }`}
        style={{ accentColor: "#3b82f6" }}
        disabled={disabled}
      />
      <input
        type="number"
        min={min}
        value={value}
        onChange={(e) => onChange(Math.max(parseInt(e.target.value), min))}
        className={`w-12 text-center rounded-md text-gray-700 bg-gray-200 dark:bg-gray-700 dark:text-gray-300 ${
          disabled ? "opacity-50" : ""
        }`}
        disabled={disabled}
      />
    </div>
  );
};

export default RangeInput;
