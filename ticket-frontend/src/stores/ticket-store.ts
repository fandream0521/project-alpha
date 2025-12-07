import { create } from 'zustand'
import { devtools } from 'zustand/middleware'
import type { Ticket, CreateTicketRequest, UpdateTicketRequest } from '@/types'
import { ticketApi, type TicketFilters } from '@/services'

interface TicketState {
  tickets: Ticket[]
  loading: boolean
  error: string | null

  // Actions
  setTickets: (tickets: Ticket[]) => void
  addTicket: (ticket: Ticket) => void
  updateTicket: (id: string, ticket: Partial<Ticket>) => void
  removeTicket: (id: string) => void
  setLoading: (loading: boolean) => void
  setError: (error: string | null) => void
  clearError: () => void

  // API Actions
  fetchTickets: (filters?: TicketFilters) => Promise<void>
  createTicket: (data: CreateTicketRequest) => Promise<void>
  updateTicketApi: (id: string, data: UpdateTicketRequest) => Promise<void>
  deleteTicket: (id: string) => Promise<void>
}

export const useTicketStore = create<TicketState>()(
  devtools(
    (set) => ({
      tickets: [],
      loading: false,
      error: null,

      setTickets: (tickets) => set({ tickets }, false, 'setTickets'),

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

      setLoading: (loading) => set({ loading }, false, 'setLoading'),

      setError: (error) => set({ error }, false, 'setError'),

      clearError: () => set({ error: null }, false, 'clearError'),

      // API Actions
      fetchTickets: async (filters) => {
        try {
          set({ loading: true, error: null }, false, 'fetchTickets-start')
          const tickets = await ticketApi.getAll(filters)
          console.log('Fetched tickets:', tickets)
          // Use backend values directly since frontend components now expect backend format
          set({ tickets, loading: false }, false, 'fetchTickets-success')
        } catch (error) {
          const errorMessage = error instanceof Error ? error.message : '获取Tickets失败'
          set({ error: errorMessage, loading: false }, false, 'fetchTickets-error')
          throw error
        }
      },

      createTicket: async (data) => {
        try {
          set({ loading: true, error: null }, false, 'createTicket-start')
          console.log('Store createTicket, input data:', data)
          // Frontend and backend now use the same values, no transformation needed
          const newTicket = await ticketApi.create(data)
          console.log('Store createTicket, created ticket:', newTicket)
          // Use backend values directly since frontend components now expect backend format
          set((state) => ({
            tickets: [...state.tickets, newTicket],
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
          const updatedTicket = await ticketApi.update(id, data)
          // Use backend values directly since frontend components now expect backend format
          set((state) => ({
            tickets: state.tickets.map((ticket) =>
              ticket.id === id ? updatedTicket : ticket
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
    }),
    {
      name: 'ticket-store',
    }
  )
)