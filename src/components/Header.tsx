import React from "react";
import { Sun, Moon } from "lucide-react";
import { useTheme } from "../contexts/ThemeContext";

const Header: React.FC = () => {
  const { theme, toggleTheme } = useTheme();

  return (
    <header className="mb-4 text-center">
      <div className="flex justify-between mb-2">
        <h1 className="text-2xl sm:text-3xl font-bold text-gray-800 dark:text-gray-100 mb-0">
          Phomo: easy photo mosaics
        </h1>
        <div className="flex justify-end mb-2">
          <button
            onClick={toggleTheme}
            className="p-2 rounded-full bg-gray-200 dark:bg-gray-700 text-gray-800 dark:text-gray-200"
            aria-label="Toggle dark mode"
          >
            {theme === "light" ? <Moon size={20} /> : <Sun size={20} />}
          </button>
        </div>
      </div>
      {/* <p className="text-gray-600 dark:text-gray-300">Create beautiful photo mosaics with ease</p> */}
    </header>
  );
};

export default Header;
