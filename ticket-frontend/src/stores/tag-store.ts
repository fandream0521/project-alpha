import { create } from 'zustand'
import { devtools } from 'zustand/middleware'
import type { Tag, CreateTagRequest, UpdateTagRequest } from '@/types'
import { tagApi } from '@/services'

interface TagState {
  tags: Tag[]
  loading: boolean
  error: string | null

  // Actions
  setTags: (tags: Tag[]) => void
  addTag: (tag: Tag) => void
  updateTag: (id: string, tag: Partial<Tag>) => void
  removeTag: (id: string) => void
  setLoading: (loading: boolean) => void
  setError: (error: string | null) => void
  clearError: () => void

  // API Actions
  fetchTags: () => Promise<void>
  createTag: (data: CreateTagRequest) => Promise<void>
  updateTagApi: (id: string, data: UpdateTagRequest) => Promise<void>
  deleteTag: (id: string) => Promise<void>
}

export const useTagStore = create<TagState>()(
  devtools(
    (set) => ({
      tags: [],
      loading: false,
      error: null,

      setTags: (tags) => set({ tags }, false, 'setTags'),

      addTag: (tag) =>
        set((state) => ({ tags: [...state.tags, tag] }), false, 'addTag'),

      updateTag: (id, updatedTag) =>
        set(
          (state) => ({
            tags: state.tags.map((tag) =>
              tag.id === id ? { ...tag, ...updatedTag } : tag
            ),
          }),
          false,
          'updateTag'
        ),

      removeTag: (id) =>
        set(
          (state) => ({
            tags: state.tags.filter((tag) => tag.id !== id),
          }),
          false,
          'removeTag'
        ),

      setLoading: (loading) => set({ loading }, false, 'setLoading'),

      setError: (error) => set({ error }, false, 'setError'),

      clearError: () => set({ error: null }, false, 'clearError'),

      // API Actions
      fetchTags: async () => {
        try {
          set({ loading: true, error: null }, false, 'fetchTags-start')
          const tags = await tagApi.getAll()
          set({ tags, loading: false }, false, 'fetchTags-success')
        } catch (error) {
          const errorMessage = error instanceof Error ? error.message : '获取Tags失败'
          set({ error: errorMessage, loading: false }, false, 'fetchTags-error')
          throw error
        }
      },

      createTag: async (data) => {
        try {
          set({ loading: true, error: null }, false, 'createTag-start')
          const newTag = await tagApi.create(data)
          set((state) => ({
            tags: [...state.tags, newTag],
            loading: false
          }), false, 'createTag-success')
        } catch (error) {
          const errorMessage = error instanceof Error ? error.message : '创建Tag失败'
          set({ error: errorMessage, loading: false }, false, 'createTag-error')
          throw error
        }
      },

      updateTagApi: async (id, data) => {
        try {
          set({ loading: true, error: null }, false, 'updateTag-start')
          const updatedTag = await tagApi.update(id, data)
          set((state) => ({
            tags: state.tags.map((tag) =>
              tag.id === id ? updatedTag : tag
            ),
            loading: false
          }), false, 'updateTag-success')
        } catch (error) {
          const errorMessage = error instanceof Error ? error.message : '更新Tag失败'
          set({ error: errorMessage, loading: false }, false, 'updateTag-error')
          throw error
        }
      },

      deleteTag: async (id) => {
        try {
          set({ loading: true, error: null }, false, 'deleteTag-start')
          await tagApi.delete(id)
          set((state) => ({
            tags: state.tags.filter((tag) => tag.id !== id),
            loading: false
          }), false, 'deleteTag-success')
        } catch (error) {
          const errorMessage = error instanceof Error ? error.message : '删除Tag失败'
          set({ error: errorMessage, loading: false }, false, 'deleteTag-error')
          throw error
        }
      },
    }),
    {
      name: 'tag-store',
    }
  )
)