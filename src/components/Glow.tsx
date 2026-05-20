import { AnimatePresence, motion } from 'framer-motion';
import { useEffect, useState } from 'react';

interface GlowProps {
  visible: boolean;
  translationText: string;
  onConfirm: () => void;
  onCancel: () => void;
  title?: string;
  subtitle?: string;
  confirmDisabled?: boolean;
}

export function Glow({
  visible,
  translationText,
  onConfirm,
  onCancel,
  title = '译文预览',
  subtitle = '按 Enter 可替换，按 Esc 取消',
  confirmDisabled = false,
}: GlowProps) {
  const [countdown, setCountdown] = useState(10);

  useEffect(() => {
    if (!visible) {
      return;
    }

    setCountdown(10);
    const timer = window.setTimeout(onCancel, 10000);
    const interval = window.setInterval(() => {
      setCountdown((current) => Math.max(0, current - 1));
    }, 1000);

    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'Enter' && !confirmDisabled) {
        e.preventDefault();
        onConfirm();
      } else if (e.key === 'Escape') {
        e.preventDefault();
        onCancel();
      }
    };

    window.addEventListener('keydown', handleKeyDown);

    return () => {
      window.clearTimeout(timer);
      window.clearInterval(interval);
      window.removeEventListener('keydown', handleKeyDown);
    };
  }, [confirmDisabled, onCancel, onConfirm, visible]);

  return (
    <AnimatePresence>
      {visible ? (
        <motion.div
          initial={{ opacity: 0, scale: 0.96, y: 12 }}
          animate={{ opacity: 1, scale: 1, y: 0 }}
          exit={{ opacity: 0, scale: 0.98, y: 8 }}
          transition={{ duration: 0.24 }}
          className="fixed inset-0 z-50 flex items-center justify-center p-4"
          onClick={onCancel}
        >
          <div className="absolute inset-0 bg-slate-950/42 backdrop-blur-sm" />

          <motion.div
            initial={{ y: 20 }}
            animate={{ y: 0 }}
            className="relative w-full max-w-[720px]"
            onClick={(e) => e.stopPropagation()}
          >
            <div className="relative overflow-hidden rounded-[28px] border border-white/[0.14] bg-white/10 shadow-[0_28px_90px_rgba(0,0,0,0.38)] backdrop-blur-2xl">
              <div
                className="absolute inset-0 rounded-[28px] p-[1px]"
                style={{
                  background:
                    'linear-gradient(90deg, transparent, rgba(255,255,255,0.36), transparent)',
                  backgroundSize: '200% 100%',
                  animation: 'glow-rotate 3s linear infinite',
                }}
              />

              <div className="relative p-6 md:p-7">
                <div className="mb-4 flex items-start justify-between gap-4">
                  <div>
                    <h3 className="text-xl font-semibold text-white">{title}</h3>
                    <p className="mt-1 text-sm text-white/[0.64]">{subtitle}</p>
                  </div>
                  <div className="rounded-full border border-white/10 bg-white/10 px-3 py-1 text-xs font-medium text-white/[0.78]">
                    {countdown}s
                  </div>
                </div>

                <div className="max-h-[320px] overflow-y-auto rounded-2xl border border-white/[0.08] bg-black/[0.15] p-5">
                  <p className="whitespace-pre-wrap text-[15px] leading-7 text-white/[0.92]">
                    {translationText || '暂无译文结果'}
                  </p>
                </div>

                <div className="mt-5 flex flex-wrap items-center justify-between gap-3 border-t border-white/10 pt-4">
                  <div className="flex items-center gap-2 text-sm text-white/[0.72]">
                    <kbd className="rounded-lg border border-white/10 bg-white/[0.08] px-2 py-1 font-mono text-xs">Enter</kbd>
                    <span>{confirmDisabled ? '不可用' : '替换'}</span>
                  </div>
                  <div className="flex items-center gap-2 text-sm text-white/[0.72]">
                    <kbd className="rounded-lg border border-white/10 bg-white/[0.08] px-2 py-1 font-mono text-xs">Esc</kbd>
                    <span>取消</span>
                  </div>
                  <div className="text-xs text-white/[0.46]">
                    10 秒后自动关闭
                  </div>
                </div>
              </div>
            </div>
          </motion.div>
        </motion.div>
      ) : null}
    </AnimatePresence>
  );
}
