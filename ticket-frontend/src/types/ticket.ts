import type { Tag } from './tag'

export interface Ticket {
  id: string
  title: string
  description?: string
  priority: Priority
  status: TicketStatus
  created_at: string
  updated_at: string
  tags?: Tag[]
}

export type Priority = 'low' | 'medium' | 'high' | 'urgent'
export type TicketStatus = 'open' | 'in_progress' | 'resolved'

export interface CreateTicketRequest {
  title: string
  description?: string
  priority?: Priority
  status?: TicketStatus
  tag_ids?: string[]
}

export interface UpdateTicketRequest {
  title?: string
  description?: string
  priority?: Priority
  status?: TicketStatus
  tag_ids?: string[]
}

// Ticket filter types for API
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