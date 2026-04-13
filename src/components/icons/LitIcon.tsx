import React from "react";

const LitIcon = ({
  width,
  height,
  className,
}: {
  width?: number;
  height?: number;
  className?: string;
}) => {
  return (
    <svg
      width={width}
      height={height}
      className={className}
      viewBox="0 0 32 32"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
    >
      <path
        d="M8 4L16 28L24 4"
        stroke="currentColor"
        strokeWidth="3"
        strokeLinecap="round"
        strokeLinejoin="round"
        className="logo-primary"
      />
      <path
        d="M8 4V28"
        stroke="currentColor"
        strokeWidth="3"
        strokeLinecap="round"
        className="logo-primary"
      />
    </svg>
  );
};

export default LitIcon;
