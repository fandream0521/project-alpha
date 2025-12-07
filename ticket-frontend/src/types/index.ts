// Export all types
export type { Ticket, Priority, TicketStatus, CreateTicketRequest, UpdateTicketRequest, TicketFilters, TicketListResponse } from './ticket'
export type { Tag, CreateTagRequest, UpdateTagRequest } from './tag'
export type { ApiResponse, ApiError, PaginationParams, PaginationResponse } from './api'
export type { BaseComponentProps, ButtonProps, InputProps, TextareaProps, CardProps, BadgeProps } from './ui'

// Re-export for backward compatibility
export type {
  Ticket as TicketType,
  Tag as TagType,
  Priority as TicketPriority,
  TicketStatus as TicketStatusEnum,
} from './ticket'