import { useAppStore } from './stores/appStore'
import { useProjects } from './hooks/useProjects'
import { useSyncStatus } from './hooks/useSyncStatus'
import { AppShell } from './components/Layout/AppShell'
import { AuthScreen } from './components/Auth/AuthScreen'
import { VersionHistory } from './components/VersionHistory/VersionHistory'
import { SettingsPanel } from './components/Settings/SettingsPanel'
import { FileAudio, FolderOpen } from 'lucide-react'

function EmptyState() {
  return (
    <div className="flex flex-col items-center justify-center h-full text-text-muted">
      <FileAudio size={48} className="mb-4 opacity-50" />
      <h2 className="text-lg font-medium mb-2">No project selected</h2>
      <p className="text-sm">Select a project from the sidebar or add a new one</p>
    </div>
  )
}

function ProjectView() {
  return (
    <div className="space-y-6">
      <div className="flex items-center gap-3">
        <FolderOpen size={24} className="text-accent" />
        <div>
          <h2 className="text-lg font-semibold text-text">Project Overview</h2>
          <p className="text-sm text-text-muted">
            Snapshots and version history appear below
          </p>
        </div>
      </div>

      <div className="grid grid-cols-3 gap-4">
        <div className="bg-surface-alt rounded-md p-4 border border-border">
          <div className="text-2xl font-bold text-accent">0</div>
          <div className="text-xs text-text-muted mt-1">Snapshots</div>
        </div>
        <div className="bg-surface-alt rounded-md p-4 border border-border">
          <div className="text-2xl font-bold text-success">0</div>
          <div className="text-xs text-text-muted mt-1">Files Tracked</div>
        </div>
        <div className="bg-surface-alt rounded-md p-4 border border-border">
          <div className="text-2xl font-bold text-warning">0 B</div>
          <div className="text-xs text-text-muted mt-1">Total Size</div>
        </div>
      </div>

      <VersionHistory snapshots={[]} onRestore={() => {}} />
      <SettingsPanel />
    </div>
  )
}

export default function App() {
  const { isAuthenticated, selectedProjectId } = useAppStore()

  useProjects()
  useSyncStatus()

  if (!isAuthenticated) {
    return <AuthScreen />
  }

  return (
    <AppShell>
      {selectedProjectId ? <ProjectView /> : <EmptyState />}
    </AppShell>
  )
}
