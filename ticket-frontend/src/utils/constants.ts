import type { TicketStatus, Priority } from '@/types'

// Ticket status constants
export const TICKET_STATUSES: { value: TicketStatus; label: string; color: string }[] = [
  { value: 'open', label: '待处理', color: 'blue' },
  { value: 'in_progress', label: '进行中', color: 'yellow' },
  { value: 'resolved', label: '已解决', color: 'green' },
]

// Priority constants
export const TICKET_PRIORITIES: { value: Priority; label: string; color: string }[] = [
  { value: 'low', label: '低', color: 'gray' },
  { value: 'medium', label: '中', color: 'blue' },
  { value: 'high', label: '高', color: 'orange' },
  { value: 'urgent', label: '紧急', color: 'red' },
]

// API endpoints
export const API_ENDPOINTS = {
  HEALTH: '/health',
  HEALTH_DETAILED: '/api/health',
  DB_STATS: '/api/db/stats',
  DB_OPTIMIZE: '/api/db/optimize',
  TICKETS: '/api/v1/tickets',
  TAGS: '/api/v1/tags',
} as const

// Pagination defaults
export const PAGINATION_DEFAULTS = {
  PAGE: 1,
  LIMIT: 20,
  MAX_LIMIT: 100,
} as const

// Local storage keys
export const STORAGE_KEYS = {
  AUTH_TOKEN: 'auth_token',
  USER_PREFERENCES: 'user_preferences',
  THEME: 'theme',
  LANGUAGE: 'language',
} as const