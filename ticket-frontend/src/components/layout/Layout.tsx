import { Navigation } from './Navigation'

interface LayoutProps {
  children: React.ReactNode
}

function Layout({ children }: LayoutProps) {
  return (
    <div className="min-h-screen bg-background">
      <Navigation />
      <main className="py-8">
        {children}
      </main>
    </div>
  )
}

export { Layout }