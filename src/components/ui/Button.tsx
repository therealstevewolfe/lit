import React from "react";

interface ButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  variant?:
    | "primary"
    | "primary-soft"
    | "secondary"
    | "danger"
    | "danger-ghost"
    | "ghost";
  size?: "sm" | "md" | "lg";
}

export const Button: React.FC<ButtonProps> = ({
  children,
  className = "",
  variant = "primary",
  size = "md",
  ...props
}) => {
  const baseClasses =
    "font-bold rounded-lg border focus:outline-none transition-all duration-200 disabled:opacity-50 disabled:cursor-not-allowed cursor-pointer active:scale-[0.98]";

  const variantClasses = {
    primary:
      "bg-primary text-on-primary hover:scale-95 shadow-lg shadow-primary/20",
    "primary-soft":
      "text-primary bg-primary/10 border-transparent hover:bg-primary/20",
    secondary:
      "bg-surface-container-highest text-on-surface border-white/5 hover:bg-surface-container-high",
    danger:
      "text-white bg-error border-transparent hover:bg-error/90",
    "danger-ghost":
      "text-error border-transparent hover:bg-error-container/50",
    ghost:
      "text-on-surface border-transparent hover:bg-surface-container-low",
  };

  const sizeClasses = {
    sm: "px-3 py-1.5 text-xs",
    md: "px-4 py-2 text-sm",
    lg: "px-6 py-3 text-base",
  };

  return (
    <button
      className={`${baseClasses} ${variantClasses[variant]} ${sizeClasses[size]} ${className}`}
      {...props}
    >
      {children}
    </button>
  );
};