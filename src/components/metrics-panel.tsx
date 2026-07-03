import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Label } from '@/components/ui/label'

export function MetricsPanel() {
  return (
    <div className="space-y-6">
      <Card>
        <CardHeader>
          <CardTitle>Usage Statistics</CardTitle>
          <CardDescription>Your voice assistant usage metrics</CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="grid grid-cols-2 gap-4">
            <div className="space-y-2">
              <Label className="text-muted-foreground">Total Transcriptions</Label>
              <p className="text-2xl font-bold">0</p>
            </div>
            <div className="space-y-2">
              <Label className="text-muted-foreground">Total Duration</Label>
              <p className="text-2xl font-bold">0m</p>
            </div>
            <div className="space-y-2">
              <Label className="text-muted-foreground">Avg. Latency</Label>
              <p className="text-2xl font-bold">0ms</p>
            </div>
            <div className="space-y-2">
              <Label className="text-muted-foreground">Success Rate</Label>
              <p className="text-2xl font-bold">100%</p>
            </div>
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>Provider Performance</CardTitle>
          <CardDescription>Performance by provider</CardDescription>
        </CardHeader>
        <CardContent>
          <p className="text-muted-foreground text-sm">
            No data available yet. Start using the voice assistant to see performance metrics.
          </p>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>Resource Usage</CardTitle>
          <CardDescription>System resource monitoring</CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="grid grid-cols-3 gap-4">
            <div className="space-y-2">
              <Label className="text-muted-foreground">CPU</Label>
              <p className="text-xl font-bold">--</p>
            </div>
            <div className="space-y-2">
              <Label className="text-muted-foreground">Memory</Label>
              <p className="text-xl font-bold">--</p>
            </div>
            <div className="space-y-2">
              <Label className="text-muted-foreground">GPU</Label>
              <p className="text-xl font-bold">--</p>
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  )
}
