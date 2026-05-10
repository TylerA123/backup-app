import type { Project } from '../../types'
import { formatTimeAgo } from '../../utils/formatters'

interface Props {
  project: Project
  isSelected: boolean
  onClick: () => void
}

export function ProjectCard({ project, isSelected, onClick }: Props) {
  return (
    <button
      onClick={onClick}
      className={`w-full text-left px-3 py-2 rounded-md text-sm transition-colors cursor-pointer ${
        isSelected
          ? 'bg-backup/20 text-accent'
          : 'text-text hover:bg-surface/50'
      }`}
    >
      <div className="font-medium truncate">{project.name}</div>
      {project.last_synced_at && (
        <div className="text-xs text-text-muted mt-0.5">
          {formatTimeAgo(project.last_synced_at)}
        </div>
      )}
    </button>
  )
}
