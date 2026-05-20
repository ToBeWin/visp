import { AnimatePresence, motion } from 'framer-motion';
import { useEffect, useState, type ReactNode } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface SettingsProps {
  visible: boolean;
  onClose: () => void;
}

interface AppConfig {
  hotkeys: {
    voice_mode: string;
    text_mode: string;
  };
  translation: {
    direction: 'Auto' | 'ChineseToEnglish' | 'EnglishToChinese';
    preserve_formatting: boolean;
  };
  ui: {
    opacity: number;
    animation_speed: number;
    auto_dismiss_timeout: number;
  };
  logging: {
    level: 'Error' | 'Warn' | 'Info' | 'Debug';
    max_file_size: number;
  };
}

interface RuntimeReadiness {
  platform: string;
  voice_ready: boolean;
  text_ready: boolean;
  microphone_permission: boolean;
  speech_permission: boolean;
  asr_available: boolean;
  asr_summary: string;
  asr_detail: string;
  translation_available: boolean;
  translation_summary: string;
  translation_detail: string;
  models_dir: string;
  ollama_cli_available: boolean;
  ollama_service_reachable: boolean;
  ollama_model: string | null;
  recommended_actions: string[];
}

type PanelTone = 'idle' | 'loading' | 'saving' | 'error' | 'success';
const isMac = /Mac|iPhone|iPad|iPod/.test(navigator.platform);
const voicePlaceholder = isMac ? 'CommandOrControl+Shift+V' : 'Ctrl+Shift+V';
const textPlaceholder = isMac ? 'CommandOrControl+Shift+C' : 'Ctrl+Shift+C';

export function Settings({ visible, onClose }: SettingsProps) {
  const [config, setConfig] = useState<AppConfig | null>(null);
  const [readiness, setReadiness] = useState<RuntimeReadiness | null>(null);
  const [loading, setLoading] = useState(true);
  const [diagnosticsLoading, setDiagnosticsLoading] = useState(false);
  const [saving, setSaving] = useState(false);
  const [tone, setTone] = useState<PanelTone>('idle');
  const [message, setMessage] = useState('正在加载配置...');

  useEffect(() => {
    if (visible) {
      loadConfig();
      loadRuntimeReadiness();
    }
  }, [visible]);

  const loadConfig = async () => {
    try {
      setLoading(true);
      setTone('loading');
      setMessage('正在读取本地配置...');
      const cfg = await invoke<AppConfig>('get_config');
      setConfig(cfg);
      setTone('success');
      setMessage('配置已加载，可以修改后保存。');
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : String(error);
      setConfig(null);
      setTone('error');
      setMessage(`加载配置失败: ${errorMessage}`);
    } finally {
      setLoading(false);
    }
  };

  const saveConfig = async () => {
    if (!config) {
      return;
    }

    try {
      setSaving(true);
      setTone('saving');
      setMessage('正在保存配置...');
      await invoke('update_config', { newConfig: config });
      setTone('success');
      setMessage('配置保存成功。');
      onClose();
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : String(error);
      setTone('error');
      setMessage(`保存失败: ${errorMessage}`);
    } finally {
      setSaving(false);
    }
  };

  const loadRuntimeReadiness = async () => {
    try {
      setDiagnosticsLoading(true);
      const next = await invoke<RuntimeReadiness>('get_runtime_readiness');
      setReadiness(next);
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : String(error);
      setReadiness(null);
      setTone('error');
      setMessage(`运行诊断失败: ${errorMessage}`);
    } finally {
      setDiagnosticsLoading(false);
    }
  };

  const updateConfig = (path: string[], value: unknown) => {
    if (!config) {
      return;
    }

    const next = structuredClone(config) as AppConfig;
    let cursor: Record<string, unknown> = next as unknown as Record<string, unknown>;

    for (let index = 0; index < path.length - 1; index += 1) {
      cursor = cursor[path[index]] as Record<string, unknown>;
    }

    cursor[path[path.length - 1]] = value;
    setConfig(next);
    setTone('idle');
    setMessage('配置已修改，记得保存。');
  };

  const statusStyles: Record<PanelTone, string> = {
    idle: 'border-white/10 bg-white/5 text-white/[0.72]',
    loading: 'border-sky-400/20 bg-sky-500/10 text-sky-100',
    saving: 'border-amber-400/20 bg-amber-500/10 text-amber-100',
    error: 'border-rose-400/20 bg-rose-500/10 text-rose-100',
    success: 'border-emerald-400/20 bg-emerald-500/10 text-emerald-100',
  };

  return (
    <AnimatePresence>
      {visible ? (
        <motion.div
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          exit={{ opacity: 0 }}
          className="fixed inset-0 z-50 flex items-center justify-center p-4"
          onClick={onClose}
        >
          <div className="absolute inset-0 bg-slate-950/60 backdrop-blur-md" />

          <motion.div
            initial={{ scale: 0.96, y: 16 }}
            animate={{ scale: 1, y: 0 }}
            exit={{ scale: 0.96, y: 16 }}
            className="relative w-full max-w-2xl max-h-[86vh] overflow-hidden"
            onClick={(e) => e.stopPropagation()}
          >
            <div className="rounded-[28px] border border-white/[0.14] bg-white/10 shadow-[0_24px_80px_rgba(0,0,0,0.38)] backdrop-blur-2xl">
              <div className="flex items-center justify-between border-b border-white/10 px-6 py-5">
                <div>
                  <h2 className="text-2xl font-semibold text-white">设置</h2>
                  <p className="mt-1 text-sm text-white/[0.55]">快捷键、翻译偏好和日志级别都在这里统一管理。</p>
                </div>
                <button
                  onClick={onClose}
                  className="rounded-xl border border-white/10 bg-white/5 px-3 py-2 text-white/[0.85] transition-colors hover:bg-white/10"
                >
                  关闭
                </button>
              </div>

              <div className="border-b border-white/[0.08] px-6 py-4">
                <div className={`rounded-2xl border px-4 py-3 text-sm ${statusStyles[tone]}`}>
                  {loading || saving ? '处理中...' : message}
                </div>
              </div>

              <div className="max-h-[62vh] overflow-y-auto px-6 py-6">
                {loading ? (
                  <div className="flex items-center justify-center gap-3 rounded-3xl border border-white/10 bg-white/5 py-12 text-white/70">
                    <div className="h-5 w-5 animate-spin rounded-full border-2 border-white/25 border-t-white" />
                    <span>正在加载配置...</span>
                  </div>
                ) : config ? (
                  <div className="space-y-8">
                    <SettingsGroup title="快捷键" description="设置语音和文本翻译的全局快捷键。">
                      <Field label="语音翻译模式">
                        <input
                          type="text"
                          value={config.hotkeys.voice_mode}
                          onChange={(e) => updateConfig(['hotkeys', 'voice_mode'], e.target.value)}
                          className="w-full rounded-2xl border border-white/10 bg-white/5 px-4 py-3 text-white outline-none transition-colors placeholder:text-white/30 focus:border-cyan-400/40"
                          placeholder={voicePlaceholder}
                        />
                      </Field>
                      <Field label="文本翻译模式">
                        <input
                          type="text"
                          value={config.hotkeys.text_mode}
                          onChange={(e) => updateConfig(['hotkeys', 'text_mode'], e.target.value)}
                          className="w-full rounded-2xl border border-white/10 bg-white/5 px-4 py-3 text-white outline-none transition-colors placeholder:text-white/30 focus:border-cyan-400/40"
                          placeholder={textPlaceholder}
                        />
                      </Field>
                      <div className="rounded-2xl border border-white/10 bg-white/5 px-4 py-3 text-sm leading-6 text-white/60">
                        语音翻译现在采用“按一次开始、再按一次结束”的方式。`Fn` 在当前热键库里不稳定，单独 `Space` 会影响正常输入，建议优先使用组合键或 `F` 系列按键。
                      </div>
                    </SettingsGroup>

                    <SettingsGroup title="运行诊断" description="这里会显示当前机器上哪些能力已经真正可用。">
                      <div className="grid gap-4 md:grid-cols-2">
                        <DiagnosticCard
                          title="语音翻译"
                          ready={readiness?.voice_ready ?? false}
                          summary={readiness?.asr_summary ?? '正在检查...'}
                          detail={readiness?.asr_detail ?? '正在检查麦克风权限、语音识别权限和 ASR 后端。'}
                        >
                          <PermissionRow
                            label="麦克风权限"
                            granted={readiness?.microphone_permission ?? false}
                            actionLabel="打开麦克风设置"
                            onAction={() => invoke('open_permission_settings', { permission: 'microphone' })}
                          />
                          <PermissionRow
                            label="语音识别权限"
                            granted={readiness?.speech_permission ?? false}
                            actionLabel="打开语音识别设置"
                            onAction={() => invoke('open_permission_settings', { permission: 'speech' })}
                          />
                        </DiagnosticCard>

                        <DiagnosticCard
                          title="文本翻译"
                          ready={readiness?.text_ready ?? false}
                          summary={readiness?.translation_summary ?? '正在检查...'}
                          detail={readiness?.translation_detail ?? '正在检查可用翻译后端。'}
                        >
                          <StatusRow label="Ollama 已安装" ready={readiness?.ollama_cli_available ?? false} />
                          <StatusRow label="Ollama 服务可访问" ready={readiness?.ollama_service_reachable ?? false} />
                          <div className="rounded-2xl border border-white/10 bg-white/5 px-4 py-3 text-sm leading-6 text-white/65">
                            <p>当前模型：{readiness?.ollama_model ?? '未检测到'}</p>
                            <p className="mt-1">模型目录：{readiness?.models_dir ?? '正在检查...'}</p>
                          </div>
                        </DiagnosticCard>
                      </div>

                      {readiness?.recommended_actions?.length ? (
                        <div className="rounded-2xl border border-amber-400/20 bg-amber-500/10 px-4 py-3 text-sm leading-6 text-amber-100">
                          {readiness.recommended_actions.map((item) => (
                            <p key={item}>{item}</p>
                          ))}
                        </div>
                      ) : (
                        <div className="rounded-2xl border border-emerald-400/20 bg-emerald-500/10 px-4 py-3 text-sm leading-6 text-emerald-100">
                          当前机器上的主链路已经具备运行条件，可以直接使用。
                        </div>
                      )}
                    </SettingsGroup>

                    <SettingsGroup title="翻译" description="控制翻译方向和格式保留策略。">
                      <Field label="翻译方向">
                        <select
                          value={config.translation.direction}
                          onChange={(e) =>
                            updateConfig(['translation', 'direction'], e.target.value as AppConfig['translation']['direction'])
                          }
                          className="w-full rounded-2xl border border-white/10 bg-white/5 px-4 py-3 text-white outline-none transition-colors focus:border-cyan-400/40"
                        >
                          <option value="Auto">自动检测</option>
                          <option value="ChineseToEnglish">中文 → 英文</option>
                          <option value="EnglishToChinese">英文 → 中文</option>
                        </select>
                      </Field>

                      <label className="flex items-center gap-3 rounded-2xl border border-white/10 bg-white/5 px-4 py-3 text-sm text-white/[0.80]">
                        <input
                          type="checkbox"
                          checked={config.translation.preserve_formatting}
                          onChange={(e) =>
                            updateConfig(['translation', 'preserve_formatting'], e.target.checked)
                          }
                          className="h-4 w-4 rounded border-white/20 bg-white/5"
                        />
                        <span>保留原文格式和换行</span>
                      </label>
                    </SettingsGroup>

                    <SettingsGroup title="界面" description="控制透明度和动画节奏。">
                      <Field label={`透明度 ${(config.ui.opacity * 100).toFixed(0)}%`}>
                        <input
                          type="range"
                          min="0.5"
                          max="1"
                          step="0.05"
                          value={config.ui.opacity}
                          onChange={(e) => updateConfig(['ui', 'opacity'], parseFloat(e.target.value))}
                          className="w-full"
                        />
                      </Field>

                      <Field label={`动画速度 ${config.ui.animation_speed.toFixed(1)}x`}>
                        <input
                          type="range"
                          min="0.5"
                          max="2"
                          step="0.1"
                          value={config.ui.animation_speed}
                          onChange={(e) =>
                            updateConfig(['ui', 'animation_speed'], parseFloat(e.target.value))
                          }
                          className="w-full"
                        />
                      </Field>

                      <Field label={`自动关闭时间 ${config.ui.auto_dismiss_timeout} 秒`}>
                        <input
                          type="range"
                          min="5"
                          max="30"
                          step="5"
                          value={config.ui.auto_dismiss_timeout}
                          onChange={(e) =>
                            updateConfig(['ui', 'auto_dismiss_timeout'], parseInt(e.target.value, 10))
                          }
                          className="w-full"
                        />
                      </Field>
                    </SettingsGroup>

                    <SettingsGroup title="日志" description="调整日志级别，便于排查本地问题。">
                      <Field label="日志级别">
                        <select
                          value={config.logging.level}
                          onChange={(e) =>
                            updateConfig(['logging', 'level'], e.target.value as AppConfig['logging']['level'])
                          }
                          className="w-full rounded-2xl border border-white/10 bg-white/5 px-4 py-3 text-white outline-none transition-colors focus:border-cyan-400/40"
                        >
                          <option value="Error">Error</option>
                          <option value="Warn">Warn</option>
                          <option value="Info">Info</option>
                          <option value="Debug">Debug</option>
                        </select>
                      </Field>

                      <Field label={`最大日志文件大小 ${config.logging.max_file_size} MB`}>
                        <input
                          type="number"
                          min="10"
                          max="200"
                          step="10"
                          value={config.logging.max_file_size}
                          onChange={(e) =>
                            updateConfig(['logging', 'max_file_size'], parseInt(e.target.value, 10))
                          }
                          className="w-full rounded-2xl border border-white/10 bg-white/5 px-4 py-3 text-white outline-none transition-colors placeholder:text-white/30 focus:border-cyan-400/40"
                        />
                      </Field>
                    </SettingsGroup>
                  </div>
                ) : (
                  <div className="rounded-3xl border border-rose-400/20 bg-rose-500/10 px-5 py-6 text-center text-rose-100">
                    配置加载失败，请稍后重试。
                  </div>
                )}
              </div>

              <div className="flex items-center justify-between gap-3 border-t border-white/10 px-6 py-5">
                <button
                  onClick={() => {
                    loadConfig();
                    loadRuntimeReadiness();
                  }}
                  className="rounded-xl border border-white/10 bg-white/5 px-4 py-2.5 text-sm text-white/[0.80] transition-colors hover:bg-white/10"
                >
                  {diagnosticsLoading ? '检查中...' : '重新加载'}
                </button>
                <div className="flex items-center gap-3">
                  <button
                    onClick={onClose}
                    className="rounded-xl border border-white/10 bg-white/5 px-4 py-2.5 text-sm text-white/[0.80] transition-colors hover:bg-white/10"
                  >
                    取消
                  </button>
                  <button
                    onClick={saveConfig}
                    disabled={saving || loading || !config}
                    className="rounded-xl bg-[linear-gradient(135deg,_#7c3aed,_#3b82f6)] px-4 py-2.5 text-sm font-semibold text-white shadow-lg shadow-purple-500/20 transition-transform duration-200 hover:scale-[1.01] disabled:cursor-not-allowed disabled:opacity-50"
                  >
                    {saving ? '保存中...' : '保存'}
                  </button>
                </div>
              </div>
            </div>
          </motion.div>
        </motion.div>
      ) : null}
    </AnimatePresence>
  );
}

function DiagnosticCard({
  title,
  ready,
  summary,
  detail,
  children,
}: {
  title: string;
  ready: boolean;
  summary: string;
  detail: string;
  children: ReactNode;
}) {
  return (
    <div className="rounded-[24px] border border-white/10 bg-white/5 p-4">
      <div className="flex items-center justify-between gap-3">
        <h4 className="text-base font-semibold text-white">{title}</h4>
        <span
          className={`rounded-full border px-3 py-1 text-xs ${
            ready
              ? 'border-emerald-400/20 bg-emerald-500/10 text-emerald-100'
              : 'border-amber-400/20 bg-amber-500/10 text-amber-100'
          }`}
        >
          {ready ? '可用' : '待处理'}
        </span>
      </div>
      <p className="mt-3 text-sm font-medium text-white/80">{summary}</p>
      <p className="mt-2 text-sm leading-6 text-white/55">{detail}</p>
      <div className="mt-4 space-y-3">{children}</div>
    </div>
  );
}

function PermissionRow({
  label,
  granted,
  actionLabel,
  onAction,
}: {
  label: string;
  granted: boolean;
  actionLabel: string;
  onAction: () => void;
}) {
  return (
    <div className="flex items-center justify-between gap-3 rounded-2xl border border-white/10 bg-white/5 px-4 py-3">
      <div>
        <p className="text-sm text-white/80">{label}</p>
        <p className={`mt-1 text-xs ${granted ? 'text-emerald-200' : 'text-amber-100'}`}>
          {granted ? '已授权' : '未授权'}
        </p>
      </div>
      {!granted ? (
        <button
          onClick={onAction}
          className="rounded-xl border border-white/10 bg-white/10 px-3 py-2 text-xs text-white transition-colors hover:bg-white/15"
        >
          {actionLabel}
        </button>
      ) : null}
    </div>
  );
}

function StatusRow({
  label,
  ready,
}: {
  label: string;
  ready: boolean;
}) {
  return (
    <div className="flex items-center justify-between gap-3 rounded-2xl border border-white/10 bg-white/5 px-4 py-3">
      <p className="text-sm text-white/80">{label}</p>
      <span className={`text-xs ${ready ? 'text-emerald-200' : 'text-amber-100'}`}>
        {ready ? '已就绪' : '未就绪'}
      </span>
    </div>
  );
}

function SettingsGroup({
  title,
  description,
  children,
}: {
  title: string;
  description: string;
  children: ReactNode;
}) {
  return (
    <section className="rounded-[28px] border border-white/10 bg-white/5 p-5">
      <div className="mb-4">
        <h3 className="text-lg font-semibold text-white">{title}</h3>
        <p className="mt-1 text-sm text-white/[0.55]">{description}</p>
      </div>
      <div className="space-y-4">{children}</div>
    </section>
  );
}

function Field({
  label,
  children,
}: {
  label: string;
  children: ReactNode;
}) {
  return (
    <label className="block">
      <span className="mb-2 block text-sm text-white/[0.72]">{label}</span>
      {children}
    </label>
  );
}
