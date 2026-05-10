import { useState } from 'react'
import { useAppStore } from '../../stores/appStore'

export function OnboardingFlow() {
  const { setOnboarding } = useAppStore()
  const [step, setStep] = useState(0)

  const steps = [
    {
      title: 'Welcome to Studio Backup',
      description: 'Automatically back up and version your music production projects. Never lose a session again.',
    },
    {
      title: 'Add Your First Project',
      description: 'Select a folder containing your DAW project. Studio Backup will watch for changes and create automatic snapshots.',
    },
    {
      title: 'Automatic & Invisible',
      description: 'Studio Backup runs quietly in the background. When it detects changes, it creates a new version. When you need it, your history is one click away.',
    },
  ]

  const handleFinish = () => {
    setOnboarding(false)
  }

  return (
    <div className="h-screen flex items-center justify-center bg-surface">
      <div className="text-center max-w-md">
        <div className="text-5xl mb-6">🎵</div>
        <h1 className="text-2xl font-bold text-text mb-3">
          {steps[step].title}
        </h1>
        <p className="text-text-muted mb-8 text-sm leading-relaxed">
          {steps[step].description}
        </p>
        <div className="flex justify-center gap-2 mb-8">
          {steps.map((_, i) => (
            <div
              key={i}
              className={`w-2 h-2 rounded-full ${
                i === step ? 'bg-accent' : 'bg-border'
              }`}
            />
          ))}
        </div>
        <div className="flex justify-center gap-3">
          {step > 0 && (
            <button
              onClick={() => setStep(step - 1)}
              className="px-4 py-2 text-sm text-text-muted hover:text-text cursor-pointer"
            >
              Back
            </button>
          )}
          {step < steps.length - 1 ? (
            <button
              onClick={() => setStep(step + 1)}
              className="px-6 py-2 text-sm bg-backup hover:bg-backup-dark text-white rounded-md cursor-pointer"
            >
              Next
            </button>
          ) : (
            <button
              onClick={handleFinish}
              className="px-6 py-2 text-sm bg-backup hover:bg-backup-dark text-white rounded-md cursor-pointer"
            >
              Get Started
            </button>
          )}
        </div>
      </div>
    </div>
  )
}
