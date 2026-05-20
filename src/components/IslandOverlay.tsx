import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { useEffect, useState } from 'react';
import { Island } from './Island';

type OverlayPhase = 'idle' | 'recording' | 'thinking';

interface OverlayState {
  visible: boolean;
  phase: OverlayPhase;
  title: string;
  detail: string;
}

const defaultState: OverlayState = {
  visible: false,
  phase: 'idle',
  title: '准备就绪',
  detail: '等待语音快捷键',
};

export function IslandOverlay() {
  const [state, setState] = useState<OverlayState>(defaultState);

  useEffect(() => {
    const unlistenUpdate = listen<OverlayState>('island:update', (event) => {
      setState(event.payload);
    });

    const unlistenHide = listen('island:hide', () => {
      setState(defaultState);
    });

    const unlistenFocus = listen('island:focus-main', () => {
      void invoke('show_main_window');
    });

    return () => {
      unlistenUpdate.then((fn) => fn());
      unlistenHide.then((fn) => fn());
      unlistenFocus.then((fn) => fn());
    };
  }, []);

  return (
    <div className="h-screen w-screen bg-transparent">
      <div className="flex h-full items-end justify-center px-4 pb-4">
        <Island
          visible={state.visible}
          phase={state.phase}
          title={state.title}
          detail={state.detail}
        />
      </div>
    </div>
  );
}
