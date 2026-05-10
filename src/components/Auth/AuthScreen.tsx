import { useState } from 'react'
import { useAuth } from '../../hooks/useAuth'
import { FileAudio } from 'lucide-react'

export function AuthScreen() {
  const [isSignUp, setIsSignUp] = useState(false)
  const [email, setEmail] = useState('')
  const [password, setPassword] = useState('')
  const { handleSignIn, handleSignUp, loading, error } = useAuth()

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    if (isSignUp) {
      await handleSignUp(email, password)
    } else {
      await handleSignIn(email, password)
    }
  }

  return (
    <div className="h-screen flex items-center justify-center bg-surface">
      <div className="w-full max-w-sm">
        <div className="text-center mb-8">
          <FileAudio size={40} className="text-accent mx-auto mb-3" />
          <h1 className="text-xl font-bold text-text">Studio Backup</h1>
          <p className="text-sm text-text-muted mt-1">
            Sign {isSignUp ? 'up' : 'in'} to get started
          </p>
        </div>

        <form onSubmit={handleSubmit} className="space-y-4">
          <div>
            <input
              type="email"
              value={email}
              onChange={(e) => setEmail(e.target.value)}
              placeholder="Email"
              className="w-full px-4 py-2.5 bg-surface-alt border border-border rounded-lg text-text text-sm placeholder:text-text-muted focus:outline-none focus:border-accent"
              required
            />
          </div>
          <div>
            <input
              type="password"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              placeholder="Password"
              className="w-full px-4 py-2.5 bg-surface-alt border border-border rounded-lg text-text text-sm placeholder:text-text-muted focus:outline-none focus:border-accent"
              required
              minLength={6}
            />
          </div>

          {error && (
            <p className="text-xs text-error text-center">{error}</p>
          )}

          <button
            type="submit"
            disabled={loading}
            className="w-full py-2.5 bg-backup hover:bg-backup-dark disabled:opacity-50 text-white text-sm font-medium rounded-lg transition-colors cursor-pointer"
          >
            {loading ? 'Please wait...' : isSignUp ? 'Create Account' : 'Sign In'}
          </button>
        </form>

        <p className="text-center text-xs text-text-muted mt-6">
          {isSignUp ? 'Already have an account?' : "Don't have an account?"}{' '}
          <button
            onClick={() => setIsSignUp(!isSignUp)}
            className="text-accent hover:underline cursor-pointer"
          >
            {isSignUp ? 'Sign in' : 'Sign up'}
          </button>
        </p>
      </div>
    </div>
  )
}
