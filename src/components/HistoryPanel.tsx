import { useState, useEffect, useRef } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { motion, AnimatePresence } from 'framer-motion';

interface TranslationRecord {
  id: string;
  timestamp: string;
  source_text: string;
  translated_text: string;
  source_lang: string;
  target_lang: string;
  mode: string;
}

function formatDate(timestamp: string): string {
  const date = new Date(timestamp);
  const now = new Date();
  const diffMs = now.getTime() - date.getTime();
  const diffMins = Math.floor(diffMs / 60000);
  const diffHours = Math.floor(diffMs / 3600000);
  const diffDays = Math.floor(diffMs / 86400000);

  if (diffMins < 1) return '刚刚';
  if (diffMins < 60) return `${diffMins} 分钟前`;
  if (diffHours < 24) return `${diffHours} 小时前`;
  if (diffDays < 7) return `${diffDays} 天前`;
  return date.toLocaleDateString('zh-CN', {
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
  });
}

function groupByDate(entries: TranslationRecord[]): Map<string, TranslationRecord[]> {
  const groups = new Map<string, TranslationRecord[]>();
  for (const entry of entries) {
    const date = new Date(entry.timestamp);
    const key = date.toLocaleDateString('zh-CN', {
      year: 'numeric',
      month: 'long',
      day: 'numeric',
    });
    if (!groups.has(key)) groups.set(key, []);
    groups.get(key)!.push(entry);
  }
  return groups;
}

function HistoryEntryCard({
  entry,
  onDelete,
}: {
  entry: TranslationRecord;
  onDelete: (id: string) => void;
}) {
  const [expanded, setExpanded] = useState(false);

  return (
    <motion.div
      layout
      initial={{ opacity: 0, y: 10 }}
      animate={{ opacity: 1, y: 0 }}
      className="rounded-xl border border-white/10 bg-white/[0.05] p-3 transition-colors hover:bg-white/[0.08]"
    >
      <div className="cursor-pointer" onClick={() => setExpanded(!expanded)}>
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-2">
            <span
              className={`rounded-full px-2 py-0.5 text-[10px] font-medium ${
                entry.mode === 'voice'
                  ? 'bg-rose-500/20 text-rose-300'
                  : 'bg-sky-500/20 text-sky-300'
              }`}
            >
              {entry.mode === 'voice' ? '语音' : '文本'}
            </span>
            <span className="text-xs text-white/40">{formatDate(entry.timestamp)}</span>
          </div>
          <button
            onClick={(e) => {
              e.stopPropagation();
              onDelete(entry.id);
            }}
            className="rounded-lg p-1 text-white/30 transition-colors hover:bg-white/10 hover:text-rose-400"
            title="删除"
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              className="h-3.5 w-3.5"
              viewBox="0 0 20 20"
              fill="currentColor"
            >
              <path
                fillRule="evenodd"
                d="M9 2a1 1 0 00-.894.553L7.382 4H4a1 1 0 000 2v10a2 2 0 002 2h8a2 2 0 002-2V6a1 1 0 100-2h-3.382l-.724-1.447A1 1 0 0011 2H9zM7 8a1 1 0 012 0v6a1 1 0 11-2 0V8zm5-1a1 1 0 00-1 1v6a1 1 0 102 0V8a1 1 0 00-1-1z"
                clipRule="evenodd"
              />
            </svg>
          </button>
        </div>

        <p className="mt-2 text-sm text-white/70 line-clamp-2">
          {entry.source_text.length > 100
            ? entry.source_text.slice(0, 100) + '...'
            : entry.source_text}
        </p>

        <AnimatePresence>
          {expanded && (
            <motion.div
              initial={{ height: 0, opacity: 0 }}
              animate={{ height: 'auto', opacity: 1 }}
              exit={{ height: 0, opacity: 0 }}
              className="mt-2 overflow-hidden"
            >
              <div className="rounded-lg bg-white/5 p-2">
                <p className="text-xs text-white/50">译文</p>
                <p className="mt-1 text-sm text-white/80">{entry.translated_text}</p>
              </div>
            </motion.div>
          )}
        </AnimatePresence>
      </div>
    </motion.div>
  );
}

function HistoryPanelContent({ onClose }: { onClose: () => void }) {
  const [entries, setEntries] = useState<TranslationRecord[]>([]);
  const [loading, setLoading] = useState(true);
  const [clearing, setClearing] = useState(false);
  const [searchQuery, setSearchQuery] = useState('');
  const [searching, setSearching] = useState(false);
  const debounceRef = useRef<ReturnType<typeof setTimeout> | undefined>(undefined);

  function loadSearchResults(query: string) {
    if (!query.trim()) {
      loadHistory();
      return;
    }
    setSearching(true);
    invoke<TranslationRecord[]>('search_history', { query: query.trim() })
      .then((result) => {
        setEntries(result);
      })
      .catch((error) => {
        console.error('Failed to search history:', error);
      })
      .finally(() => {
        setSearching(false);
        setLoading(false);
      });
  }

  function loadHistory() {
    setLoading(true);
    invoke<TranslationRecord[]>('get_history', {
      limit: 200,
      offset: 0,
    })
      .then((result) => {
        setEntries(result);
      })
      .catch((error) => {
        console.error('Failed to load history:', error);
      })
      .finally(() => {
        setLoading(false);
        setSearching(false);
      });
  }

  useEffect(() => {
    loadHistory();
  }, []);

  useEffect(() => {
    clearTimeout(debounceRef.current);
    if (searchQuery.trim() === '') {
      loadHistory();
      return;
    }
    debounceRef.current = setTimeout(() => {
      loadSearchResults(searchQuery);
    }, 300);
    return () => {
      clearTimeout(debounceRef.current);
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [searchQuery]);

  const handleDelete = async (id: string) => {
    try {
      await invoke<string>('delete_history_entry', { id });
      setEntries((prev) => prev.filter((e) => e.id !== id));
    } catch (error) {
      console.error('Failed to delete entry:', error);
    }
  };

  const handleClearAll = async () => {
    if (entries.length === 0) return;
    setClearing(true);
    try {
      await invoke<string>('clear_history');
      setEntries([]);
    } catch (error) {
      console.error('Failed to clear history:', error);
    } finally {
      setClearing(false);
    }
  };

  const handleExport = async () => {
    try {
      const data = await invoke<string>('export_history');
      const blob = new Blob([data], { type: 'application/json' });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = `visp_history_${new Date().toISOString().slice(0, 10)}.json`;
      document.body.appendChild(a);
      a.click();
      document.body.removeChild(a);
      URL.revokeObjectURL(url);
    } catch (error) {
      console.error('Export failed:', error);
    }
  };

  const groups = groupByDate(entries);

  return (
    <div className="flex h-screen flex-col bg-[#0b1220]">
      <header className="flex items-center justify-between border-b border-white/10 px-6 py-4">
        <div className="flex items-center gap-3">
          <button
            onClick={onClose}
            className="rounded-xl border border-white/10 bg-white/5 px-3 py-2 text-sm text-white/70 transition-colors hover:bg-white/10 hover:text-white"
          >
            返回
          </button>
          <h2 className="text-lg font-semibold text-white">翻译历史</h2>
          {entries.length > 0 && (
            <span className="rounded-full bg-white/10 px-2 py-0.5 text-xs text-white/50">
              {entries.length} 条
            </span>
          )}
        </div>
        <div className="flex items-center gap-2">
          {entries.length > 0 && (
            <button
              onClick={handleExport}
              className="rounded-xl border border-white/10 bg-white/5 px-3 py-2 text-xs text-white/60 transition-colors hover:bg-white/10 hover:text-white"
              title="导出为 JSON"
            >
              导出
            </button>
          )}
          {entries.length > 0 && (
            <button
              onClick={handleClearAll}
              disabled={clearing}
              className="rounded-xl border border-rose-500/20 bg-rose-500/10 px-3 py-2 text-xs text-rose-300 transition-colors hover:bg-rose-500/20 disabled:opacity-50"
            >
              {clearing ? '清空中...' : '清空历史'}
            </button>
          )}
        </div>
      </header>

      <div className="px-6 py-3">
        <div className="relative">
          <svg
            className="pointer-events-none absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-white/30"
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 20 20"
            fill="currentColor"
          >
            <path
              fillRule="evenodd"
              d="M9 3.5a5.5 5.5 0 100 11 5.5 5.5 0 000-11zM8 2a6.5 6.5 0 115.573 10.027 1 1 0 00-.827.85l-2.43 6.42a1 1 0 01-1.801-.288l2.43-6.42A1 1 0 0010 12.748 6.5 6.5 0 018 2z"
              clipRule="evenodd"
            />
          </svg>
          <input
            type="search"
            placeholder="搜索翻译历史..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            className="w-full rounded-xl border border-white/10 bg-white/[0.05] py-2.5 pl-10 pr-4 text-sm text-white/80 placeholder:text-white/30 outline-none transition-colors focus:border-white/20 focus:bg-white/[0.08]"
          />
        </div>
      </div>

      <main className="flex-1 overflow-y-auto px-6 py-4">
        {loading ? (
          <div className="flex items-center justify-center py-20">
            <div className="text-sm text-white/40">
              {searchQuery ? '搜索中...' : '加载中...'}
            </div>
          </div>
        ) : searching ? (
          <div className="flex items-center justify-center py-20">
            <div className="text-sm text-white/40">搜索中...</div>
          </div>
        ) : groups.size === 0 ? (searchQuery.trim() ? (
          <div className="flex flex-col items-center justify-center py-20">
            <div className="text-4xl mb-3">🔍</div>
            <div className="text-sm text-white/40">未找到匹配的结果</div>
            <div className="mt-1 text-xs text-white/30">尝试使用不同的关键词</div>
          </div>
        ) : (
          <div className="flex flex-col items-center justify-center py-20">
            <div className="text-4xl mb-3">📝</div>
            <div className="text-sm text-white/40">还没有翻译记录</div>
            <div className="mt-1 text-xs text-white/30">使用快捷键开始翻译</div>
          </div>
        )) : (
          <div className="space-y-6 max-w-2xl mx-auto">
            {Array.from(groups.entries()).map(([date, dateEntries]) => (
              <div key={date}>
                <h3 className="mb-2 text-xs font-medium uppercase tracking-wider text-white/30">
                  {date}
                </h3>
                <div className="space-y-2">
                  {dateEntries.map((entry) => (
                    <HistoryEntryCard
                      key={entry.id}
                      entry={entry}
                      onDelete={handleDelete}
                    />
                  ))}
                </div>
              </div>
            ))}
          </div>
        )}
      </main>
    </div>
  );
}

export function HistoryPanel({
  visible,
  onClose,
}: {
  visible: boolean;
  onClose: () => void;
}) {
  return (
    <AnimatePresence>
      {visible && (
        <motion.div
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          exit={{ opacity: 0 }}
          transition={{ duration: 0.2 }}
          className="fixed inset-0 z-[100]"
        >
          <HistoryPanelContent onClose={onClose} />
        </motion.div>
      )}
    </AnimatePresence>
  );
}
