import { create } from 'zustand'
import { devtools } from 'zustand/middleware'
import type { Ticket, Tag, CreateTicketRequest, UpdateTicketRequest } from '@/types'
import { ticketApi, tagApi } from '@/services/api'

// Note: Backend and frontend now use the same status/priority values
// Backend: open, in_progress, resolved, urgent, high, medium, low
// Frontend: open, in_progress, resolved, urgent, high, medium, low

// Frontend and backend now use the same values, no mapping needed

interface TicketState {
  tickets: Ticket[]
  tags: Tag[]
  loading: boolean
  error: string | null

  // Actions
  setTickets: (tickets: Ticket[]) => void
  setTags: (tags: Tag[]) => void
  addTicket: (ticket: Ticket) => void
  updateTicket: (id: string, ticket: Partial<Ticket>) => void
  removeTicket: (id: string) => void
  addTag: (tag: Tag) => void
  updateTag: (id: string, tag: Partial<Tag>) => void
  removeTag: (id: string) => void
  setLoading: (loading: boolean) => void
  setError: (error: string | null) => void
  clearError: () => void

  // API Actions
  fetchTickets: () => Promise<void>
  fetchTags: () => Promise<void>
  createTicket: (data: CreateTicketRequest) => Promise<void>
  updateTicketApi: (id: string, data: UpdateTicketRequest) => Promise<void>
  deleteTicket: (id: string) => Promise<void>
  createTag: (data: { name: string; color?: string }) => Promise<void>
  updateTagApi: (id: string, data: { name?: string; color?: string }) => Promise<void>
  deleteTag: (id: string) => Promise<void>
}

export const useTicketStore = create<TicketState>()(
  devtools(
    (set) => ({
      tickets: [],
      tags: [],
      loading: false,
      error: null,

      setTickets: (tickets) => set({ tickets }, false, 'setTickets'),

      setTags: (tags) => set({ tags }, false, 'setTags'),

      addTicket: (ticket) =>
        set((state) => ({ tickets: [...state.tickets, ticket] }), false, 'addTicket'),

      updateTicket: (id, updatedTicket) =>
        set(
          (state) => ({
            tickets: state.tickets.map((ticket) =>
              ticket.id === id ? { ...ticket, ...updatedTicket } : ticket
            ),
          }),
          false,
          'updateTicket'
        ),

      removeTicket: (id) =>
        set(
          (state) => ({
            tickets: state.tickets.filter((ticket) => ticket.id !== id),
          }),
          false,
          'removeTicket'
        ),

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
      fetchTickets: async () => {
        try {
          set({ loading: true, error: null }, false, 'fetchTickets-start')
          const tickets = await ticketApi.getAll()
          // Transform backend status values to frontend expected values
          // Use backend values directly since frontend components now expect backend format
          const transformedTickets = tickets
          set({ tickets: transformedTickets, loading: false }, false, 'fetchTickets-success')
        } catch (error) {
          const errorMessage = error instanceof Error ? error.message : '获取Tickets失败'
          set({ error: errorMessage, loading: false }, false, 'fetchTickets-error')
          throw error
        }
      },

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

      createTicket: async (data) => {
        try {
          set({ loading: true, error: null }, false, 'createTicket-start')
          // Frontend and backend now use the same values, no transformation needed
          const backendData = data
          const newTicket = await ticketApi.create(backendData)
          // Transform backend response to frontend format
          // Use backend values directly since frontend components now expect backend format
          const transformedTicket = newTicket
          set((state) => ({
            tickets: [...state.tickets, transformedTicket],
            loading: false
          }), false, 'createTicket-success')
        } catch (error) {
          const errorMessage = error instanceof Error ? error.message : '创建Ticket失败'
          set({ error: errorMessage, loading: false }, false, 'createTicket-error')
          throw error
        }
      },

      updateTicketApi: async (id, data) => {
        try {
          set({ loading: true, error: null }, false, 'updateTicket-start')
          // Frontend and backend now use the same values, no transformation needed
          const backendData = data
          const updatedTicket = await ticketApi.update(id, backendData)
          // Transform backend response to frontend format
          // Use backend values directly since frontend components now expect backend format
          const transformedTicket = updatedTicket
          set((state) => ({
            tickets: state.tickets.map((ticket) =>
              ticket.id === id ? transformedTicket : ticket
            ),
            loading: false
          }), false, 'updateTicket-success')
        } catch (error) {
          const errorMessage = error instanceof Error ? error.message : '更新Ticket失败'
          set({ error: errorMessage, loading: false }, false, 'updateTicket-error')
          throw error
        }
      },

      deleteTicket: async (id) => {
        try {
          set({ loading: true, error: null }, false, 'deleteTicket-start')
          await ticketApi.delete(id)
          set((state) => ({
            tickets: state.tickets.filter((ticket) => ticket.id !== id),
            loading: false
          }), false, 'deleteTicket-success')
        } catch (error) {
          const errorMessage = error instanceof Error ? error.message : '删除Ticket失败'
          set({ error: errorMessage, loading: false }, false, 'deleteTicket-error')
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
      name: 'ticket-store',
    }
  )
)

