import React from "react";
import { SettingContainer } from "./SettingContainer";

interface ToggleSwitchProps {
  checked: boolean;
  onChange: (checked: boolean) => void;
  disabled?: boolean;
  isUpdating?: boolean;
  label?: string;
  description?: string;
  descriptionMode?: "inline" | "tooltip";
  grouped?: boolean;
  tooltipPosition?: "top" | "bottom";
}

export const ToggleSwitch: React.FC<ToggleSwitchProps> = ({
  checked,
  onChange,
  disabled = false,
  isUpdating = false,
  label,
  description,
  descriptionMode = "tooltip",
  grouped = false,
  tooltipPosition = "top",
}) => {
  // If label and description are provided, use SettingContainer wrapper
  if (label && description) {
    return (
      <SettingContainer
        title={label}
        description={description}
        descriptionMode={descriptionMode}
        grouped={grouped}
        disabled={disabled}
        tooltipPosition={tooltipPosition}
      >
        <label
          className={`relative inline-flex items-center cursor-pointer ${disabled || isUpdating ? "cursor-not-allowed" : ""}`}
        >
          <input
            type="checkbox"
            value=""
            className="sr-only peer"
            checked={checked}
            disabled={disabled || isUpdating}
            onChange={(e) => onChange(e.target.checked)}
          />
          <div className="w-11 h-6 bg-surface-container-highest peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:start-[2px] after:bg-outline after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-primary-container peer-checked:after:bg-on-primary peer-disabled:opacity-50"></div>
        </label>
        {isUpdating && (
          <div className="absolute inset-0 flex items-center justify-center">
            <div className="w-4 h-4 border-2 border-primary border-t-transparent rounded-full animate-spin"></div>
          </div>
        )}
      </SettingContainer>
    );
  }

  // Standalone toggle without wrapper
  return (
    <label
      className={`relative inline-flex items-center cursor-pointer ${disabled || isUpdating ? "cursor-not-allowed" : ""}`}
    >
      <input
        type="checkbox"
        value=""
        className="sr-only peer"
        checked={checked}
        disabled={disabled || isUpdating}
        onChange={(e) => onChange(e.target.checked)}
      />
      <div className="w-11 h-6 bg-surface-container-highest peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:start-[2px] after:bg-outline after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-primary-container peer-checked:after:bg-on-primary peer-disabled:opacity-50"></div>
    </label>
  );
};