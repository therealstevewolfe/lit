import React from "react";
import { useTranslation } from "react-i18next";
import { Settings, Layers, Terminal, History, Sparkles, FlaskConical, Info, Cpu, LayoutGrid, HelpCircle } from "lucide-react";
import LitTextLogo from "./icons/LitTextLogo";
import { useSettings } from "../hooks/useSettings";
import {
  GeneralSettings,
  AdvancedSettings,
  HistorySettings,
  DebugSettings,
  AboutSettings,
  PostProcessingSettings,
  ModelsSettings,
} from "./settings";

export type SidebarSection = keyof typeof SECTIONS_CONFIG;

interface IconProps {
  width?: number | string;
  height?: number | string;
  size?: number | string;
  className?: string;
  [key: string]: any;
}

interface SectionConfig {
  labelKey: string;
  icon: React.ComponentType<IconProps>;
  component: React.ComponentType;
  enabled: (settings: any) => boolean;
}

export const SECTIONS_CONFIG = {
  general: {
    labelKey: "sidebar.general",
    icon: Settings,
    component: GeneralSettings,
    enabled: () => true,
  },
  models: {
    labelKey: "sidebar.models",
    icon: Cpu,
    component: ModelsSettings,
    enabled: () => true,
  },
  advanced: {
    labelKey: "sidebar.advanced",
    icon: Terminal,
    component: AdvancedSettings,
    enabled: () => true,
  },
  history: {
    labelKey: "sidebar.history",
    icon: History,
    component: HistorySettings,
    enabled: () => true,
  },
  postprocessing: {
    labelKey: "sidebar.postProcessing",
    icon: Sparkles,
    component: PostProcessingSettings,
    enabled: (settings) => settings?.post_process_enabled ?? false,
  },
  debug: {
    labelKey: "sidebar.debug",
    icon: FlaskConical,
    component: DebugSettings,
    enabled: (settings) => settings?.debug_mode ?? false,
  },
  about: {
    labelKey: "sidebar.about",
    icon: Info,
    component: AboutSettings,
    enabled: () => true,
  },
} as const satisfies Record<string, SectionConfig>;

interface SidebarProps {
  activeSection: SidebarSection;
  onSectionChange: (section: SidebarSection) => void;
}

export const Sidebar: React.FC<SidebarProps> = ({
  activeSection,
  onSectionChange,
}) => {
  const { t } = useTranslation();
  const { settings } = useSettings();

  const availableSections = Object.entries(SECTIONS_CONFIG)
    .filter(([_, config]) => config.enabled(settings))
    .map(([id, config]) => ({ id: id as SidebarSection, ...config }));

  return (
    <aside className="fixed left-0 top-12 bottom-0 w-64 bg-zinc-950 flex flex-col py-6 border-r border-white/10">
      {/* Logo Header */}
      <div className="px-6 mb-8">
        <div className="flex items-center gap-3">
          <div className="w-8 h-8 rounded bg-surface-container-high flex items-center justify-center">
            <span className="material-symbols-outlined text-on-primary-container text-sm">auto_awesome</span>
          </div>
          <div>
            <h2 className="text-xl font-black text-zinc-100 leading-none tracking-tighter">Lit</h2>
            <p className="font-mono text-[11px] uppercase tracking-[0.05em] text-zinc-500 mt-0.5">V1.0.4</p>
          </div>
        </div>
      </div>

      {/* Navigation */}
      <nav className="flex-1 space-y-1 px-3">
        {availableSections.map((section) => {
          const Icon = section.icon;
          const isActive = activeSection === section.id;

          return (
            <div
              key={section.id}
              onClick={() => onSectionChange(section.id)}
              className={`flex items-center gap-3 px-3 py-2.5 rounded-lg cursor-pointer transition-all duration-200 ease-in-out ${
                isActive
                  ? "text-violet-300 border-l-2 border-violet-300 bg-zinc-900/50"
                  : "text-zinc-500 hover:text-zinc-300 hover:bg-zinc-900/30"
              }`}
            >
              <Icon width={18} height={18} className="shrink-0" />
              <span className="font-mono text-[11px] uppercase tracking-[0.05em]">
                {t(section.labelKey)}
              </span>
            </div>
          );
        })}
      </nav>

      {/* Bottom Actions */}
      <div className="px-6 mt-auto">
        <button className="w-full py-2.5 px-4 bg-primary text-on-primary font-bold text-xs uppercase tracking-widest rounded-md hover:scale-95 transition-all duration-200 flex items-center justify-center gap-2">
          <span className="material-symbols-outlined text-sm">add</span>
          <span>New Project</span>
        </button>
        
        <div className="mt-6 border-t border-zinc-900 pt-6">
          <div className="flex items-center gap-3 py-2 text-zinc-500 hover:text-zinc-300 transition-colors cursor-pointer">
            <HelpCircle width={18} height={18} className="shrink-0" />
            <span className="font-mono text-[11px] uppercase tracking-[0.05em]">Support</span>
          </div>
        </div>
      </div>
    </aside>
  );
};