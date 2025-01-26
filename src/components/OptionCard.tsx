interface OptionCardProps {
  icon: React.ElementType;
  label: string;
  description: string;
  isSelected: boolean;
  onClick: () => void;
  disabled?: boolean;
  colSpan?: string;
}

const OptionCard: React.FC<OptionCardProps> = ({
  icon: Icon,
  label,
  description,
  isSelected,
  onClick,
  colSpan,
  disabled,
}) => {
  return (
    <div
      className={`p-4 rounded-lg transition-colors ${
        disabled
          ? "bg-gray-100 dark:bg-gray-700"
          : (isSelected
              ? "bg-blue-100 dark:bg-blue-900 border-blue-500"
              : "bg-gray-100 dark:bg-gray-700 dark:border-gray-600 hover:bg-gray-200 dark:hover:bg-gray-600") +
            " cursor-pointer border-2"
      } ${colSpan ? colSpan : "col-span-1"}`}
      onClick={disabled ? undefined : onClick}
    >
      <div className="flex items-center mb-2">
        <Icon
          className={`"w-5 h-5 mr-2 ${disabled ? "text-gray-500" : "text-blue-500"}`}
        />
        <h3
          className={`font-medium ${disabled ? "text-gray-500 dark:text-gray-400" : "text-gray-800 dark:text-gray-200"}`}
        >
          {label}
        </h3>
      </div>
      <p
        className={`text-sm ${disabled ? "text-gray-500" : "text-gray-600 dark:text-gray-400"}`}
      >
        {description}
      </p>
    </div>
  );
};

export default OptionCard;
