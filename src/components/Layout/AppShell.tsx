import type { ReactNode } from 'react'
import { Sidebar } from '../ProjectList/Sidebar'
import { GlobalStatusBar } from '../StatusBar/GlobalStatusBar'

interface Props {
  children: ReactNode
}

export function AppShell({ children }: Props) {
  return (
    <div className="h-screen flex flex-col bg-surface text-text">
      <div className="flex flex-1 overflow-hidden">
        <Sidebar />
        <main className="flex-1 overflow-y-auto p-6">{children}</main>
      </div>
      <GlobalStatusBar />
    </div>
  )
}
