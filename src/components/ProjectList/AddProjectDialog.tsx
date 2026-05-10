import { useState } from 'react'
import { useProjects } from '../../hooks/useProjects'

interface Props {
  onClose: () => void
}

export function AddProjectDialog({ onClose }: Props) {
  const [name, setName] = useState('')
  const [path, setPath] = useState('')
  const { createProject } = useProjects()

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    if (!name || !path) return
    await createProject(name, path)
    onClose()
  }

  const handleBrowse = async () => {
    try {
      const { open } = await import('@tauri-apps/plugin-dialog')
      const selected = await open({ directory: true, multiple: false })
      if (selected) {
        setPath(selected)
        if (!name) {
          setName(selected.split(/[/\\]/).pop() || '')
        }
      }
    } catch {
      // Not available in dev mode, fallback to manual input
    }
  }

  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div className="bg-surface border border-border rounded-lg p-6 w-96 shadow-xl">
        <h2 className="text-lg font-semibold mb-4">Add Project</h2>
        <form onSubmit={handleSubmit} className="space-y-4">
          <div>
            <label className="block text-sm text-text-muted mb-1">Project Name</label>
            <input
              type="text"
              value={name}
              onChange={(e) => setName(e.target.value)}
              className="w-full px-3 py-2 bg-surface-alt border border-border rounded-md text-text text-sm focus:outline-none focus:border-accent"
              placeholder="My Track"
            />
          </div>
          <div>
            <label className="block text-sm text-text-muted mb-1">Folder Path</label>
            <div className="flex gap-2">
              <input
                type="text"
                value={path}
                onChange={(e) => setPath(e.target.value)}
                className="flex-1 px-3 py-2 bg-surface-alt border border-border rounded-md text-text text-sm focus:outline-none focus:border-accent"
                placeholder="C:\Projects\My Track"
              />
              <button
                type="button"
                onClick={handleBrowse}
                className="px-3 py-2 text-sm bg-surface-alt border border-border rounded-md text-text hover:bg-surface cursor-pointer"
              >
                Browse
              </button>
            </div>
          </div>
          <div className="flex justify-end gap-2 pt-2">
            <button
              type="button"
              onClick={onClose}
              className="px-4 py-2 text-sm text-text-muted hover:text-text cursor-pointer"
            >
              Cancel
            </button>
            <button
              type="submit"
              className="px-4 py-2 text-sm bg-backup hover:bg-backup-dark text-white rounded-md cursor-pointer"
            >
              Add Project
            </button>
          </div>
        </form>
      </div>
    </div>
  )
}
