import { useEffect, useCallback } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { useAppStore } from '../stores/appStore'
import type { Project } from '../types'

export function useProjects() {
  const { projects, setProjects, addProject, setSyncStatus } = useAppStore()

  const loadProjects = useCallback(async () => {
    try {
      const result = await invoke<Project[]>('list_projects')
      setProjects(result)
    } catch (err) {
      console.error('Failed to load projects:', err)
    }
  }, [setProjects])

  useEffect(() => {
    loadProjects()
  }, [loadProjects])

  const createProject = useCallback(
    async (name: string, localPath: string) => {
      const id = crypto.randomUUID()
      try {
        await invoke('add_project', { id, name, localPath })
        addProject({ id, name, local_path: localPath, last_synced_at: null, server_id: null, is_deleted: false })
        // Trigger initial snapshot
        await invoke('trigger_snapshot', { projectId: id, projectPath: localPath })
        setSyncStatus({ is_syncing: true })
      } catch (err) {
        console.error('Failed to create project:', err)
      }
    },
    [addProject, setSyncStatus],
  )

  return { projects, loadProjects, createProject }
}
