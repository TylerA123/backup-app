import { useState, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { Settings } from 'lucide-react'

export function SettingsPanel() {
  const [debounce, setDebounce] = useState('30000')
  const [maxUploads, setMaxUploads] = useState('3')
  const [chunkSize, setChunkSize] = useState('5')
  const [snapshotInterval, setSnapshotInterval] = useState('60')

  useEffect(() => {
    const load = async () => {
      try {
        setDebounce((await invoke('get_setting', { key: 'debounce_ms' })) || '30000')
        setMaxUploads((await invoke('get_setting', { key: 'max_concurrent_uploads' })) || '3')
        setChunkSize((await invoke('get_setting', { key: 'chunk_size_mb' })) || '5')
        setSnapshotInterval((await invoke('get_setting', { key: 'snapshot_interval_minutes' })) || '60')
      } catch {}
    }
    load()
  }, [])

  const save = async (key: string, value: string) => {
    try {
      await invoke('set_setting', { key, value })
    } catch {}
  }

  return (
    <div className="max-w-md">
      <h2 className="text-sm font-semibold text-text-muted uppercase tracking-wider mb-4 flex items-center gap-2">
        <Settings size={14} /> Settings
      </h2>
      <div className="space-y-4">
        <div>
          <label className="block text-sm text-text-muted mb-1">Debounce (ms)</label>
          <input
            type="number"
            value={debounce}
            onChange={(e) => { setDebounce(e.target.value); save('debounce_ms', e.target.value) }}
            className="w-full px-3 py-2 bg-surface-alt border border-border rounded-md text-text text-sm"
          />
        </div>
        <div>
          <label className="block text-sm text-text-muted mb-1">Max Concurrent Uploads</label>
          <input
            type="number"
            value={maxUploads}
            onChange={(e) => { setMaxUploads(e.target.value); save('max_concurrent_uploads', e.target.value) }}
            className="w-full px-3 py-2 bg-surface-alt border border-border rounded-md text-text text-sm"
          />
        </div>
        <div>
          <label className="block text-sm text-text-muted mb-1">Chunk Size (MB)</label>
          <input
            type="number"
            value={chunkSize}
            onChange={(e) => { setChunkSize(e.target.value); save('chunk_size_mb', e.target.value) }}
            className="w-full px-3 py-2 bg-surface-alt border border-border rounded-md text-text text-sm"
          />
        </div>
        <div>
          <label className="block text-sm text-text-muted mb-1">Snapshot Interval (min)</label>
          <input
            type="number"
            value={snapshotInterval}
            onChange={(e) => { setSnapshotInterval(e.target.value); save('snapshot_interval_minutes', e.target.value) }}
            className="w-full px-3 py-2 bg-surface-alt border border-border rounded-md text-text text-sm"
          />
        </div>
      </div>
    </div>
  )
}
