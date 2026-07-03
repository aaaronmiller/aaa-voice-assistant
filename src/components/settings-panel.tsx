import { useEffect, useState } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Label } from '@/components/ui/label'
import { Input } from '@/components/ui/input'
import { Switch } from '@/components/ui/switch'
import { Slider } from '@/components/ui/slider'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import type { Settings } from '../App'

interface SettingsPanelProps {
  settings: Settings
  onSettingsChange: (settings: Settings) => void
}

export function SettingsPanel({ settings, onSettingsChange }: SettingsPanelProps) {
  const [audioDevices, setAudioDevices] = useState<Array<{ id: string; name: string }>>([])

  useEffect(() => {
    invoke<Array<{ id: string; name: string }>>('list_audio_devices')
      .then(setAudioDevices)
      .catch(() => setAudioDevices([]))
  }, [])

  const updateSettings = (partial: Partial<Settings>) => {
    onSettingsChange({ ...settings, ...partial })
  }

  const updateSTT = (partial: Partial<Settings['stt']>) => {
    onSettingsChange({ ...settings, stt: { ...settings.stt, ...partial } })
  }

  const updateTTS = (partial: Partial<Settings['tts']>) => {
    onSettingsChange({ ...settings, tts: { ...settings.tts, ...partial } })
  }

  const updateLLM = (partial: Partial<Settings['llm']>) => {
    onSettingsChange({ ...settings, llm: { ...settings.llm, ...partial } })
  }

  const updateAudio = (partial: Partial<Settings['audio']>) => {
    onSettingsChange({ ...settings, audio: { ...settings.audio, ...partial } })
  }

  return (
    <div className="space-y-6">
      <Card>
        <CardHeader>
          <CardTitle>Speech-to-Text</CardTitle>
          <CardDescription>Configure your STT provider</CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="space-y-2">
            <Label>Provider</Label>
            <Select value={settings.stt.provider} onValueChange={(v) => updateSTT({ provider: v })}>
              <SelectTrigger>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="whisper.cpp">Whisper.cpp (Local)</SelectItem>
                <SelectItem value="vosk">Vosk (Local)</SelectItem>
                <SelectItem value="assemblyai">AssemblyAI (Cloud)</SelectItem>
                <SelectItem value="deepgram">Deepgram (Cloud)</SelectItem>
                <SelectItem value="openai">OpenAI Whisper (Cloud)</SelectItem>
              </SelectContent>
            </Select>
          </div>
          
          <div className="space-y-2">
            <Label>Language</Label>
            <Select value={settings.stt.language} onValueChange={(v) => updateSTT({ language: v })}>
              <SelectTrigger>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="en-US">English (US)</SelectItem>
                <SelectItem value="en-GB">English (UK)</SelectItem>
                <SelectItem value="es-ES">Spanish</SelectItem>
                <SelectItem value="fr-FR">French</SelectItem>
                <SelectItem value="de-DE">German</SelectItem>
                <SelectItem value="ja-JP">Japanese</SelectItem>
                <SelectItem value="zh-CN">Chinese (Simplified)</SelectItem>
              </SelectContent>
            </Select>
          </div>

          <div className="flex items-center justify-between">
            <Label htmlFor="streaming">Streaming Transcription</Label>
            <Switch
              id="streaming"
              checked={settings.stt.streaming}
              onCheckedChange={(v) => updateSTT({ streaming: v })}
            />
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>Text-to-Speech</CardTitle>
          <CardDescription>Configure your TTS provider</CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="space-y-2">
            <Label>Provider</Label>
            <Select value={settings.tts.provider} onValueChange={(v) => updateTTS({ provider: v })}>
              <SelectTrigger>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="piper">Piper (Local)</SelectItem>
                <SelectItem value="elevenlabs">ElevenLabs (Cloud)</SelectItem>
                <SelectItem value="openai">OpenAI TTS (Cloud)</SelectItem>
                <SelectItem value="inworld">Inworld AI (Cloud)</SelectItem>
              </SelectContent>
            </Select>
          </div>

          <div className="space-y-2">
            <Label>Speed: {settings.tts.speed.toFixed(1)}x</Label>
            <Slider
              value={[settings.tts.speed]}
              onValueChange={([v]) => updateTTS({ speed: v })}
              min={0.5}
              max={2}
              step={0.1}
            />
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>LLM Backend</CardTitle>
          <CardDescription>Configure your AI model provider</CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="space-y-2">
            <Label>Provider</Label>
            <Select value={settings.llm.provider} onValueChange={(v) => updateLLM({ provider: v })}>
              <SelectTrigger>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="gemini">Google Gemini</SelectItem>
                <SelectItem value="openrouter">OpenRouter</SelectItem>
                <SelectItem value="openai">OpenAI</SelectItem>
              </SelectContent>
            </Select>
          </div>

          <div className="space-y-2">
            <Label>Model</Label>
            <Input
              value={settings.llm.model}
              onChange={(e) => updateLLM({ model: e.target.value })}
              placeholder="gemini-pro"
            />
          </div>

          <div className="space-y-2">
            <Label>Temperature: {settings.llm.temperature}</Label>
            <Slider
              value={[settings.llm.temperature]}
              onValueChange={([v]) => updateLLM({ temperature: v })}
              min={0}
              max={2}
              step={0.1}
            />
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>Audio Settings</CardTitle>
          <CardDescription>Configure audio input/output</CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="space-y-2">
            <Label>Input Device</Label>
            <Select value={settings.audio.input_device} onValueChange={(v) => updateAudio({ input_device: v })}>
              <SelectTrigger>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="default">Default</SelectItem>
                {audioDevices.map((device) => (
                  <SelectItem key={device.id} value={device.id}>
                    {device.name}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>

          <div className="flex items-center justify-between">
            <Label htmlFor="noise">Noise Suppression</Label>
            <Switch
              id="noise"
              checked={settings.audio.noise_suppression}
              onCheckedChange={(v) => updateAudio({ noise_suppression: v })}
            />
          </div>

          <div className="flex items-center justify-between">
            <Label htmlFor="agc">Auto Gain Control</Label>
            <Switch
              id="agc"
              checked={settings.audio.auto_gain_control}
              onCheckedChange={(v) => updateAudio({ auto_gain_control: v })}
            />
          </div>

          <div className="space-y-2">
            <Label>Silence Threshold: {settings.audio.silence_threshold.toFixed(2)}</Label>
            <Slider
              value={[settings.audio.silence_threshold]}
              onValueChange={([v]) => updateAudio({ silence_threshold: v })}
              min={0}
              max={1}
              step={0.01}
            />
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>Transcription Options</CardTitle>
          <CardDescription>Text processing settings</CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="flex items-center justify-between">
            <Label htmlFor="auto-cap">Auto Capitalization</Label>
            <Switch
              id="auto-cap"
              checked={settings.auto_capitalization}
              onCheckedChange={(v) => updateSettings({ auto_capitalization: v })}
            />
          </div>

          <div className="flex items-center justify-between">
            <Label htmlFor="auto-punc">Auto Punctuation</Label>
            <Switch
              id="auto-punc"
              checked={settings.auto_punctuation}
              onCheckedChange={(v) => updateSettings({ auto_punctuation: v })}
            />
          </div>

          <div className="flex items-center justify-between">
            <Label htmlFor="cleanup">Transcription Cleanup</Label>
            <Switch
              id="cleanup"
              checked={settings.transcription_cleanup}
              onCheckedChange={(v) => updateSettings({ transcription_cleanup: v })}
            />
          </div>

          {settings.transcription_cleanup && (
            <div className="space-y-2">
              <Label>Cleanup Prompt</Label>
              <Input
                value={settings.cleanup_prompt}
                onChange={(e) => updateSettings({ cleanup_prompt: e.target.value })}
              />
            </div>
          )}
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>Wake Word</CardTitle>
          <CardDescription>Hands-free activation settings</CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="flex items-center justify-between">
            <Label htmlFor="wake-enabled">Enable Wake Word</Label>
            <Switch
              id="wake-enabled"
              checked={settings.wake_word_enabled}
              onCheckedChange={(v) => updateSettings({ wake_word_enabled: v })}
            />
          </div>

          <div className="space-y-2">
            <Label>Wake Phrase</Label>
            <Input
              value={settings.wake_word}
              onChange={(e) => updateSettings({ wake_word: e.target.value })}
              placeholder="hey computer"
            />
          </div>
        </CardContent>
      </Card>
    </div>
  )
}
