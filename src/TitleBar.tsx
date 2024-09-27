import { appWindow } from '@tauri-apps/api/window';
import { X, Minus, Square } from 'lucide-react';

const TitleBar = () => {
  const handleClose = () => appWindow.close();
  const handleMinimize = () => appWindow.minimize();
  const handleToggleMaximize = () => appWindow.toggleMaximize();

  return (
    <div className="flex justify-between items-center bg-gray-800 text-white h-8 px-2" data-tauri-drag-region>
      <div className="flex items-center">
        <span className="text-sm font-semibold">VRClipboard-IME</span>
        <span className="text-xs font-semibold ml-2">v1.9.0</span>
      </div>
      <div className="flex">
        <button onClick={handleMinimize} className="p-1 hover:bg-gray-700 focus:outline-none">
          <Minus size={16} />
        </button>
        <button onClick={handleToggleMaximize} className="p-1 hover:bg-gray-700 focus:outline-none">
          <Square size={16} />
        </button>
        <button onClick={handleClose} className="p-1 hover:bg-red-600 focus:outline-none">
          <X size={16} />
        </button>
      </div>
    </div>
  );
};

export default TitleBar;