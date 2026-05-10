import { useState } from 'react'
import { FolderOpen, ArrowLeft, CheckCircle2 } from 'lucide-react'

interface Props {
  snapshotId: string
  onClose: () => void
}

export function RestoreWizard({ snapshotId, onClose }: Props) {
  const [destination, setDestination] = useState('')
  const [step, setStep] = useState<'choose' | 'restoring' | 'done'>('choose')
  const [progress, setProgress] = useState(0)

  const handleRestore = async () => {
    setStep('restoring')
    // Restore will be implemented via Tauri command
    for (let i = 0; i <= 100; i += 10) {
      await new Promise((r) => setTimeout(r, 200))
      setProgress(i)
    }
    setStep('done')
  }

  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div className="bg-surface border border-border rounded-lg p-6 w-96 shadow-xl">
        {step === 'choose' && (
          <>
            <h2 className="text-lg font-semibold mb-4">Restore Snapshot</h2>
            <p className="text-sm text-text-muted mb-4">
              Choose where to restore the files. We recommend restoring to a new
              location to avoid overwriting your current project.
            </p>
            <div className="mb-4">
              <label className="block text-sm text-text-muted mb-1">
                Destination Folder
              </label>
              <div className="flex gap-2">
                <input
                  type="text"
                  value={destination}
                  onChange={(e) => setDestination(e.target.value)}
                  className="flex-1 px-3 py-2 bg-surface-alt border border-border rounded-md text-text text-sm"
                  placeholder="C:\Restores\My Track v2"
                />
                <button className="px-3 py-2 bg-surface-alt border border-border rounded-md text-text cursor-pointer">
                  <FolderOpen size={16} />
                </button>
              </div>
            </div>
            <div className="flex justify-end gap-2">
              <button
                onClick={onClose}
                className="px-4 py-2 text-sm text-text-muted hover:text-text cursor-pointer"
              >
                Cancel
              </button>
              <button
                onClick={handleRestore}
                className="px-4 py-2 text-sm bg-backup hover:bg-backup-dark text-white rounded-md cursor-pointer"
              >
                Start Restore
              </button>
            </div>
          </>
        )}

        {step === 'restoring' && (
          <>
            <h2 className="text-lg font-semibold mb-4">Restoring...</h2>
            <div className="w-full bg-surface-alt rounded-full h-2 mb-2">
              <div
                className="bg-accent h-2 rounded-full transition-all duration-300"
                style={{ width: `${progress}%` }}
              />
            </div>
            <p className="text-xs text-text-muted">{progress}% complete</p>
          </>
        )}

        {step === 'done' && (
          <>
            <div className="text-center py-4">
              <CheckCircle2 size={40} className="text-success mx-auto mb-3" />
              <h2 className="text-lg font-semibold mb-2">Restore Complete</h2>
              <p className="text-sm text-text-muted">
                All files restored successfully to the destination folder.
              </p>
            </div>
            <button
              onClick={onClose}
              className="w-full mt-4 px-4 py-2 text-sm bg-backup hover:bg-backup-dark text-white rounded-md cursor-pointer"
            >
              Done
            </button>
          </>
        )}
      </div>
    </div>
  )
}
