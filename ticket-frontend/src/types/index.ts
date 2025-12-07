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

export interface Tag {
  id: string
  name: string
  color?: string
  created_at: string
  updated_at: string
}

export type Priority = 'low' | 'medium' | 'high' | 'urgent'
export type TicketStatus = 'open' | 'in_progress' | 'resolved'

export interface CreateTicketRequest {
  title: string
  description?: string
  priority?: Priority
  status?: TicketStatus
  tagIds?: string[]
}

export interface UpdateTicketRequest {
  title?: string
  description?: string
  priority?: Priority
  status?: TicketStatus
  tagIds?: string[]
}

export interface CreateTagRequest {
  name: string
  color?: string
}

export interface UpdateTagRequest {
  name?: string
  color?: string
}

export interface ApiResponse<T> {
  data: T
}

export interface ApiError {
  message: string
  status: number
}