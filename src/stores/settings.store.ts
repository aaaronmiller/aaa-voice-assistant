import { create } from 'zustand'
import { invoke } from '@tauri-apps/api/core'

interface Settings {
  stt: {
    provider: string
    fallback_providers: string[]
    language: string
    streaming: boolean
  }
  tts: {
    provider: string
    voice: string
    volume: number
    speed: number
  }
  llm: {
    provider: string
    model: string
    temperature: number
    max_tokens: number
  }
  hotkeys: {
    ptt: string
    paste_last: string
    toggle_wake_word: string
    toggle_cleanup: string
    open_history: string
    open_settings: string
    cancel_recording: string
  }
  audio: {
    input_device: string
    output_device: string
    noise_suppression: boolean
    auto_gain_control: boolean
    silence_threshold: number
  }
  wake_word: string
  wake_word_enabled: boolean
  transcription_mode: string
  auto_capitalization: boolean
  auto_punctuation: boolean
  transcription_cleanup: boolean
  cleanup_prompt: string
  history_retention: number
  theme: string
}

const defaultSettings: Settings = {
  stt: {
    provider: 'whisper.cpp',
    fallback_providers: ['vosk'],
    language: 'en-US',
    streaming: false,
  },
  tts: {
    provider: 'piper',
    voice: 'en_US-lessac-medium',
    volume: 1.0,
    speed: 1.0,
  },
  llm: {
    provider: 'gemini',
    model: 'gemini-pro',
    temperature: 0.7,
    max_tokens: 2048,
  },
  hotkeys: {
    ptt: 'F1',
    paste_last: 'Ctrl+Shift+V',
    toggle_wake_word: 'Ctrl+Shift+W',
    toggle_cleanup: 'Ctrl+Shift+C',
    open_history: 'Ctrl+Shift+H',
    open_settings: 'Ctrl+Shift+S',
    cancel_recording: 'Esc',
  },
  audio: {
    input_device: 'default',
    output_device: 'default',
    noise_suppression: true,
    auto_gain_control: true,
    silence_threshold: 0.1,
  },
  wake_word: 'hey computer',
  wake_word_enabled: false,
  transcription_mode: 'insert',
  auto_capitalization: true,
  auto_punctuation: true,
  transcription_cleanup: false,
  cleanup_prompt: 'Fix grammar, punctuation, and capitalization',
  history_retention: 10000,
  theme: 'system',
}

interface SettingsStore {
  settings: Settings
  setSettings: (settings: Settings) => void
  updateSettings: (settings: Partial<Settings>) => void
  loadSettings: () => Promise<void>
  saveSettings: () => Promise<void>
}

export const useSettingsStore = create<SettingsStore>((set, get) => ({
  settings: defaultSettings,
  
  setSettings: (settings) => set({ settings }),
  
  updateSettings: (partial) => {
    const current = get().settings
    set({ settings: { ...current, ...partial } })
    get().saveSettings()
  },
  
  loadSettings: async () => {
    try {
      const settings = await invoke<Settings>('get_settings')
      set({ settings })
    } catch (error) {
      console.error('Failed to load settings:', error)
      set({ settings: defaultSettings })
    }
  },
  
  saveSettings: async () => {
    try {
      await invoke('update_settings', { settings: get().settings })
    } catch (error) {
      console.error('Failed to save settings:', error)
    }
  },
}))
