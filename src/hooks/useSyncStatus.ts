import { useEffect, useCallback } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { useAppStore } from '../stores/appStore'

export function useSyncStatus() {
  const { setSyncStatus } = useAppStore()

  const checkStatus = useCallback(async () => {
    try {
      const pending = await invoke<number>('get_pending_upload_count')
      setSyncStatus({ pending_uploads: pending })
    } catch {
      setSyncStatus({ connected: false })
    }
  }, [setSyncStatus])

  useEffect(() => {
    checkStatus()
    const interval = setInterval(checkStatus, 5000)
    return () => clearInterval(interval)
  }, [checkStatus])
}
