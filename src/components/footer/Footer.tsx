import React, { useState, useEffect } from "react";
import { getVersion } from "@tauri-apps/api/app";

import ModelSelector from "../model-selector";
import UpdateChecker from "../update-checker";

const Footer: React.FC = () => {
  const [version, setVersion] = useState("");

  useEffect(() => {
    const fetchVersion = async () => {
      try {
        const appVersion = await getVersion();
        setVersion(appVersion);
      } catch (error) {
        console.error("Failed to get app version:", error);
        setVersion("1.0.4");
      }
    };

    fetchVersion();
  }, []);

  return (
    <footer className="fixed bottom-0 left-64 right-0 h-8 bg-zinc-950 border-t border-white/5 flex items-center justify-between px-6 z-50">
      {/* Left side - System status */}
      <div className="flex items-center gap-4">
        <span className="font-mono text-[10px] uppercase text-zinc-500 tracking-[0.05em]">System Ready</span>
        <div className="w-2 h-2 rounded-full bg-emerald-500/80 animate-pulse"></div>
      </div>

      {/* Right side - Links and version */}
      <div className="flex items-center gap-6">
        <a 
          href="#" 
          className="font-mono text-[10px] uppercase text-zinc-500 hover:text-zinc-300 transition-colors tracking-[0.05em]"
        >
          Documentation
        </a>
        <a 
          href="#" 
          className="font-mono text-[10px] uppercase text-zinc-500 hover:text-zinc-300 transition-colors tracking-[0.05em]"
        >
          API Status
        </a>
        <span className="font-mono text-[10px] uppercase text-zinc-800 tracking-[0.05em]">
          Lit // v{version}
        </span>
      </div>
    </footer>
  );
};

export default Footer;