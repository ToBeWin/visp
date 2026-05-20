import { AnimatePresence, motion } from 'framer-motion';
import { invoke } from '@tauri-apps/api/core';
import { useEffect, useState } from 'react';

interface IslandProps {
  visible: boolean;
  phase: 'idle' | 'recording' | 'thinking';
  title: string;
  detail: string;
}

const phaseStyles: Record<IslandProps['phase'], { icon: string; gradient: string }> = {
  idle: {
    icon: '🎤',
    gradient: 'from-sky-400 via-blue-500 to-violet-500',
  },
  recording: {
    icon: '🔴',
    gradient: 'from-rose-500 via-pink-500 to-fuchsia-500',
  },
  thinking: {
    icon: '🤔',
    gradient: 'from-amber-400 via-orange-500 to-rose-500',
  },
};

export function Island({ visible, phase, title, detail }: IslandProps) {
  const [waveform, setWaveform] = useState<number[]>(Array(18).fill(0.2));
  const [elapsedSeconds, setElapsedSeconds] = useState(0);

  useEffect(() => {
    if (!visible || phase !== 'recording') {
      setElapsedSeconds(0);
      return;
    }

    const timer = window.setInterval(() => {
      void invoke<number[]>('get_waveform_data')
        .then((samples) => {
          if (Array.isArray(samples) && samples.length > 0) {
            setWaveform(samples.map((value) => Math.max(0.05, Math.min(1, value * 3.2))));
          }
        })
        .catch(() => {
          setWaveform(Array(18).fill(0.12));
        });
    }, 90);

    return () => window.clearInterval(timer);
  }, [phase, visible]);

  useEffect(() => {
    if (!visible || phase !== 'recording') {
      setElapsedSeconds(0);
      return;
    }

    const timer = window.setInterval(() => {
      setElapsedSeconds((current) => current + 1);
    }, 1000);

    return () => window.clearInterval(timer);
  }, [phase, visible]);

  const current = phaseStyles[phase];
  const elapsedLabel = `${String(Math.floor(elapsedSeconds / 60)).padStart(2, '0')}:${String(elapsedSeconds % 60).padStart(2, '0')}`;

  return (
    <AnimatePresence>
      {visible ? (
        <motion.div
          initial={{ y: -36, opacity: 0, scale: 0.96 }}
          animate={{ y: 0, opacity: 1, scale: 1 }}
          exit={{ y: -24, opacity: 0, scale: 0.98 }}
          transition={{ type: 'spring', stiffness: 320, damping: 26 }}
          className="w-[min(92vw,360px)]"
        >
          <div className="relative overflow-hidden rounded-[24px] border border-white/[0.12] bg-[#08111d]/85 px-4 py-3 shadow-[0_24px_80px_rgba(2,6,23,0.42)] backdrop-blur-2xl">
            <div className={`absolute inset-0 bg-gradient-to-r ${current.gradient} opacity-[0.14]`} />
            <div className="relative flex items-center gap-3">
              <div className="relative flex h-10 w-10 items-center justify-center rounded-[16px] border border-white/10 bg-white/[0.08]">
                {phase === 'recording' ? (
                  <>
                    <motion.span
                      className="absolute h-5 w-5 rounded-full bg-rose-400/30"
                      animate={{ scale: [1, 1.45, 1], opacity: [0.4, 0.08, 0.4] }}
                      transition={{ duration: 1.2, repeat: Infinity }}
                    />
                    <span className="relative text-sm text-rose-200">●</span>
                  </>
                ) : (
                  <span className="text-base">{current.icon}</span>
                )}
              </div>

              <div className="min-w-0 flex-1">
                <div className="flex items-center gap-2.5">
                  <div className="truncate text-sm font-semibold text-white/95">{title}</div>
                  {phase === 'recording' ? (
                    <span className="rounded-full border border-white/10 bg-white/[0.08] px-2 py-0.5 font-mono text-[11px] text-white/70">
                      {elapsedLabel}
                    </span>
                  ) : null}
                </div>
                <div className="truncate text-[12px] text-white/[0.62]">{detail}</div>
                {phase === 'recording' ? (
                  <div className="mt-2.5 flex h-8 items-end gap-1">
                    {waveform.map((height, index) => (
                      <motion.span
                        key={index}
                        className="w-1 rounded-full bg-white/[0.82]"
                        animate={{ height: `${Math.max(6, height * 26)}px` }}
                        transition={{ duration: 0.12 }}
                      />
                    ))}
                  </div>
                ) : null}
              </div>

              {phase !== 'recording' ? (
                <div className="rounded-full border border-white/10 bg-white/[0.08] px-3 py-1 text-[11px] font-medium text-white/[0.78]">
                  {phase === 'thinking' ? '处理中' : '就绪'}
                </div>
              ) : null}
            </div>
          </div>
        </motion.div>
      ) : null}
    </AnimatePresence>
  );
}
