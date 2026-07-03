import { cn } from '@/lib/utils'

interface RecordingIndicatorProps {
  isRecording: boolean
}

export function RecordingIndicator({ isRecording }: RecordingIndicatorProps) {
  if (!isRecording) return null

  return (
    <div className="flex items-center gap-2 px-3 py-1.5 rounded-full bg-red-500/10 border border-red-500/20">
      <div className="relative">
        <div className="w-2 h-2 rounded-full bg-red-500" />
        <div className="absolute inset-0 w-2 h-2 rounded-full bg-red-500 animate-ping" />
      </div>
      <span className="text-sm font-medium text-red-500">Recording</span>
    </div>
  )
}

export function RecordingOverlay({ isRecording }: RecordingIndicatorProps) {
  if (!isRecording) return null

  return (
    <div className="fixed inset-0 pointer-events-none flex items-center justify-center bg-black/20 backdrop-blur-sm z-50">
      <div className="flex flex-col items-center gap-4 p-8 rounded-2xl bg-background/90 border shadow-2xl">
        <div className="relative">
          <div className="w-16 h-16 rounded-full bg-red-500 flex items-center justify-center">
            <div className="w-8 h-8 rounded-full bg-white" />
          </div>
          <div className="absolute inset-0 w-16 h-16 rounded-full bg-red-500/50 animate-ping" />
        </div>
        <div className="text-center">
          <p className="text-lg font-semibold">Recording...</p>
          <p className="text-sm text-muted-foreground">
            Press hotkey again to stop
          </p>
        </div>
        <EQVisualizer />
      </div>
    </div>
  )
}

function EQVisualizer() {
  const bars = Array.from({ length: 16 }, (_, i) => i)

  return (
    <div className="flex items-end justify-center gap-1 h-12">
      {bars.map((i) => (
        <div
          key={i}
          className={cn(
            "w-1 bg-primary rounded-full transition-all duration-150",
            "animate-pulse"
          )}
          style={{
            height: `${Math.random() * 100}%`,
            animationDelay: `${i * 50}ms`,
          }}
        />
      ))}
    </div>
  )
}
