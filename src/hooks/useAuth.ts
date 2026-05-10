import { useCallback, useState } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { useAppStore } from '../stores/appStore'

export function useAuth() {
  const { setAuth, setOnboarding } = useAppStore()
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const handleSignUp = useCallback(
    async (email: string, password: string) => {
      setLoading(true)
      setError(null)
      try {
        const session = await invoke<{
          access_token: string
          refresh_token: string
          user: { id: string; email: string | null }
        }>('sign_up', { email, password })
        setAuth(session.user, session.access_token)
        setOnboarding(false)
      } catch (err) {
        setError(String(err))
      } finally {
        setLoading(false)
      }
    },
    [setAuth, setOnboarding],
  )

  const handleSignIn = useCallback(
    async (email: string, password: string) => {
      setLoading(true)
      setError(null)
      try {
        const session = await invoke<{
          access_token: string
          refresh_token: string
          user: { id: string; email: string | null }
        }>('sign_in', { email, password })
        setAuth(session.user, session.access_token)
        setOnboarding(false)
      } catch (err) {
        setError(String(err))
      } finally {
        setLoading(false)
      }
    },
    [setAuth, setOnboarding],
  )

  const handleSignOut = useCallback(async () => {
    const token = useAppStore.getState().accessToken
    if (token) {
      try {
        await invoke('sign_out', { accessToken: token })
      } catch {
        // Proceed anyway
      }
    }
    setAuth(null, null)
  }, [setAuth])

  return { handleSignUp, handleSignIn, handleSignOut, loading, error }
}
