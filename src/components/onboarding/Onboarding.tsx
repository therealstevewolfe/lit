import React, { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { toast } from "sonner";
import type { ModelInfo } from "@/bindings";
import type { ModelCardStatus } from "./ModelCard";
import ModelCard from "./ModelCard";
import { useModelStore } from "../../stores/modelStore";

interface OnboardingProps {
  onModelSelected: () => void;
}

const Onboarding: React.FC<OnboardingProps> = ({ onModelSelected }) => {
  const { t } = useTranslation();
  const {
    models,
    downloadModel,
    selectModel,
    downloadingModels,
    verifyingModels,
    extractingModels,
    downloadProgress,
    downloadStats,
  } = useModelStore();
  const [selectedModelId, setSelectedModelId] = useState<string | null>(null);

  const isDownloading = selectedModelId !== null;

  // Watch for the selected model to finish downloading + verifying + extracting
  useEffect(() => {
    if (!selectedModelId) return;

    const model = models.find((m) => m.id === selectedModelId);
    const stillDownloading = selectedModelId in downloadingModels;
    const stillVerifying = selectedModelId in verifyingModels;
    const stillExtracting = selectedModelId in extractingModels;

    if (
      model?.is_downloaded &&
      !stillDownloading &&
      !stillVerifying &&
      !stillExtracting
    ) {
      // Model is ready — select it and transition
      selectModel(selectedModelId).then((success) => {
        if (success) {
          onModelSelected();
        } else {
          toast.error(t("onboarding.errors.selectModel"));
          setSelectedModelId(null);
        }
      });
    }
  }, [
    selectedModelId,
    models,
    downloadingModels,
    verifyingModels,
    extractingModels,
    selectModel,
    onModelSelected,
  ]);

  const handleDownloadModel = async (modelId: string) => {
    setSelectedModelId(modelId);

    // Error toast is handled centrally by the model-download-failed event listener
    // in modelStore — no toast here to avoid duplicates.
    const success = await downloadModel(modelId);
    if (!success) {
      setSelectedModelId(null);
    }
  };

  const getModelStatus = (modelId: string): ModelCardStatus => {
    if (modelId in extractingModels) return "extracting";
    if (modelId in verifyingModels) return "verifying";
    if (modelId in downloadingModels) return "downloading";
    return "downloadable";
  };

  const getModelDownloadProgress = (modelId: string): number | undefined => {
    return downloadProgress[modelId]?.percentage;
  };

  const getModelDownloadSpeed = (modelId: string): number | undefined => {
    return downloadStats[modelId]?.speed;
  };

  return (
    <div className="relative flex flex-col items-center justify-between min-h-screen bg-background overflow-hidden px-8 py-16">
      {/* Background glow orbs */}
      <div className="absolute inset-0 z-0 pointer-events-none">
        <div className="absolute top-[-10%] left-[-20%] w-[140%] h-[60%] bg-gradient-to-b from-primary-container/10 via-transparent to-transparent opacity-40 blur-[100px]"></div>
        <div className="absolute bottom-0 left-0 w-full h-[30%] bg-gradient-to-t from-background via-background to-transparent z-10"></div>
      </div>

      {/* Logo section */}
      <div className="relative z-20 flex flex-col items-center w-full mt-12">
        <div className="w-24 h-24 mb-8 bg-surface-container-high rounded-full flex items-center justify-center neon-glow border border-white/5">
          <span className="text-primary font-headline text-5xl font-extrabold tracking-tighter">Lit</span>
        </div>
        <div className="text-center space-y-4 max-w-xs">
          <h1 className="text-4xl font-extrabold tracking-tight text-on-surface leading-tight">
            Speak. <span className="text-primary">Transcribe.</span> Done.
          </h1>
          <p className="text-outline font-body text-base leading-relaxed opacity-80">
            {t("onboarding.subtitle")}
          </p>
        </div>
      </div>

      {/* Model cards */}
      <div className="relative z-20 w-full flex flex-col items-center gap-8 mb-12">
        <div className="w-full space-y-6 max-w-md">
          {models
            .filter((m: ModelInfo) => !m.is_downloaded)
            .filter((model: ModelInfo) => model.is_recommended)
            .map((model: ModelInfo) => (
              <ModelCard
                key={model.id}
                model={model}
                variant="featured"
                status={getModelStatus(model.id)}
                disabled={isDownloading}
                onSelect={handleDownloadModel}
                onDownload={handleDownloadModel}
                downloadProgress={getModelDownloadProgress(model.id)}
                downloadSpeed={getModelDownloadSpeed(model.id)}
              />
            ))}

          {models
            .filter((m: ModelInfo) => !m.is_downloaded)
            .filter((model: ModelInfo) => !model.is_recommended)
            .sort(
              (a: ModelInfo, b: ModelInfo) =>
                Number(a.size_mb) - Number(b.size_mb),
            )
            .map((model: ModelInfo) => (
              <ModelCard
                key={model.id}
                model={model}
                status={getModelStatus(model.id)}
                disabled={isDownloading}
                onSelect={handleDownloadModel}
                onDownload={handleDownloadModel}
                downloadProgress={getModelDownloadProgress(model.id)}
                downloadSpeed={getModelDownloadSpeed(model.id)}
              />
            ))}
        </div>
      </div>

      {/* Version indicator */}
      <div className="absolute top-[25%] left-[50%] -translate-x-1/2 w-full max-w-md aspect-square pointer-events-none opacity-20">
        <div className="w-full h-full rounded-full border border-primary/20 scale-150 blur-sm"></div>
        <div className="absolute inset-0 w-full h-full rounded-full border border-primary/10 scale-125"></div>
        <div className="absolute inset-0 w-full h-full rounded-full border border-primary/5 scale-110"></div>
      </div>
      
      <div className="relative z-20 pb-4">
        <p className="font-mono text-[11px] text-outline-variant flex items-center gap-2">
          <span className="w-1.5 h-1.5 bg-primary rounded-full animate-pulse"></span>
          v1.0.4 Ready for session
        </p>
      </div>
    </div>
  );
};

export default Onboarding;