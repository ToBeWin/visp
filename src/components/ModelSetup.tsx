import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { AnimatePresence, motion } from 'framer-motion';

interface ModelStatus {
  asr_available: boolean;
  translation_available: boolean;
  missing_files: string[];
  models_dir: string;
  asr_summary: string;
  asr_detail: string;
  translation_summary: string;
  translation_detail: string;
  download_supported: boolean;
}

interface DownloadProgressPayload {
  model_name: string;
  progress: number;
  status: string;
}

export function ModelSetup() {
  const [status, setStatus] = useState<ModelStatus | null>(null);
  const [loading, setLoading] = useState(true);
  const [downloading, setDownloading] = useState(false);
  const [progress, setProgress] = useState<DownloadProgressPayload | null>(null);

  useEffect(() => {
    const unlistenProgress = listen<DownloadProgressPayload>('download-progress', (event) => {
      setProgress(event.payload);
    });

    const unlistenComplete = listen('download-complete', () => {
      setDownloading(false);
      setProgress(null);
      void checkModels();
    });

    void checkModels();

    return () => {
      unlistenProgress.then((fn) => fn());
      unlistenComplete.then((fn) => fn());
    };
  }, []);

  const checkModels = async () => {
    try {
      setLoading(true);
      const result = await invoke<ModelStatus>('check_models');
      setStatus(result);
    } catch (error) {
      console.error('Failed to check models:', error);
      setStatus(null);
    } finally {
      setLoading(false);
    }
  };

  const downloadModels = async () => {
    try {
      setDownloading(true);
      await invoke('download_models');
    } catch (error) {
      console.error('Failed to download models:', error);
      setDownloading(false);
    }
  };

  if (loading) {
    return (
      <div className="fixed inset-0 z-40 flex items-center justify-center bg-black/[0.45] backdrop-blur-sm">
        <div className="rounded-2xl border border-white/10 bg-white/10 px-6 py-5 text-white shadow-2xl">
          正在检查模型文件...
        </div>
      </div>
    );
  }

  if (!status || (status.asr_available && status.translation_available)) {
    return null;
  }

  return (
    <AnimatePresence>
      <motion.div
        initial={{ opacity: 0 }}
        animate={{ opacity: 1 }}
        exit={{ opacity: 0 }}
        className="fixed inset-0 z-40 flex items-center justify-center bg-slate-950/[0.65] p-4 backdrop-blur-md"
      >
        <motion.div
          initial={{ scale: 0.96, y: 18 }}
          animate={{ scale: 1, y: 0 }}
          exit={{ scale: 0.96, y: 18 }}
          className="w-full max-w-2xl rounded-[28px] border border-white/[0.12] bg-white/10 p-8 text-white shadow-[0_24px_80px_rgba(0,0,0,0.35)] backdrop-blur-2xl"
        >
          <div className="mb-6">
            <h2 className="text-2xl font-semibold">需要完成运行准备</h2>
            <p className="mt-2 text-sm text-white/70">
              Visp 会在主界面检查可用后端、系统能力和本地资源。当前版本优先使用 macOS 系统语音识别和本机 Ollama，而不是单纯依赖模型文件。
            </p>
          </div>

          <div className="space-y-3">
            <ModelRow
              name="语音识别"
              available={status.asr_available}
              summary={status.asr_summary}
              detail={status.asr_detail}
            />
            <ModelRow
              name="文本翻译"
              available={status.translation_available}
              summary={status.translation_summary}
              detail={status.translation_detail}
            />
          </div>

          <div className="mt-6 rounded-2xl border border-white/10 bg-black/20 p-4 text-sm text-white/75">
            <div className="font-medium text-white">推荐准备方式</div>
            <div className="mt-2 space-y-2 leading-6">
              <p>1. macOS 语音识别默认使用系统能力，不需要下载 Whisper 模型。</p>
              <p>2. 文本翻译请先安装并启动 Ollama。</p>
              <p>3. 推荐执行 <code className="rounded bg-black/25 px-2 py-1">ollama pull translategemma:latest</code></p>
              <p className="text-white/55">
                应用数据目录仍保留在 <code className="rounded bg-black/25 px-2 py-1">{status.models_dir}</code>，后续接入应用内 GGUF 后端时会继续使用。
              </p>
            </div>
          </div>

          <div className="mt-6 flex flex-wrap gap-3">
            {status.download_supported ? (
              <button
                onClick={downloadModels}
                disabled={downloading}
                className="rounded-xl bg-white px-4 py-2 font-medium text-slate-900 transition disabled:cursor-not-allowed disabled:opacity-60"
              >
                {downloading ? '下载中...' : '一键下载模型'}
              </button>
            ) : null}
            <button
              onClick={checkModels}
              disabled={downloading}
              className="rounded-xl border border-white/[0.15] bg-white/5 px-4 py-2 font-medium text-white transition disabled:cursor-not-allowed disabled:opacity-60"
            >
              重新检查
            </button>
          </div>

          {progress ? (
            <div className="mt-6 rounded-2xl border border-white/10 bg-black/20 p-4">
              <div className="flex items-center justify-between gap-4 text-sm">
                <span className="font-medium">{progress.model_name}</span>
                <span className="text-white/70">{progress.status}</span>
              </div>
              <div className="mt-3 h-2 overflow-hidden rounded-full bg-white/10">
                <div
                  className="h-full rounded-full bg-gradient-to-r from-sky-400 to-violet-500 transition-all"
                  style={{ width: `${progress.progress}%` }}
                />
              </div>
            </div>
          ) : null}
        </motion.div>
      </motion.div>
    </AnimatePresence>
  );
}

function ModelRow({
  name,
  available,
  summary,
  detail,
}: {
  name: string;
  available: boolean;
  summary: string;
  detail: string;
}) {
  return (
    <div className="rounded-2xl border border-white/10 bg-white/5 px-4 py-4">
      <div className="flex items-center justify-between gap-4">
        <span className="text-sm font-medium">{name}</span>
        <span className={available ? 'text-emerald-300' : 'text-amber-300'}>
          {summary}
        </span>
      </div>
      <p className="mt-2 text-sm leading-6 text-white/65">{detail}</p>
    </div>
  );
}
