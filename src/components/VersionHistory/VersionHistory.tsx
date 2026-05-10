import { History, RotateCcw } from 'lucide-react'
import { formatTimeAgo } from '../../utils/formatters'

interface VersionEntry {
  id: string
  created_at: string
  file_count: number
  total_size: number
  trigger: string
  synced: boolean
}

interface Props {
  snapshots: VersionEntry[]
  onRestore: (snapshotId: string) => void
}

export function VersionHistory({ snapshots, onRestore }: Props) {
  if (snapshots.length === 0) {
    return (
      <div className="text-center text-text-muted py-12">
        <History size={32} className="mx-auto mb-3 opacity-50" />
        <p className="text-sm">No snapshots yet</p>
        <p className="text-xs mt-1">Changes will appear here automatically</p>
      </div>
    )
  }

  return (
    <div className="space-y-2">
      <h2 className="text-sm font-semibold text-text-muted uppercase tracking-wider mb-3">
        Version History
      </h2>
      {snapshots.map((s) => (
        <div
          key={s.id}
          className="flex items-center justify-between p-3 bg-surface-alt rounded-md border border-border"
        >
          <div>
            <div className="text-sm text-text">{formatTimeAgo(s.created_at)}</div>
            <div className="text-xs text-text-muted mt-0.5">
              {s.file_count} files &middot; {formatFileSize(s.total_size)} &middot;
              {s.trigger === 'manual' ? ' manual' : ' auto'}
            </div>
          </div>
          <button
            onClick={() => onRestore(s.id)}
            className="flex items-center gap-1 px-3 py-1.5 text-xs text-accent hover:bg-accent/10 rounded-md transition-colors cursor-pointer"
          >
            <RotateCcw size={14} />
            Restore
          </button>
        </div>
      ))}
    </div>
  )
}

function formatFileSize(bytes: number): string {
  if (bytes === 0) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return `${parseFloat((bytes / Math.pow(k, i)).toFixed(1))} ${sizes[i]}`
}
