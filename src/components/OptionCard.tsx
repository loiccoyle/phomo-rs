interface OptionCardProps {
  icon: React.ElementType;
  label: string;
  description: string;
  isSelected: boolean;
  onClick: () => void;
  colSpan?: string;
}

const OptionCard: React.FC<OptionCardProps> = ({
  icon: Icon,
  label,
  description,
  isSelected,
  onClick,
  colSpan,
}) => {
  return (
    <div
      className={`p-4 rounded-lg cursor-pointer transition-colors ${
        isSelected
          ? "bg-blue-100 dark:bg-blue-900 border-2 border-blue-500"
          : "bg-gray-100 dark:bg-gray-700 border-2 dark:border-gray-600 hover:bg-gray-200 dark:hover:bg-gray-600"
      } ${colSpan ? colSpan : "col-span-1"}`}
      onClick={onClick}
    >
      <div className="flex items-center mb-2">
        <Icon className="w-5 h-5 mr-2 text-blue-500" />
        <h3 className="font-medium text-gray-800 dark:text-gray-200">
          {label}
        </h3>
      </div>
      <p className="text-sm text-gray-600 dark:text-gray-400">{description}</p>
    </div>
  );
};

export default OptionCard;
