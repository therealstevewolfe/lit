import React from "react";
import { useSettings } from "../../hooks/useSettings";
import { GlobalShortcutInput } from "./GlobalShortcutInput";
import { LitKeysShortcutInput } from "./LitKeysShortcutInput";

interface ShortcutInputProps {
  descriptionMode?: "inline" | "tooltip";
  grouped?: boolean;
  shortcutId: string;
  disabled?: boolean;
}

/**
 * Wrapper component that selects the appropriate shortcut input implementation
 * based on the keyboard_implementation setting.
 *
 * - "tauri" (default): Uses GlobalShortcutInput with JS keyboard events
 * - "lit_keys": Uses LitKeysShortcutInput with backend key events
 */
export const ShortcutInput: React.FC<ShortcutInputProps> = (props) => {
  const { getSetting } = useSettings();
  const keyboardImplementation = getSetting("keyboard_implementation");

  // Default to Tauri implementation if not set
  if (keyboardImplementation === "lit_keys") {
    return <LitKeysShortcutInput {...props} />;
  }

  return <GlobalShortcutInput {...props} />;
};
