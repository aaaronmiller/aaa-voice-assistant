import { useState, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { SettingsPanel } from './components/settings-panel'
import { HistoryPanel } from './components/history-panel'
import { MetricsPanel } from './components/metrics-panel'
import { RecordingIndicator } from './components/recording-indicator'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { useSettingsStore } from './stores/settings.store'

export interface Settings {
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

function App() {
  const [isRecording] = useState(false)
  const [gpuBackend, setGpuBackend] = useState<string>('')
  const { settings, setSettings, loadSettings } = useSettingsStore()

  useEffect(() => {
    loadSettings()
    
    invoke<string>('get_gpu_backend').then(setGpuBackend).catch(console.error)
  }, [loadSettings])

  return (
    <div className="min-h-screen bg-background text-foreground">
      <div className="container mx-auto p-4">
        <header className="flex items-center justify-between mb-6">
          <h1 className="text-2xl font-bold">Voice Assistant</h1>
          <div className="flex items-center gap-4">
            <span className="text-sm text-muted-foreground">
              GPU: {gpuBackend}
            </span>
            <RecordingIndicator isRecording={isRecording} />
          </div>
        </header>

        <Tabs defaultValue="settings" className="w-full">
          <TabsList className="grid w-full grid-cols-3">
            <TabsTrigger value="settings">Settings</TabsTrigger>
            <TabsTrigger value="history">History</TabsTrigger>
            <TabsTrigger value="metrics">Metrics</TabsTrigger>
          </TabsList>
          
          <TabsContent value="settings">
            <SettingsPanel 
              settings={settings} 
              onSettingsChange={setSettings} 
            />
          </TabsContent>
          
          <TabsContent value="history">
            <HistoryPanel />
          </TabsContent>
          
          <TabsContent value="metrics">
            <MetricsPanel />
          </TabsContent>
        </Tabs>
      </div>
    </div>
  )
}

export default App
