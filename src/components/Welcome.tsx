import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { motion, AnimatePresence } from 'framer-motion';

interface WelcomeProps {
  onComplete: () => void;
}

type Step = 'welcome' | 'permissions' | 'shortcuts' | 'complete';

interface PermissionResponse {
  microphone: boolean;
  speech: boolean;
  granted: boolean;
}

const isMac = /Mac|iPhone|iPad|iPod/.test(navigator.platform);
const voiceShortcutKeys = isMac ? ['Command', 'Shift', 'V'] : ['Ctrl', 'Shift', 'V'];
const textShortcutKeys = isMac ? ['Command', 'Shift', 'C'] : ['Ctrl', 'Shift', 'C'];

export function Welcome({ onComplete }: WelcomeProps) {
  const [currentStep, setCurrentStep] = useState<Step>('welcome');

  const nextStep = () => {
    const steps: Step[] = ['welcome', 'permissions', 'shortcuts', 'complete'];
    const currentIndex = steps.indexOf(currentStep);

    if (currentIndex < steps.length - 1) {
      setCurrentStep(steps[currentIndex + 1]);
      return;
    }

    onComplete();
  };

  return (
    <div className="fixed inset-0 z-[100] flex items-start justify-center overflow-y-auto bg-[radial-gradient(circle_at_top,_rgba(56,189,248,0.18),_transparent_36%),linear-gradient(135deg,_#07111f,_#0f172a_55%,_#111827)] px-4 py-4 sm:items-center sm:py-8">
      <AnimatePresence mode="wait">
        {currentStep === 'welcome' && (
          <WelcomeStep key="welcome" onNext={nextStep} onSkip={onComplete} />
        )}
        {currentStep === 'permissions' && (
          <PermissionsStep key="permissions" onNext={nextStep} onSkip={onComplete} />
        )}
        {currentStep === 'shortcuts' && (
          <ShortcutsStep key="shortcuts" onNext={nextStep} />
        )}
        {currentStep === 'complete' && (
          <CompleteStep key="complete" onFinish={onComplete} />
        )}
      </AnimatePresence>
    </div>
  );
}

function WelcomeStep({ onNext, onSkip }: { onNext: () => void; onSkip: () => void }) {
  return (
    <motion.div
      initial={{ opacity: 0, scale: 0.94, y: 20 }}
      animate={{ opacity: 1, scale: 1, y: 0 }}
      exit={{ opacity: 0, scale: 0.96, y: 10 }}
      className="my-auto w-full max-w-2xl overflow-y-auto rounded-[32px] border border-white/[0.14] bg-white/10 p-8 shadow-[0_24px_80px_rgba(0,0,0,0.32)] backdrop-blur-2xl max-h-[calc(100vh-2rem)] md:p-12"
    >
      <motion.div
        initial={{ scale: 0 }}
        animate={{ scale: 1 }}
        transition={{ type: 'spring', duration: 0.7 }}
        className="mx-auto mb-8 flex h-28 w-28 items-center justify-center rounded-[28px] bg-[linear-gradient(135deg,_rgba(34,211,238,1),_rgba(59,130,246,1),_rgba(168,85,247,1))] shadow-2xl shadow-cyan-500/20"
      >
        <motion.div
          animate={{ rotateY: [0, 360] }}
          transition={{ duration: 2.2, repeat: Infinity, ease: 'linear' }}
          className="text-6xl font-black text-white"
        >
          V
        </motion.div>
      </motion.div>

      <motion.h1
        initial={{ opacity: 0, y: 16 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ delay: 0.15 }}
        className="text-center text-4xl font-semibold tracking-tight text-white md:text-5xl"
      >
        欢迎使用 Visp
      </motion.h1>

      <motion.p
        initial={{ opacity: 0, y: 16 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ delay: 0.22 }}
        className="mx-auto mt-4 max-w-xl text-center text-base text-white/[0.72] md:text-lg"
      >
        一个把翻译能力嵌入工作流的本地桌面工具。按一次开始录音，选中后再确认替换，让翻译尽量不打断你的思路。
      </motion.p>

      <motion.div
        initial={{ opacity: 0, y: 16 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ delay: 0.3 }}
        className="mt-8 grid gap-3"
      >
        <Feature icon="🎤" title="语音翻译" description="按一次开始录音，再按一次结束并自动输入结果" />
        <Feature icon="📝" title="文本翻译" description="选中文本后预览译文，再决定是否替换" />
        <Feature icon="🔒" title="本地优先" description="尽量在设备上完成处理，减少隐私和延迟顾虑" />
        <Feature icon="⚡" title="极速工作流" description="快捷键驱动，尽量把操作压缩到一次动作内" />
      </motion.div>

      <motion.div
        initial={{ opacity: 0, y: 16 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ delay: 0.38 }}
        className="mt-10 flex flex-col gap-3 sm:flex-row"
      >
        <button
          onClick={onNext}
          className="flex-1 rounded-2xl bg-[linear-gradient(135deg,_#22d3ee,_#3b82f6,_#8b5cf6)] px-6 py-4 font-semibold text-white shadow-lg shadow-cyan-500/20 transition-transform duration-200 hover:scale-[1.01]"
        >
          开始设置
        </button>
        <button
          onClick={onSkip}
          className="rounded-2xl border border-white/[0.15] bg-white/5 px-6 py-4 font-medium text-white/[0.78] transition-colors hover:bg-white/10"
        >
          跳过
        </button>
      </motion.div>

      <motion.p
        initial={{ opacity: 0 }}
        animate={{ opacity: 1 }}
        transition={{ delay: 0.45 }}
        className="mt-6 text-center text-sm text-white/42"
      >
        只需要 3 步即可开始使用，主界面会继续检查后端可用性和本地资源状态
      </motion.p>
    </motion.div>
  );
}

function PermissionsStep({ onNext, onSkip }: { onNext: () => void; onSkip: () => void }) {
  const [checking, setChecking] = useState(false);
  const [granted, setGranted] = useState(false);
  const [error, setError] = useState('');

  const permissionScope = error.includes('语音识别权限')
    ? 'speech'
    : error.includes('麦克风权限')
      ? 'microphone'
      : 'privacy';

  const checkPermission = async () => {
    try {
      setChecking(true);
      setError('');
      const result = await invoke<PermissionResponse>('request_permissions');
      setGranted(result.granted);

      if (result.granted) {
        window.setTimeout(onNext, 700);
      } else {
        setError('权限尚未完全授予，请在系统设置中允许麦克风和语音识别后重试。');
      }
    } catch (err) {
      const message = err instanceof Error ? err.message : String(err);
      setError(message);
    } finally {
      setChecking(false);
    }
  };

  return (
    <motion.div
      initial={{ opacity: 0, x: 80 }}
      animate={{ opacity: 1, x: 0 }}
      exit={{ opacity: 0, x: -80 }}
      className="my-auto w-full max-w-2xl overflow-y-auto rounded-[32px] border border-white/[0.14] bg-white/10 p-8 shadow-[0_24px_80px_rgba(0,0,0,0.32)] backdrop-blur-2xl max-h-[calc(100vh-2rem)] md:p-12"
    >
      <StepDots active={2} total={3} />

      <motion.div
        animate={{ scale: [1, 1.06, 1] }}
        transition={{ duration: 2, repeat: Infinity }}
        className="mx-auto mb-8 flex h-24 w-24 items-center justify-center rounded-full bg-[linear-gradient(135deg,_#fb7185,_#ec4899)] shadow-2xl shadow-pink-500/20"
      >
        <span className="text-5xl">🎙️</span>
      </motion.div>

      <h2 className="text-center text-3xl font-semibold text-white">麦克风权限</h2>
      <p className="mx-auto mt-3 max-w-xl text-center text-base text-white/[0.72]">
        语音翻译需要同时访问麦克风和系统语音识别。所有处理都尽量在本地完成，不会把内容上传到服务器。
      </p>

      <div className="mt-8 rounded-3xl border border-sky-400/[0.15] bg-sky-400/[0.08] p-5">
        <p className="mb-3 text-sm font-semibold text-sky-100">为什么需要这个权限</p>
        <ul className="space-y-2 text-sm text-white/70">
          <li>录制你的语音以便进行识别和翻译</li>
          <li>调用系统语音识别把音频转换成文字</li>
          <li>在本地完成处理，减少隐私顾虑</li>
          <li>你可以随时在系统设置中撤销授权</li>
        </ul>
      </div>

      <div className="mt-6 space-y-3">
        {error ? (
          <div className="rounded-2xl border border-rose-400/20 bg-rose-500/10 px-4 py-3 text-sm text-rose-100">
            权限检查失败: {error}
          </div>
        ) : null}
        {granted ? (
          <div className="rounded-2xl border border-emerald-400/20 bg-emerald-500/10 px-4 py-3 text-sm text-emerald-100">
            麦克风和语音识别权限都已就绪，可以继续下一步。
          </div>
        ) : null}
      </div>

      <div className="mt-8 flex flex-col gap-3 sm:flex-row">
        <button
          onClick={checkPermission}
          disabled={checking || granted}
          className="flex-1 rounded-2xl bg-[linear-gradient(135deg,_#3b82f6,_#8b5cf6)] px-6 py-4 font-semibold text-white shadow-lg shadow-blue-500/20 transition-transform duration-200 hover:scale-[1.01] disabled:cursor-not-allowed disabled:opacity-60"
        >
          {checking ? '检查中...' : granted ? '已授权' : '授予权限'}
        </button>

        {!granted && error ? (
          <button
            onClick={() => void invoke('open_permission_settings', { permission: permissionScope })}
            className="rounded-2xl border border-white/[0.15] bg-white/5 px-6 py-4 font-medium text-white/[0.78] transition-colors hover:bg-white/10"
          >
            打开系统设置
          </button>
        ) : null}

        {!granted ? (
          <button
            onClick={onSkip}
            className="rounded-2xl border border-white/[0.15] bg-white/5 px-6 py-4 font-medium text-white/[0.78] transition-colors hover:bg-white/10"
          >
            跳过
          </button>
        ) : null}
      </div>
    </motion.div>
  );
}

function ShortcutsStep({ onNext }: { onNext: () => void }) {
  return (
    <motion.div
      initial={{ opacity: 0, x: 80 }}
      animate={{ opacity: 1, x: 0 }}
      exit={{ opacity: 0, x: -80 }}
      className="my-auto w-full max-w-2xl overflow-y-auto rounded-[32px] border border-white/[0.14] bg-white/10 p-8 shadow-[0_24px_80px_rgba(0,0,0,0.32)] backdrop-blur-2xl max-h-[calc(100vh-2rem)] md:p-12"
    >
      <StepDots active={3} total={3} />

      <h2 className="text-center text-3xl font-semibold text-white">快捷键说明</h2>
      <p className="mx-auto mt-3 max-w-xl text-center text-base text-white/[0.72]">
        记住这两个组合键，Visp 就能像系统能力一样自然地融入你的工作流。
      </p>

      <div className="mt-8 space-y-4">
        <ShortcutCard
          keys={voiceShortcutKeys}
          title="语音翻译"
          description="按一次开始录音，再按一次结束并输入翻译结果"
          icon="🎤"
          color="from-rose-400 to-fuchsia-500"
        />
        <ShortcutCard
          keys={textShortcutKeys}
          title="文本翻译"
          description="选中文本后触发预览，确认后再替换原内容"
          icon="📝"
          color="from-sky-400 to-indigo-500"
        />
      </div>

      <div className="mt-8 rounded-3xl border border-amber-300/[0.15] bg-amber-400/10 p-5 text-sm text-amber-50">
        后续的后端可用性检查和运行准备会在主界面与设置页里统一处理，不需要你反复手动排查。
      </div>

      <div className="mt-8">
        <button
          onClick={onNext}
          className="w-full rounded-2xl bg-[linear-gradient(135deg,_#22c55e,_#10b981)] px-6 py-4 font-semibold text-white shadow-lg shadow-emerald-500/20 transition-transform duration-200 hover:scale-[1.01]"
        >
          完成设置
        </button>
      </div>
    </motion.div>
  );
}

function CompleteStep({ onFinish }: { onFinish: () => void }) {
  useEffect(() => {
    const timer = window.setTimeout(onFinish, 2500);
    return () => window.clearTimeout(timer);
  }, [onFinish]);

  return (
    <motion.div
      initial={{ opacity: 0, scale: 0.9 }}
      animate={{ opacity: 1, scale: 1 }}
      exit={{ opacity: 0, scale: 0.9 }}
      className="my-auto w-full max-w-2xl overflow-y-auto rounded-[32px] border border-white/[0.14] bg-white/10 p-8 text-center shadow-[0_24px_80px_rgba(0,0,0,0.32)] backdrop-blur-2xl max-h-[calc(100vh-2rem)] md:p-12"
    >
      <motion.div
        initial={{ scale: 0 }}
        animate={{ scale: 1 }}
        transition={{ type: 'spring', duration: 0.7 }}
        className="mx-auto mb-8 flex h-28 w-28 items-center justify-center rounded-full bg-[linear-gradient(135deg,_#34d399,_#10b981)] shadow-2xl shadow-emerald-500/20"
      >
        <span className="text-6xl text-white">✓</span>
      </motion.div>

      <h2 className="text-4xl font-semibold text-white">一切就绪</h2>
      <p className="mx-auto mt-4 max-w-xl text-lg text-white/[0.72]">
        现在可以开始使用 Visp 了。接下来进入主界面后，应用会自动检查语音识别、翻译后端和本地资源是否可用。
      </p>

      <div className="mt-8 flex items-center justify-center gap-2 text-white/42">
        <div className="h-2 w-2 animate-bounce rounded-full bg-white/[0.45]" />
        <div className="h-2 w-2 animate-bounce rounded-full bg-white/[0.45] [animation-delay:150ms]" />
        <div className="h-2 w-2 animate-bounce rounded-full bg-white/[0.45] [animation-delay:300ms]" />
      </div>
    </motion.div>
  );
}

function StepDots({ active, total }: { active: number; total: number }) {
  return (
    <div className="mb-8 flex items-center justify-center gap-2">
      {Array.from({ length: total }, (_, index) => {
        const step = index + 1;
        const filled = step <= active;
        return (
          <div
            key={step}
            className={`h-2.5 rounded-full transition-all ${filled ? 'w-8 bg-gradient-to-r from-cyan-400 to-purple-500' : 'w-2.5 bg-white/20'}`}
          />
        );
      })}
    </div>
  );
}

function Feature({ icon, title, description }: { icon: string; title: string; description: string }) {
  return (
    <div className="flex items-start gap-4 rounded-2xl border border-white/[0.08] bg-white/[0.06] p-4">
      <span className="text-3xl">{icon}</span>
      <div>
        <h3 className="font-semibold text-white">{title}</h3>
        <p className="text-sm text-white/[0.64]">{description}</p>
      </div>
    </div>
  );
}

function ShortcutCard({
  keys,
  title,
  description,
  icon,
  color,
}: {
  keys: string[];
  title: string;
  description: string;
  icon: string;
  color: string;
}) {
  return (
    <div className="rounded-3xl border border-white/[0.08] bg-white/[0.06] p-5">
      <div className="mb-3 flex items-center gap-4">
        <div className={`flex h-12 w-12 items-center justify-center rounded-2xl bg-gradient-to-br ${color} shadow-lg`}>
          <span className="text-2xl">{icon}</span>
        </div>
        <div className="min-w-0 flex-1">
          <h3 className="text-lg font-semibold text-white">{title}</h3>
          <p className="text-sm text-white/60">{description}</p>
        </div>
        <div className="flex shrink-0 items-center gap-2">
          {keys.map((key, index) => (
            <span key={key} className="flex items-center">
              <kbd className="rounded-xl border border-white/10 bg-white/[0.08] px-3 py-2 text-sm font-semibold text-white/[0.80]">
                {key}
              </kbd>
              {index < keys.length - 1 ? <span className="mx-2 text-white/35">+</span> : null}
            </span>
          ))}
        </div>
      </div>
    </div>
  );
}
