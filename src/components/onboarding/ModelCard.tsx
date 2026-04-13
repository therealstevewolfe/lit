import React from "react";
import { useTranslation } from "react-i18next";
import {
  Check,
  Download,
  Globe,
  Languages,
  Loader2,
  Trash2,
} from "lucide-react";
import type { ModelInfo } from "@/bindings";
import { formatModelSize } from "../../lib/utils/format";
import {
  getTranslatedModelDescription,
  getTranslatedModelName,
} from "../../lib/utils/modelTranslation";
import { LANGUAGES } from "../../lib/constants/languages";
import Badge from "../ui/Badge";
import { Button } from "../ui/Button";

// Get display text for model's language support
const getLanguageDisplayText = (
  supportedLanguages: string[],
  t: (key: string, options?: Record<string, unknown>) => string,
): string => {
  if (supportedLanguages.length === 1) {
    const langCode = supportedLanguages[0];
    const langName =
      LANGUAGES.find((l) => l.value === langCode)?.label || langCode;
    return t("modelSelector.capabilities.languageOnly", { language: langName });
  }
  return t("modelSelector.capabilities.multiLanguage");
};

export type ModelCardStatus =
  | "downloadable"
  | "downloading"
  | "verifying"
  | "extracting"
  | "switching"
  | "active"
  | "available";

interface ModelCardProps {
  model: ModelInfo;
  variant?: "default" | "featured";
  status?: ModelCardStatus;
  disabled?: boolean;
  className?: string;
  onSelect: (modelId: string) => void;
  onDownload?: (modelId: string) => void;
  onDelete?: (modelId: string) => void;
  onCancel?: (modelId: string) => void;
  downloadProgress?: number;
  downloadSpeed?: number; // MB/s
  showRecommended?: boolean;
}

const ModelCard: React.FC<ModelCardProps> = ({
  model,
  variant = "default",
  status = "downloadable",
  disabled = false,
  className = "",
  onSelect,
  onDownload,
  onDelete,
  onCancel,
  downloadProgress,
  downloadSpeed,
  showRecommended = true,
}) => {
  const { t } = useTranslation();
  const isFeatured = variant === "featured";
  const isClickable =
    status === "available" || status === "active" || status === "downloadable";

  // Get translated model name and description
  const displayName = getTranslatedModelName(model, t);
  const displayDescription = getTranslatedModelDescription(model, t);

  const baseClasses =
    "relative flex flex-col rounded-2xl overflow-hidden transition-all duration-300 text-left";

  const getVariantClasses = () => {
    if (status === "active") {
      return "bg-surface-container-high border-2 border-primary active-glow";
    }
    if (isFeatured) {
      return "bg-surface-container-low border-l-2 border-primary hover:bg-surface-container-high";
    }
    return "bg-surface-container-low hover:bg-surface-container-high";
  };

  const getInteractiveClasses = () => {
    if (!isClickable) return "";
    if (disabled) return "opacity-60 cursor-not-allowed";
    return "cursor-pointer hover:scale-[1.01] active:scale-[0.98]";
  };

  const handleClick = () => {
    if (!isClickable || disabled) return;
    if (status === "downloadable" && onDownload) {
      onDownload(model.id);
    } else {
      onSelect(model.id);
    }
  };

  const handleDelete = (e: React.MouseEvent) => {
    e.stopPropagation();
    onDelete?.(model.id);
  };

  return (
    <div
      onClick={handleClick}
      onKeyDown={(e) => {
        if (e.key === "Enter" && isClickable) handleClick();
      }}
      role={isClickable ? "button" : undefined}
      tabIndex={isClickable ? 0 : undefined}
      className={[
        baseClasses,
        getVariantClasses(),
        getInteractiveClasses(),
        className,
      ]
        .filter(Boolean)
        .join(" ")}
    >
      <div className="p-5">
        {/* Top section: name/description + icon */}
        <div className="flex justify-between items-start mb-4">
          <div className="flex-1">
            <div className="flex items-center gap-2 mb-1 flex-wrap">
              <h3 className="text-lg font-bold text-on-surface">
                {displayName}
              </h3>
              {showRecommended && model.is_recommended && (
                <span className="bg-surface-variant text-on-surface-variant text-[10px] px-2 py-0.5 rounded-full font-mono uppercase tracking-tighter">
                  {t("onboarding.recommended")}
                </span>
              )}
              {status === "active" && (
                <span className="bg-primary-container text-on-primary-container text-[10px] px-2 py-0.5 rounded-full font-mono uppercase tracking-tighter">
                  Current
                </span>
              )}
              {model.is_custom && (
                <Badge variant="secondary">{t("modelSelector.custom")}</Badge>
              )}
              {status === "switching" && (
                <Badge variant="secondary">
                  <Loader2 className="w-3 h-3 mr-1 animate-spin" />
                  {t("modelSelector.switching")}
                </Badge>
              )}
            </div>
            <p className="text-xs text-outline leading-relaxed">
              {displayDescription}
            </p>
          </div>
          
          {/* Status icon */}
          {status === "active" && (
            <span className="material-symbols-outlined text-primary text-2xl" style={{ fontVariationSettings: "'FILL' 1" }}>
              check_circle
            </span>
          )}
          {status === "downloadable" && (
            <span className="material-symbols-outlined text-outline text-2xl">cloud_download</span>
          )}
        </div>

        {/* Stats grid */}
        <div className="grid grid-cols-3 gap-2 mb-5">
          <div className="bg-surface-container-lowest p-2 rounded-lg">
            <span className="block font-mono text-[9px] text-outline uppercase tracking-wider mb-1">Size</span>
            <span className="text-xs font-semibold text-on-surface">{formatModelSize(Number(model.size_mb))}</span>
          </div>
          <div className="bg-surface-container-lowest p-2 rounded-lg">
            <span className="block font-mono text-[9px] text-outline uppercase tracking-wider mb-1">Accuracy</span>
            <span className={`text-xs font-semibold ${model.accuracy_score > 0.7 ? 'text-primary' : 'text-tertiary'}`}>
              {model.accuracy_score > 0.7 ? 'High' : model.accuracy_score > 0.4 ? 'Medium' : 'Fast'}
            </span>
          </div>
          <div className="bg-surface-container-lowest p-2 rounded-lg">
            <span className="block font-mono text-[9px] text-outline uppercase tracking-wider mb-1">Latency</span>
            <span className="text-xs font-semibold text-on-surface">
              {model.speed_score > 0.7 ? '~15ms' : model.speed_score > 0.4 ? '~45ms' : '~12ms'}
            </span>
          </div>
        </div>

        {/* Action button */}
        {status === "downloadable" && (
          <button
            onClick={(e) => {
              e.stopPropagation();
              onDownload?.(model.id);
            }}
            disabled={disabled}
            className="w-full h-11 bg-primary text-on-primary font-bold text-sm rounded-xl flex items-center justify-center gap-2 active:scale-[0.98] transition-transform shadow-lg shadow-primary/20"
          >
            <Download className="w-5 h-5" />
            DOWNLOAD
          </button>
        )}
        
        {status === "active" && (
          <button
            disabled
            className="w-full h-11 bg-surface-container-highest text-outline font-bold text-sm rounded-xl flex items-center justify-center gap-2 cursor-not-allowed"
          >
            <span className="material-symbols-outlined text-lg">settings</span>
            REINSTALL
          </button>
        )}

        {status === "available" && !model.is_downloaded && (
          <button
            onClick={(e) => {
              e.stopPropagation();
              onDownload?.(model.id);
            }}
            disabled={disabled}
            className="w-full h-11 bg-primary text-on-primary font-bold text-sm rounded-xl flex items-center justify-center gap-2 active:scale-[0.98] transition-transform"
          >
            <Download className="w-5 h-5" />
            DOWNLOAD
          </button>
        )}

        {onDelete && (status === "available" || status === "active") && (
          <Button
            variant="ghost"
            size="sm"
            onClick={handleDelete}
            title={t("modelSelector.deleteModel", { modelName: displayName })}
            className="flex items-center gap-1.5 ms-auto text-primary/85 hover:text-primary hover:bg-primary/10 mt-2"
          >
            <Trash2 className="w-4 h-4" />
            <span>{t("common.delete")}</span>
          </Button>
        )}
      </div>

      {/* Download/extract progress */}
      {status === "downloading" && downloadProgress !== undefined && (
        <div className="w-full px-5 pb-5">
          <div className="w-full h-1 bg-surface-container-lowest rounded-full overflow-hidden">
            <div
              className="h-full bg-primary rounded-full transition-all duration-300"
              style={{ width: `${downloadProgress}%` }}
            />
          </div>
          <div className="flex items-center justify-between text-xs mt-2">
            <span className="text-outline">
              {t("modelSelector.downloading", {
                percentage: Math.round(downloadProgress),
              })}
            </span>
            <div className="flex items-center gap-2">
              {downloadSpeed !== undefined && downloadSpeed > 0 && (
                <span className="tabular-nums text-outline">
                  {t("modelSelector.downloadSpeed", {
                    speed: downloadSpeed.toFixed(1),
                  })}
                </span>
              )}
              {onCancel && (
                <Button
                  variant="danger-ghost"
                  size="sm"
                  onClick={(e) => {
                    e.preventDefault();
                    e.stopPropagation();
                    onCancel(model.id);
                  }}
                  aria-label={t("modelSelector.cancelDownload")}
                >
                  {t("modelSelector.cancel")}
                </Button>
              )}
            </div>
          </div>
        </div>
      )}
      
      {status === "verifying" && (
        <div className="w-full px-5 pb-5">
          <div className="w-full h-1 bg-surface-container-lowest rounded-full overflow-hidden">
            <div className="h-full bg-primary rounded-full animate-pulse w-full" />
          </div>
          <p className="text-xs text-outline mt-2">
            {t("modelSelector.verifyingGeneric")}
          </p>
        </div>
      )}
      
      {status === "extracting" && (
        <div className="w-full px-5 pb-5">
          <div className="w-full h-1 bg-surface-container-lowest rounded-full overflow-hidden">
            <div className="h-full bg-primary rounded-full animate-pulse w-full" />
          </div>
          <p className="text-xs text-outline mt-2">
            {t("modelSelector.extractingGeneric")}
          </p>
        </div>
      )}
    </div>
  );
};

export default ModelCard;