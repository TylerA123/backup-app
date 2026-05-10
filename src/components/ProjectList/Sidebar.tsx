import { useAppStore } from '../../stores/appStore'
import { ProjectCard } from './ProjectCard'
import { AddProjectDialog } from './AddProjectDialog'
import { useState } from 'react'

export function Sidebar() {
  const { projects, selectedProjectId, selectProject } = useAppStore()
  const [showAdd, setShowAdd] = useState(false)

  return (
    <aside className="w-64 bg-surface-alt border-r border-border flex flex-col">
      <div className="p-4 border-b border-border">
        <h1 className="text-lg font-bold text-accent">Studio Backup</h1>
      </div>

      <div className="flex-1 overflow-y-auto p-2 space-y-1">
        {projects.length === 0 && (
          <p className="text-sm text-text-muted text-center mt-8">
            No projects yet
          </p>
        )}
        {projects.map((p) => (
          <ProjectCard
            key={p.id}
            project={p}
            isSelected={p.id === selectedProjectId}
            onClick={() => selectProject(p.id)}
          />
        ))}
      </div>

      <div className="p-2 border-t border-border">
        <button
          onClick={() => setShowAdd(true)}
          className="w-full px-3 py-2 text-sm bg-backup hover:bg-backup-dark text-white rounded-md transition-colors cursor-pointer"
        >
          + Add Project
        </button>
      </div>

      {showAdd && <AddProjectDialog onClose={() => setShowAdd(false)} />}
    </aside>
  )
}
