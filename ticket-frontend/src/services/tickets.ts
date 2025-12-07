import { apiClient } from './client'
import type {
  Ticket,
  CreateTicketRequest,
  UpdateTicketRequest,
  ApiResponse
} from '@/types'

export interface TicketFilters {
  search?: string
  status?: string
  priority?: string
  tagIds?: string[]
  page?: number
  limit?: number
}

export interface TicketListResponse {
  data: Ticket[]
  total: number
  limit: number
  page: number
}

export const ticketApi = {
  // Get all tickets with optional filters
  getAll: async (filters?: TicketFilters): Promise<Ticket[]> => {
    const params = new URLSearchParams()

    if (filters?.search) params.append('search', filters.search)
    if (filters?.status) params.append('status', filters.status)
    if (filters?.priority) params.append('priority', filters.priority)
    if (filters?.tagIds && filters.tagIds.length > 0) params.append('tag_ids', filters.tagIds.join(','))
    if (filters?.page) params.append('page', filters.page.toString())
    if (filters?.limit) params.append('limit', filters.limit.toString())

    const url = `/api/v1/tickets${params.toString() ? `?${params.toString()}` : ''}`
    const response = await apiClient.get<TicketListResponse>(url)
    return response.data.data
  },

  // Get ticket by ID
  getById: async (id: string): Promise<Ticket> => {
    const response = await apiClient.get<ApiResponse<Ticket>>(`/api/v1/tickets/${id}`)
    return response.data.data
  },

  // Create ticket
  create: async (data: CreateTicketRequest): Promise<Ticket> => {
    const response = await apiClient.post<Ticket>('/api/v1/tickets', data)
    return response.data
  },

  // Update ticket
  update: async (id: string, data: UpdateTicketRequest): Promise<Ticket> => {
    const response = await apiClient.put<ApiResponse<Ticket>>(`/api/v1/tickets/${id}`, data)
    // Handle both wrapped and direct response formats
    if (response.data && response.data.data) {
      return response.data.data
    }
    // If backend returns ticket directly (unwrap from ApiResponse wrapper)
    const ticketData = response.data as unknown as Ticket
    if (ticketData && ticketData.id) {
      return ticketData
    }
    throw new Error('Invalid response format from server')
  },

  // Delete ticket
  delete: async (id: string): Promise<void> => {
    await apiClient.delete(`/api/v1/tickets/${id}`)
  },
}