import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App';
import { IslandOverlay } from './components/IslandOverlay';
import './index.css';

const isIsland = getCurrentWebviewWindow().label === 'island';

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    {isIsland ? <IslandOverlay /> : <App />}
  </React.StrictMode>,
);
