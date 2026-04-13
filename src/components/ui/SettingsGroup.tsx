import React from "react";

interface SettingsGroupProps {
  title?: string;
  description?: string;
  children: React.ReactNode;
}

export const SettingsGroup: React.FC<SettingsGroupProps> = ({
  title,
  description,
  children,
}) => {
  return (
    <div className="space-y-3">
      {title && (
        <div className="px-2">
          <h2 className="font-mono text-[10px] uppercase tracking-[0.1em] text-primary mb-1">
            {title}
          </h2>
          {description && (
            <p className="text-on-surface text-lg font-medium">{description}</p>
          )}
        </div>
      )}
      <div className="bg-surface-container-low rounded-xl overflow-visible divide-y divide-white/5">
        {React.Children.map(children, (child, index) => (
          <div className={`px-6 py-4 ${index > 0 ? 'border-t border-white/5' : ''}`}>
            {child}
          </div>
        ))}
      </div>
    </div>
  );
};