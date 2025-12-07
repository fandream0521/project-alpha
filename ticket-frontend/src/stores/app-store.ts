import { create } from 'zustand'
import { devtools, persist } from 'zustand/middleware'

interface AppState {
  theme: 'light' | 'dark'
  language: 'en' | 'zh'
  sidebarOpen: boolean

  // Actions
  setTheme: (theme: 'light' | 'dark') => void
  setLanguage: (language: 'en' | 'zh') => void
  toggleSidebar: () => void
  setSidebarOpen: (open: boolean) => void
}

export const useAppStore = create<AppState>()(
  devtools(
    persist(
      (set) => ({
        theme: 'light',
        language: 'zh',
        sidebarOpen: true,

        setTheme: (theme) => set({ theme }, false, 'setTheme'),

        setLanguage: (language) => set({ language }, false, 'setLanguage'),

        toggleSidebar: () =>
          set((state) => ({ sidebarOpen: !state.sidebarOpen }), false, 'toggleSidebar'),

        setSidebarOpen: (open) => set({ sidebarOpen: open }, false, 'setSidebarOpen'),
      }),
      {
        name: 'app-store',
        partialize: (state) => ({
          theme: state.theme,
          language: state.language,
        }),
      }
    ),
    {
      name: 'app-store',
    }
  )
)