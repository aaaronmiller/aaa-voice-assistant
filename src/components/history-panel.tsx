import { useEffect, useState } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Input } from '@/components/ui/input'
import { Button } from '@/components/ui/button'

interface HistoryEntry {
  id: string
  timestamp: string
  text: string
  provider: string
  confidence: number | null
  duration_ms: number
  mode: string
  language: string
}

export function HistoryPanel() {
  const [entries, setEntries] = useState<HistoryEntry[]>([])
  const [search, setSearch] = useState('')
  const [loading, setLoading] = useState(true)

  useEffect(() => {
    loadHistory()
  }, [])

  const loadHistory = async () => {
    setLoading(true)
    try {
      const history = await invoke<HistoryEntry[]>('get_history', { limit: 100 })
      setEntries(history)
    } catch (error) {
      console.error('Failed to load history:', error)
    }
    setLoading(false)
  }

  const handleClear = async () => {
    if (confirm('Are you sure you want to clear all history?')) {
      await invoke('clear_history')
      setEntries([])
    }
  }

  const filtered = search
    ? entries.filter((e) => e.text.toLowerCase().includes(search.toLowerCase()))
    : entries

  return (
    <Card>
      <CardHeader>
        <CardTitle>Transcription History</CardTitle>
        <CardDescription>View and search past transcriptions</CardDescription>
      </CardHeader>
      <CardContent className="space-y-4">
        <div className="flex gap-2">
          <Input
            placeholder="Search transcriptions..."
            value={search}
            onChange={(e) => setSearch(e.target.value)}
          />
          <Button variant="destructive" onClick={handleClear}>
            Clear All
          </Button>
        </div>

        {loading ? (
          <p className="text-muted-foreground">Loading...</p>
        ) : filtered.length === 0 ? (
          <p className="text-muted-foreground">No transcriptions found</p>
        ) : (
          <div className="space-y-2 max-h-[400px] overflow-y-auto">
            {filtered.map((entry) => (
              <div
                key={entry.id}
                className="p-3 rounded-lg border bg-muted/50 hover:bg-muted transition-colors"
              >
                <div className="flex justify-between text-sm text-muted-foreground mb-1">
                  <span>{new Date(entry.timestamp).toLocaleString()}</span>
                  <span>{entry.provider}</span>
                </div>
                <p className="text-sm">{entry.text}</p>
                {entry.confidence !== null && (
                  <p className="text-xs text-muted-foreground mt-1">
                    Confidence: {(entry.confidence * 100).toFixed(1)}%
                  </p>
                )}
              </div>
            ))}
          </div>
        )}
      </CardContent>
    </Card>
  )
}
