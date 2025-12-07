import axios from 'axios'
import type {
  Ticket,
  Tag,
  CreateTicketRequest,
  UpdateTicketRequest,
  CreateTagRequest,
  UpdateTagRequest,
  ApiResponse,
  ApiError
} from '@/types'

const API_BASE_URL = import.meta.env.VITE_API_URL || 'http://localhost:3000'

const api = axios.create({
  baseURL: API_BASE_URL,
  headers: {
    'Content-Type': 'application/json',
  },
})

// Request interceptor
api.interceptors.request.use(
  (config) => {
    // Add auth token if available
    // const token = localStorage.getItem('token')
    // if (token) {
    //   config.headers.Authorization = `Bearer ${token}`
    // }
    return config
  },
  (error) => Promise.reject(error)
)

// Response interceptor
api.interceptors.response.use(
  (response) => response,
  (error): Promise<ApiError> => {
    const apiError: ApiError = {
      message: error.response?.data?.error?.message || error.message || 'An error occurred',
      status: error.response?.status || 500,
    }
    return Promise.reject(apiError)
  }
)

// Health check
export const healthCheck = () => api.get('/health')

// Database stats
export const getDbStats = () => api.get('/api/db/stats')

// Ticket API
export const ticketApi = {
  // Get all tickets with optional filters
  getAll: async (filters?: {
    search?: string
    status?: string
    priority?: string
    tagIds?: string[]
    page?: number
    limit?: number
  }): Promise<Ticket[]> => {
    const params = new URLSearchParams()

    if (filters?.search) params.append('search', filters.search)
    if (filters?.status) params.append('status', filters.status)
    if (filters?.priority) params.append('priority', filters.priority)
    if (filters?.tagIds && filters.tagIds.length > 0) params.append('tag_ids', filters.tagIds.join(','))
    if (filters?.page) params.append('page', filters.page.toString())
    if (filters?.limit) params.append('limit', filters.limit.toString())

    const url = `/api/v1/tickets${params.toString() ? `?${params.toString()}` : ''}`
    const response = await api.get<ApiResponse<Ticket[]>>(url)
    return response.data.data
  },

  // Get ticket by ID
  getById: async (id: string): Promise<Ticket> => {
    const response = await api.get<ApiResponse<Ticket>>(`/api/v1/tickets/${id}`)
    return response.data.data
  },

  // Create ticket
  create: async (data: CreateTicketRequest): Promise<Ticket> => {
    const response = await api.post<ApiResponse<Ticket>>('/api/v1/tickets', data)
    return response.data.data
  },

  // Update ticket
  update: async (id: string, data: UpdateTicketRequest): Promise<Ticket> => {
    const response = await api.put<ApiResponse<Ticket>>(`/api/v1/tickets/${id}`, data)
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
    await api.delete(`/api/v1/tickets/${id}`)
  },
}

// Tag API
export const tagApi = {
  // Get all tags
  getAll: async (): Promise<Tag[]> => {
    const response = await api.get<ApiResponse<Tag[]>>('/api/v1/tags')
    return response.data.data
  },

  // Get tag by ID
  getById: async (id: string): Promise<Tag> => {
    const response = await api.get<ApiResponse<Tag>>(`/api/v1/tags/${id}`)
    return response.data.data
  },

  // Create tag
  create: async (data: CreateTagRequest): Promise<Tag> => {
    const response = await api.post<ApiResponse<Tag>>('/api/v1/tags', data)
    return response.data.data
  },

  // Update tag
  update: async (id: string, data: UpdateTagRequest): Promise<Tag> => {
    const response = await api.put<ApiResponse<Tag>>(`/api/v1/tags/${id}`, data)
    return response.data.data
  },

  // Delete tag
  delete: async (id: string): Promise<void> => {
    await api.delete(`/api/v1/tags/${id}`)
  },
}

export default api