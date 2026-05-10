import { useAppStore } from '../../stores/appStore'
import { Wifi, WifiOff, CloudArrowUp, CheckCircle2, AlertCircle } from 'lucide-react'

export function GlobalStatusBar() {
  const { syncStatus } = useAppStore()

  return (
    <div className="h-8 bg-surface-alt border-t border-border flex items-center px-4 text-xs text-text-muted gap-4">
      <div className="flex items-center gap-1">
        {syncStatus.connected ? (
          <Wifi size={14} className="text-success" />
        ) : (
          <WifiOff size={14} className="text-error" />
        )}
        <span>{syncStatus.connected ? 'Connected' : 'Offline'}</span>
      </div>

      {syncStatus.is_syncing ? (
        <div className="flex items-center gap-1">
          <CloudArrowUp size={14} className="text-accent animate-pulse" />
          <span>Syncing...</span>
        </div>
      ) : syncStatus.pending_uploads > 0 ? (
        <div className="flex items-center gap-1">
          <AlertCircle size={14} className="text-warning" />
          <span>{syncStatus.pending_uploads} pending</span>
        </div>
      ) : (
        <div className="flex items-center gap-1">
          <CheckCircle2 size={14} className="text-success" />
          <span>Synced</span>
        </div>
      )}

      {syncStatus.last_sync && (
        <span className="ml-auto">
          Last sync: {new Date(syncStatus.last_sync).toLocaleTimeString()}
        </span>
      )}
    </div>
  )
}
