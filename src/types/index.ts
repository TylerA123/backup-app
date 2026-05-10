export interface Project {
  id: string
  name: string
  local_path: string
  last_synced_at: string | null
  server_id: string | null
  is_deleted: boolean
}

export interface Snapshot {
  id: string
  project_id: string
  fingerprint: string
  file_count: number
  total_size: number
  trigger: 'auto' | 'manual' | 'periodic'
  synced: boolean
  created_at: string
}

export interface UploadQueueItem {
  id: number
  project_id: string
  relative_path: string
  content_hash: string
  file_size: number
  status: 'pending' | 'uploading' | 'completed' | 'failed'
  retry_count: number
  last_error: string | null
}

export interface SyncStatus {
  connected: boolean
  pending_uploads: number
  last_sync: string | null
  is_syncing: boolean
}

export interface AppSettings {
  debounce_ms: number
  max_concurrent_uploads: number
  chunk_size_mb: number
  snapshot_interval_minutes: number
}
