import { Link, useLocation } from 'react-router-dom'
import { Home, Tag as TagIcon, Settings } from 'lucide-react'
import { cn } from '@/lib/utils'

export function Navigation() {
  const location = useLocation()

  const navigation = [
    { name: 'Tickets', href: '/', icon: Home, current: location.pathname === '/' },
    { name: '标签管理', href: '/tags', icon: TagIcon, current: location.pathname === '/tags' },
  ]

  return (
    <nav className="bg-card border-b">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="flex justify-between h-16">
          <div className="flex">
            <div className="flex-shrink-0 flex items-center">
              <TagIcon className="h-8 w-8 text-primary" />
              <span className="ml-2 text-xl font-bold">Ticket System</span>
            </div>
            <div className="hidden sm:ml-6 sm:flex sm:space-x-8">
              {navigation.map((item) => {
                const Icon = item.icon
                return (
                  <Link
                    key={item.name}
                    to={item.href}
                    className={cn(
                      item.current
                        ? 'border-primary text-foreground'
                        : 'border-transparent text-muted-foreground hover:text-foreground hover:border-muted-foreground',
                      'inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium transition-colors'
                    )}
                  >
                    <Icon className="h-4 w-4 mr-2" />
                    {item.name}
                  </Link>
                )
              })}
            </div>
          </div>
          <div className="flex items-center">
            <button className="p-2 rounded-md text-muted-foreground hover:text-foreground hover:bg-accent transition-colors">
              <Settings className="h-5 w-5" />
            </button>
          </div>
        </div>
      </div>

      {/* Mobile navigation */}
      <div className="sm:hidden border-t border-border">
        <div className="px-2 pt-2 pb-3 space-y-1">
          {navigation.map((item) => {
            const Icon = item.icon
            return (
              <Link
                key={item.name}
                to={item.href}
                className={cn(
                  item.current
                    ? 'bg-accent text-accent-foreground'
                    : 'text-muted-foreground hover:bg-accent hover:text-accent-foreground',
                  'group flex items-center px-2 py-2 text-base font-medium rounded-md transition-colors'
                )}
              >
                <Icon className="h-5 w-5 mr-3" />
                {item.name}
              </Link>
            )
          })}
        </div>
      </div>
    </nav>
  )
}