import { create } from 'zustand'
import type { Project, SyncStatus } from '../types'

interface AuthUser {
  id: string
  email: string | null
}

interface AppState {
  // Auth
  user: AuthUser | null
  accessToken: string | null
  isAuthenticated: boolean

  // Projects
  projects: Project[]
  selectedProjectId: string | null
  syncStatus: SyncStatus
  isOnboarding: boolean

  // Auth actions
  setAuth: (user: AuthUser | null, token: string | null) => void
  setOnboarding: (value: boolean) => void

  // Project actions
  setProjects: (projects: Project[]) => void
  addProject: (project: Project) => void
  removeProject: (id: string) => void
  selectProject: (id: string | null) => void
  setSyncStatus: (status: Partial<SyncStatus>) => void
}

export const useAppStore = create<AppState>((set) => ({
  user: null,
  accessToken: null,
  isAuthenticated: false,
  projects: [],
  selectedProjectId: null,
  syncStatus: {
    connected: false,
    pending_uploads: 0,
    last_sync: null,
    is_syncing: false,
  },
  isOnboarding: true,

  setAuth: (user, accessToken) =>
    set({ user, accessToken, isAuthenticated: !!user }),

  setOnboarding: (value) => set({ isOnboarding: value }),

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
}))
