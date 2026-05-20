import { useEffect, useRef, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { Glow } from './components/Glow';
import { ModelSetup } from './components/ModelSetup';
import { Settings } from './components/Settings';
import { Welcome } from './components/Welcome';
import { HistoryPanel } from './components/HistoryPanel';

type BannerTone = 'neutral' | 'info' | 'success' | 'warning' | 'error';

interface BannerState {
  title: string;
  detail: string;
  tone: BannerTone;
}

interface RuntimeReadiness {
  voice_ready: boolean;
  text_ready: boolean;
  microphone_permission: boolean;
  speech_permission: boolean;
  translation_available: boolean;
  translation_summary: string;
  translation_detail: string;
  recommended_actions: string[];
}

type BannerAction =
  | { kind: 'permission'; scope: Exclude<PermissionScope, null>; label: string }
  | { kind: 'settings'; label: string }
  | null;

const isMac = /Mac|iPhone|iPad|iPod/.test(navigator.platform);
const voiceShortcutLabel = isMac ? 'Command + Shift + V' : 'Ctrl + Shift + V';
const textShortcutLabel = isMac ? 'Command + Shift + C' : 'Ctrl + Shift + C';

const defaultBanner: BannerState = {
  title: 'Visp 已准备就绪',
  detail: '',
  tone: 'neutral',
};

function App() {
  const [welcomeChecked, setWelcomeChecked] = useState(false);
  const [showWelcome, setShowWelcome] = useState(false);
  const [settingsVisible, setSettingsVisible] = useState(false);
  const [showHistory, setShowHistory] = useState(false);
  const [showGlow, setShowGlow] = useState(false);
  const [translationText, setTranslationText] = useState('');
  const [banner, setBanner] = useState<BannerState>(defaultBanner);
  const [bannerAction, setBannerAction] = useState<BannerAction>(null);
  const [busyAction, setBusyAction] = useState<'voice' | 'text' | null>(null);
  const [readiness, setReadiness] = useState<RuntimeReadiness | null>(null);
  const voiceInteraction = useRef<'idle' | 'recording' | 'thinking'>('idle');

  const guards = useRef({
    voiceStart: false,
    voiceStop: false,
    textTrigger: false,
  });

  useEffect(() => {
    const hasSeenWelcome = localStorage.getItem('visp_welcome_completed');
    setShowWelcome(!hasSeenWelcome);
    setWelcomeChecked(true);
    void refreshRuntimeReadiness();

    const unlistenVoiceToggle = listen('voice-mode-toggle', () => {
      if (voiceInteraction.current === 'recording') {
        void handleVoiceStop();
        return;
      }

      if (voiceInteraction.current === 'thinking') {
        return;
      }

      void handleVoiceStart();
    });

    const unlistenTextTrigger = listen('text-mode-trigger', () => {
      if (guards.current.textTrigger) {
        guards.current.textTrigger = false;
        return;
      }

      void handleTextTrigger();
    });

    const unlistenPreview = listen<string>('text-mode-preview', (event) => {
      if (guards.current.textTrigger) {
        guards.current.textTrigger = false;
        return;
      }

      setTranslationText(event.payload);
      setShowGlow(true);
      setBanner({
        title: '译文已生成',
        detail: '按 Enter 替换当前内容，Esc 取消。',
        tone: 'success',
      });
    });

    const unlistenVoiceComplete = listen('voice-mode-complete', () => {
      guards.current.voiceStart = false;
      guards.current.voiceStop = false;
      voiceInteraction.current = 'idle';
      setBusyAction(null);
      setBanner({
        title: '语音翻译完成',
        detail: '结果已输入到当前光标位置。',
        tone: 'success',
      });
    });

    const unlistenError = listen<string>('error', (event) => {
      const detail = formatError(event.payload, readiness);
      setBannerAction(resolveBannerAction(event.payload, readiness));
      guards.current.voiceStart = false;
      guards.current.voiceStop = false;
      guards.current.textTrigger = false;
      voiceInteraction.current = 'idle';
      setBusyAction(null);
      setBanner({
        title: '操作失败',
        detail,
        tone: 'error',
      });
    });

    return () => {
      unlistenVoiceToggle.then((fn) => fn());
      unlistenTextTrigger.then((fn) => fn());
      unlistenPreview.then((fn) => fn());
      unlistenVoiceComplete.then((fn) => fn());
      unlistenError.then((fn) => fn());
    };
    // Intentionally omit handleVoiceStart/handleVoiceStop/handleTextTrigger from deps;
    // they use ref-based guards and don't need re-subscription on change.
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [readiness]);

  const refreshRuntimeReadiness = async () => {
    try {
      const next = await invoke<RuntimeReadiness>('get_runtime_readiness');
      setReadiness(next);
    } catch (error) {
      console.error('Failed to load runtime readiness:', error);
    }
  };

  const handleWelcomeComplete = () => {
    localStorage.setItem('visp_welcome_completed', 'true');
    setShowWelcome(false);
    setBannerAction(null);
    setBanner({
      title: '欢迎完成',
      detail: '',
      tone: 'success',
    });
    void refreshRuntimeReadiness();
  };

  const handleConfirmTranslation = async () => {
    try {
      setBusyAction('text');
      setBanner({
        title: '正在替换文本',
        detail: '会先备份剪贴板，再粘贴译文。',
        tone: 'info',
      });
      setBannerAction(null);
      await invoke('confirm_translation', { translationText });
      setShowGlow(false);
      setTranslationText('');
      setBanner({
        title: '文本已替换',
        detail: '译文已经写回当前光标位置。',
        tone: 'success',
      });
      void refreshRuntimeReadiness();
    } catch (error) {
      setBannerAction(resolveBannerAction(error, readiness));
      setBanner({
        title: '替换失败',
        detail: formatError(error, readiness),
        tone: 'error',
      });
    } finally {
      setBusyAction(null);
    }
  };

  const handleCancelTranslation = async () => {
    try {
      setBusyAction('text');
      await invoke('cancel_translation');
    } catch (error) {
      setBannerAction(resolveBannerAction(error, readiness));
      setBanner({
        title: '取消失败',
        detail: formatError(error, readiness),
        tone: 'error',
      });
    } finally {
      setBusyAction(null);
      setShowGlow(false);
      setTranslationText('');
      setBanner({
        title: '译文已取消',
        detail: '不会对当前内容做任何替换。',
        tone: 'neutral',
      });
      setBannerAction(null);
    }
  };

  async function handleVoiceStart() {
    try {
      guards.current.voiceStart = true;
      setBusyAction('voice');
      voiceInteraction.current = 'recording';
      setBanner({
        title: '语音模式已启动',
        detail: '',
        tone: 'info',
      });
      setBannerAction(null);
      await invoke('start_voice_mode');
    } catch (error) {
      setBannerAction(resolveBannerAction(error, readiness));
      guards.current.voiceStart = false;
      voiceInteraction.current = 'idle';
      setBanner({
        title: '语音启动失败',
        detail: formatError(error, readiness),
        tone: 'error',
      });
    } finally {
      window.setTimeout(() => {
        if (guards.current.voiceStart) {
          guards.current.voiceStart = false;
        }
      }, 1000);
      setBusyAction((current) => (current === 'voice' ? null : current));
    }
  }

  async function handleVoiceStop() {
    try {
      guards.current.voiceStop = true;
      setBusyAction('voice');
      voiceInteraction.current = 'thinking';
      setBanner({
        title: '正在处理语音',
        detail: '',
        tone: 'info',
      });
      setBannerAction(null);
      await invoke('stop_voice_mode');
    } catch (error) {
      setBannerAction(resolveBannerAction(error, readiness));
      guards.current.voiceStop = false;
      voiceInteraction.current = 'idle';
      setBanner({
        title: '语音处理失败',
        detail: formatError(error, readiness),
        tone: 'error',
      });
    } finally {
      window.setTimeout(() => {
        if (guards.current.voiceStop) {
          guards.current.voiceStop = false;
        }
      }, 1000);
      setBusyAction((current) => (current === 'voice' ? null : current));
    }
  }

  const handleTextTrigger = async () => {
    try {
      guards.current.textTrigger = true;
      setBusyAction('text');
      setBanner({
        title: '文本翻译处理中',
        detail: '会先读取当前选区，再生成译文预览。',
        tone: 'info',
      });
      setBannerAction(null);
      const result = await invoke<string>('start_text_mode');
      setTranslationText(result);
      setShowGlow(true);
      setBanner({
        title: '译文已生成',
        detail: '按 Enter 替换当前内容，Esc 取消。',
        tone: 'success',
      });
      void refreshRuntimeReadiness();
    } catch (error) {
      setBannerAction(resolveBannerAction(error, readiness));
      guards.current.textTrigger = false;
      setShowGlow(false);
      setTranslationText('');
      setBanner({
        title: '文本翻译失败',
        detail: formatError(error, readiness),
        tone: 'error',
      });
    } finally {
      window.setTimeout(() => {
        if (guards.current.textTrigger) {
          guards.current.textTrigger = false;
        }
      }, 1000);
      setBusyAction((current) => (current === 'text' ? null : current));
    }
  };

  const toneClasses: Record<BannerTone, string> = {
    neutral: 'border-white/10 bg-white/5 text-white/[0.82]',
    info: 'border-sky-400/20 bg-sky-500/10 text-sky-100',
    success: 'border-emerald-400/20 bg-emerald-500/10 text-emerald-100',
    warning: 'border-amber-400/20 bg-amber-500/10 text-amber-100',
    error: 'border-rose-400/20 bg-rose-500/10 text-rose-100',
  };

  const showBannerDetail = banner.tone === 'error';
  return (
    <div className="relative h-screen overflow-x-hidden overflow-y-auto bg-[radial-gradient(circle_at_top,_rgba(56,189,248,0.16),_transparent_34%),radial-gradient(circle_at_right,_rgba(139,92,246,0.18),_transparent_32%),linear-gradient(135deg,_#060b14,_#0b1220_52%,_#111827)] text-white">
      <div className="pointer-events-none absolute inset-0 overflow-hidden">
        <div className="absolute left-1/4 top-1/4 h-96 w-96 rounded-full bg-cyan-400/[0.12] blur-3xl animate-pulse" />
        <div className="absolute right-1/4 top-1/3 h-96 w-96 rounded-full bg-violet-500/[0.14] blur-3xl animate-pulse [animation-delay:1s]" />
        <div className="absolute bottom-1/4 left-1/2 h-80 w-80 rounded-full bg-fuchsia-500/10 blur-3xl" />
      </div>

      {welcomeChecked && !showWelcome ? (
        <>
          <header className="relative z-20 mx-auto flex w-full max-w-[760px] items-start justify-between gap-3 px-4 pt-4">
            <div className={`min-h-[68px] flex-1 rounded-[22px] border px-4 py-3 shadow-2xl backdrop-blur-xl ${toneClasses[banner.tone]}`}>
              <div className="text-sm font-semibold">{banner.title}</div>
              {showBannerDetail && banner.detail ? (
                <div className="mt-1 text-sm/6 text-white/[0.78]">{banner.detail}</div>
              ) : null}
              {bannerAction?.kind === 'permission' ? (
                <button
                  onClick={() => void openPermissionSettings(bannerAction.scope)}
                  className="mt-3 rounded-xl border border-white/15 bg-white/10 px-3 py-2 text-xs font-medium text-white transition-colors hover:bg-white/15"
                >
                  {bannerAction.label}
                </button>
              ) : bannerAction?.kind === 'settings' ? (
                <button
                  onClick={() => setSettingsVisible(true)}
                  className="mt-3 rounded-xl border border-white/15 bg-white/10 px-3 py-2 text-xs font-medium text-white transition-colors hover:bg-white/15"
                >
                  {bannerAction.label}
                </button>
              ) : null}
            </div>

            <div className="flex items-center gap-3">
              {busyAction ? (
                <div className="rounded-2xl border border-white/10 bg-white/5 px-4 py-3 text-xs font-medium text-white/[0.72] backdrop-blur-xl">
                  {busyAction === 'voice' ? '语音处理中' : '文本处理中'}
                </div>
              ) : null}
              <button
                onClick={() => setShowHistory(true)}
                className="rounded-2xl border border-white/10 bg-white/5 px-4 py-3 text-sm font-medium text-white/[0.88] shadow-2xl backdrop-blur-xl transition-colors hover:bg-white/10"
                title="历史"
              >
                历史
              </button>
              <button
                onClick={() => setSettingsVisible(true)}
                className="rounded-2xl border border-white/10 bg-white/5 px-4 py-3 text-sm font-medium text-white/[0.88] shadow-2xl backdrop-blur-xl transition-colors hover:bg-white/10"
                title="设置"
              >
                设置
              </button>
            </div>
          </header>

          <main className="relative z-10 mx-auto flex min-h-[calc(100vh-88px)] w-full max-w-[760px] items-start justify-center px-4 pb-5 pt-3">
            <section className="w-full rounded-[28px] border border-white/10 bg-white/[0.08] p-5 shadow-[0_28px_90px_rgba(0,0,0,0.3)] backdrop-blur-2xl">
              <div className="flex items-center gap-2">
                <div className="inline-flex items-center gap-2 rounded-full border border-white/10 bg-white/5 px-3 py-1.5 text-xs font-medium text-white/70">
                  <span className="h-2 w-2 rounded-full bg-emerald-300" />
                  本地翻译助手
                </div>
                <div className="rounded-full border border-white/10 bg-white/5 px-3 py-1.5 text-xs text-white/[0.7]">
                  {busyAction === 'voice' ? '语音处理中' : busyAction === 'text' ? '文本处理中' : '空闲'}
                </div>
              </div>

              <h1 className="mt-5 text-4xl font-semibold tracking-tight text-white">
                Visp
              </h1>
              <p className="mt-2 max-w-md text-sm leading-6 text-white/[0.7]">
                不离开当前工作流，直接完成语音翻译和文本替换。
              </p>

              <div className="mt-6 grid gap-3">
                <PrimaryActionCard
                  label={voiceShortcutLabel}
                  title="语音翻译"
                  detail="按一次开始录音，再按一次结束并自动写回。"
                  hint="更像 Handy / Typeless 的双击式开始与结束"
                  tone="from-rose-400 to-fuchsia-500"
                />
                <PrimaryActionCard
                  label={textShortcutLabel}
                  title="文本翻译"
                  detail="选中文本后预览译文，再确认替换。"
                  hint="先看结果，再决定是否替换当前内容"
                  tone="from-sky-400 to-indigo-500"
                />
              </div>

              <div className="mt-5 grid gap-2 text-xs text-white/[0.62] sm:grid-cols-3">
                <StatusChip label="Local First" />
                <StatusChip label="低打断" />
                <StatusChip label="确认后替换" />
              </div>
            </section>
          </main>
        </>
      ) : null}

      <Glow
        visible={showGlow}
        translationText={translationText}
        onConfirm={handleConfirmTranslation}
        onCancel={handleCancelTranslation}
        title="文本翻译预览"
        subtitle="按 Enter 替换当前选区，Esc 取消并恢复原状态。"
        confirmDisabled={busyAction === 'text'}
      />

      <Settings visible={settingsVisible} onClose={() => setSettingsVisible(false)} />

      <HistoryPanel visible={showHistory} onClose={() => setShowHistory(false)} />

      {welcomeChecked && showWelcome ? <Welcome onComplete={handleWelcomeComplete} /> : null}

      {welcomeChecked && !showWelcome ? <ModelSetup /> : null}
    </div>
  );
}

function PrimaryActionCard({
  label,
  title,
  detail,
  hint,
  tone,
}: {
  label: string;
  title: string;
  detail: string;
  hint: string;
  tone: string;
}) {
  return (
    <div className="rounded-[24px] border border-white/10 bg-white/[0.055] p-4 shadow-[0_18px_50px_rgba(2,6,23,0.18)]">
      <div className={`h-1.5 w-20 rounded-full bg-gradient-to-r ${tone}`} />
      <div className="mt-4 flex flex-wrap items-center gap-3">
        <kbd className="rounded-[14px] border border-white/10 bg-white/[0.08] px-3 py-2 font-mono text-sm text-white">
          {label}
        </kbd>
        <span className="text-base font-semibold text-white">{title}</span>
      </div>
      <p className="mt-3 text-sm leading-6 text-white/[0.7]">{detail}</p>
      <p className="mt-1 text-xs leading-5 text-white/[0.48]">{hint}</p>
    </div>
  );
}

function StatusChip({ label }: { label: string }) {
  return (
    <div className="rounded-full border border-white/10 bg-white/5 px-3 py-2 text-center">
      {label}
    </div>
  );
}

function formatError(error: unknown, readiness: RuntimeReadiness | null) {
  const message = error instanceof Error ? error.message : String(error);

  if (message.includes('No speech detected')) {
    return '没有检测到有效语音。请开始录音后说一句完整的话，再按一次快捷键结束。';
  }

  if (message.includes('未选中任何文本')) {
    return '没有读取到选中文本。请先选中内容，再触发文本翻译。';
  }

  if (message.includes('快捷键') && message.includes('占用')) {
    return '当前快捷键可能和系统或其他应用冲突了，可以去设置里换一组组合键。';
  }

  if (message.includes('语音识别权限') || message.includes('麦克风权限')) {
    return '语音翻译现在缺少系统权限。若系统设置列表里还没有 Visp，请先回到应用前台，再点击一次“授予权限”。';
  }

  if (
    message.includes('Ollama') ||
    message.includes('翻译后端') ||
    (!readiness?.translation_available && message.includes('翻译'))
  ) {
    return readiness?.translation_detail || '文本翻译当前没有可用后端。请先启动 Ollama 并准备推荐模型。';
  }

  return message;
}

function resolveBannerAction(error: unknown, readiness: RuntimeReadiness | null): BannerAction {
  const message = error instanceof Error ? error.message : String(error);

  if (message.includes('语音识别权限')) {
    return { kind: 'permission', scope: 'speech', label: '打开语音识别设置' };
  }

  if (message.includes('麦克风权限')) {
    return { kind: 'permission', scope: 'microphone', label: '打开麦克风设置' };
  }

  if (
    message.includes('Ollama') ||
    message.includes('翻译后端') ||
    (!readiness?.translation_available && message.includes('翻译'))
  ) {
    return { kind: 'settings', label: '查看运行诊断' };
  }

  return null;
}

type PermissionScope = 'speech' | 'microphone' | 'privacy' | null;

async function openPermissionSettings(scope: Exclude<PermissionScope, null>) {
  try {
    await invoke('open_permission_settings', { permission: scope });
  } catch (error) {
    console.error('Failed to open permission settings:', error);
  }
}

export default App;
