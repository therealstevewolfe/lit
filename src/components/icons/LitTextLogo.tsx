import React from "react";

const LitTextLogo = ({
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
      viewBox="0 0 120 32"
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
        d="M32 4V28"
        stroke="currentColor"
        strokeWidth="3"
        strokeLinecap="round"
        className="logo-primary"
      />
      <path
        d="M40 16H52C56.4183 16 60 19.5817 60 24C60 28.4183 56.4183 32 52 32H40"
        stroke="currentColor"
        strokeWidth="3"
        strokeLinecap="round"
        strokeLinejoin="round"
        className="logo-primary"
      />
      <text
        x="70"
        y="24"
        fontFamily="Inter, system-ui, sans-serif"
        fontSize="20"
        fontWeight="600"
        fill="currentColor"
        className="logo-stroke"
      >
        Lit
      </text>
    </svg>
  );
};

export default LitTextLogo;
