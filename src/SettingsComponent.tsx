import React, { useState, useEffect, useRef } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { ChevronDown } from 'lucide-react';

interface Config {
  prefix: string;
  split: string;
  command: string;
  ignore_prefix: boolean;
  on_copy_mode: OnCopyMode;
  skip_url: boolean; 
  use_tsf_reconvert: boolean;
  skip_on_out_of_vrc: boolean;
}

enum OnCopyMode {
  ReturnToClipboard = 'ReturnToClipboard',
  ReturnToChatbox = 'ReturnToChatbox',
  SendDirectly = 'SendDirectly'
}

const SettingsComponent = () => {
  const [settings, setSettings] = useState<Config>({
    prefix: ';',
    split: '/',
    command: ';',
    ignore_prefix: false,
    on_copy_mode: OnCopyMode.ReturnToChatbox,
    skip_url: true,
    use_tsf_reconvert: false,
    skip_on_out_of_vrc: true,
  });
  const [isOpen, setIsOpen] = useState(false);
  const dropdownRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    loadSettings();
    document.addEventListener('mousedown', handleClickOutside);
    return () => {
      document.removeEventListener('mousedown', handleClickOutside);
    };
  }, []);

  const loadSettings = async () => {
    try {
      const loadedSettings: Config = await invoke('load_settings');
      setSettings(loadedSettings);
    } catch (error) {
      console.error('Failed to load settings:', error);
    }
  };

  const saveSettings = async (newSettings: Config) => {
    try {
      await invoke('save_settings', { config: newSettings });
      alert('設定が正常に保存されました。');
    } catch (error) {
      console.error('Failed to save settings:', error);
      alert('設定の保存に失敗しました。もう一度お試しください。');
    }
  };

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value, type, checked } = e.target;
    const newSettings = {
      ...settings,
      [name]: type === 'checkbox' ? checked : value
    };
    setSettings(newSettings);
    saveSettings(newSettings);
  };

  const handleSelectChange = (value: OnCopyMode) => {
    const newSettings = { ...settings, on_copy_mode: value };
    setSettings(newSettings);
    setIsOpen(false);
    saveSettings(newSettings);
  };

  const handleClickOutside = (event: MouseEvent) => {
    if (dropdownRef.current && !dropdownRef.current.contains(event.target as Node)) {
      setIsOpen(false);
    }
  };

  const getOnCopyModeLabel = (mode: OnCopyMode) => {
    switch (mode) {
      case OnCopyMode.ReturnToClipboard:
        return 'クリップボードへ送信';
      case OnCopyMode.ReturnToChatbox:
        return 'チャットボックスへ送信';
      case OnCopyMode.SendDirectly:
        return '直接チャットへ送信';
    }
  };

  return (
    <div>
      <h2 className="text-lg font-semibold mb-2">設定</h2>
      <div className="bg-white p-3 rounded-md shadow-inner h-[calc(100vh-100px)] overflow-y-auto">
        <div className="space-y-4">
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              区切り文字
            </label>
            <input
              type="text"
              name="split"
              value={settings.split}
              onChange={handleChange}
              className="w-full p-2 border rounded-md"
            />
          </div>
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              モード変更文字
            </label>
            <input
              type="text"
              name="command"
              value={settings.command}
              onChange={handleChange}
              className="w-full p-2 border rounded-md"
            />
          </div>
          <div className="flex items-center">
            <input
              type="checkbox"
              id="ignore_prefix"
              name="ignore_prefix"
              checked={settings.ignore_prefix}
              onChange={handleChange}
              className="mr-2"
            />
            <label htmlFor="ignore_prefix" className="text-sm font-medium text-gray-700">
              無条件で変換
            </label>
          </div>
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              開始文字
            </label>
            <input
              type="text"
              name="prefix"
              value={settings.prefix}
              onChange={handleChange}
              className={`w-full p-2 border rounded-md ${settings.ignore_prefix ? 'bg-gray-100' : ''}`}
              disabled={settings.ignore_prefix}
            />
          </div>
          <div className="flex items-center">
            <input
              type="checkbox"
              id="skip_url"
              name="skip_url"
              checked={settings.skip_url}
              onChange={handleChange}
              className="mr-2"
            />
            <label htmlFor="skip_url" className="text-sm font-medium text-gray-700">
              URL が含まれている文章をスキップ
            </label>
          </div>
          <div className="relative" ref={dropdownRef}>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              コピー時の動作
            </label>
            <div
              className="w-full p-2 border rounded-md bg-white flex justify-between items-center cursor-pointer"
              onClick={() => setIsOpen(!isOpen)}
            >
              <span>{getOnCopyModeLabel(settings.on_copy_mode)}</span>
              <ChevronDown className={`transition-transform duration-200 ${isOpen ? 'transform rotate-180' : ''}`} />
            </div>
            {isOpen && (
              <div className="absolute z-10 mt-1 w-full bg-white border border-gray-300 rounded-md shadow-lg">
                {Object.values(OnCopyMode).map((mode) => (
                  <div
                    key={mode}
                    className="p-2 hover:bg-gray-100 cursor-pointer"
                    onClick={() => handleSelectChange(mode)}
                  >
                    {getOnCopyModeLabel(mode)}
                  </div>
                ))}
              </div>
            )}
          </div>
          <div className="flex items-center">
            <input
              type="checkbox"
              id="use_tsf_reconvert"
              name="use_tsf_reconvert"
              checked={settings.use_tsf_reconvert}
              onChange={handleChange}
              className="mr-2"
            />
            <label htmlFor="use_tsf_reconvert" className="text-sm font-medium text-gray-700">
              ベータ機能: Text Services Framework 再変換を使用（区切り、モード変更、開始文字が無効化されます）<br />
              Windows10または11を使用している場合は、「以前のバージョンの Microsoft IME を使う」を有効化する必要があります。
            </label>
          </div>
          <div className="flex items-center">
            <input
              type="checkbox"
              id="skip_on_out_of_vrc"
              name="skip_on_out_of_vrc"
              checked={settings.skip_on_out_of_vrc}
              onChange={handleChange}
              className="mr-2"
            />
            <label htmlFor="skip_on_out_of_vrc" className="text-sm font-medium text-gray-700">
              VRChat以外からのコピーをスキップ
            </label>
          </div>
        </div>
      </div>
    </div>
  );
};

export default SettingsComponent;