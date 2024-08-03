import { useState, useEffect } from "react";
import { List, Settings } from 'lucide-react';
import { listen } from "@tauri-apps/api/event";
import "./App.css";
import TitleBar from "./TitleBar";
import SettingsComponent from "./SettingsComponent";

interface Log {
  time: string;
  original: string;
  converted: string;
}

function App() {
  const [logs, setLogs] = useState<Log[]>([]);
  const [activeMenuItem, setActiveMenuItem] = useState('home');

  useEffect(() => {
    const unlisten = listen<Log>('addLog', (event) => {
      setLogs(prevLogs => [{ time: event.payload.time, original: event.payload.original, converted: event.payload.converted }, ...prevLogs]);
    });

    return () => {
      unlisten.then(f => f());
    }
  }, []);

  const renderLogEntry = (log: { time: string; original: string; converted: string }, index: number) => (
    <div key={index} className="text-sm mb-1">
      <span className="text-blue-600 font-medium">{log.time}</span>:
      <span className="text-red-500 ml-2">{log.original}</span>
      <span className="text-gray-500 mx-1">→</span>
      <span className="text-green-600">{log.converted}</span>
    </div>
  );

  const renderContent = () => {
    switch (activeMenuItem) {
      case 'home':
        return (
          <div>
            <h2 className="text-lg font-semibold mb-2">変換ログ</h2>
            <div className="bg-white p-3 rounded-md shadow-inner h-[calc(100vh-100px)] overflow-y-auto">
              {logs.map((log, index) => renderLogEntry(log, index))}
            </div>
          </div>
        );
      case 'settings':
        return <SettingsComponent />;
      default:
        return null;
    }
  };

  return (
    <div>
      <TitleBar />
      <div className="flex h-[calc(100vh-32px)] bg-gray-100">
        {/* サイドメニュー */}
        <div className="w-16 bg-gray-800 text-white p-4" data-tauri-drag-region>
          <div className="flex flex-col items-center space-y-4">
            <button onClick={() => setActiveMenuItem('home')} className={`p-2 rounded ${activeMenuItem === 'home' ? 'bg-gray-600' : ''}`}>
              <List size={24} />
            </button>
            <button onClick={() => setActiveMenuItem('settings')} className={`p-2 rounded ${activeMenuItem === 'settings' ? 'bg-gray-600' : ''}`}>
              <Settings size={24} />
            </button>
          </div>
        </div>

        {/* メインコンテンツ */}
        <div className="flex-1 p-4 overflow-y-auto">
          {renderContent()}
        </div>
      </div>
    </div>
  );
};

export default App;
