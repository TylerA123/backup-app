import { create } from 'zustand'
import type { Project, SyncStatus } from '../types'

interface AppState {
  projects: Project[]
  selectedProjectId: string | null
  syncStatus: SyncStatus
  isOnboarding: boolean

  setProjects: (projects: Project[]) => void
  addProject: (project: Project) => void
  removeProject: (id: string) => void
  selectProject: (id: string | null) => void
  setSyncStatus: (status: Partial<SyncStatus>) => void
  setOnboarding: (value: boolean) => void
}

export const useAppStore = create<AppState>((set) => ({
  projects: [],
  selectedProjectId: null,
  syncStatus: {
    connected: false,
    pending_uploads: 0,
    last_sync: null,
    is_syncing: false,
  },
  isOnboarding: true,

  setProjects: (projects) => set({ projects }),
  addProject: (project) =>
    set((state) => ({ projects: [...state.projects, project] })),
  removeProject: (id) =>
    set((state) => ({
      projects: state.projects.filter((p) => p.id !== id),
    })),
  selectProject: (id) => set({ selectedProjectId: id }),
  setSyncStatus: (status) =>
    set((state) => ({
      syncStatus: { ...state.syncStatus, ...status },
    })),
  setOnboarding: (value) => set({ isOnboarding: value }),
}))
