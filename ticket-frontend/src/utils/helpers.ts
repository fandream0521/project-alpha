import type { Ticket, Tag, TicketStatus, Priority } from '@/types'
import { TICKET_STATUSES, TICKET_PRIORITIES } from './constants'

// Date formatting utilities
export const formatDate = (dateString: string): string => {
  const date = new Date(dateString)
  return date.toLocaleDateString('zh-CN', {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
  })
}

export const formatRelativeTime = (dateString: string): string => {
  const date = new Date(dateString)
  const now = new Date()
  const diffMs = now.getTime() - date.getTime()
  const diffHours = Math.floor(diffMs / (1000 * 60 * 60))
  const diffDays = Math.floor(diffMs / (1000 * 60 * 60 * 24))

  if (diffHours < 1) return '刚刚'
  if (diffHours < 24) return `${diffHours}小时前`
  if (diffDays < 7) return `${diffDays}天前`
  return formatDate(dateString)
}

// Ticket utilities
export const getTicketStatusInfo = (status: TicketStatus) => {
  return TICKET_STATUSES.find(s => s.value === status) || TICKET_STATUSES[0]
}

export const getTicketPriorityInfo = (priority: Priority) => {
  return TICKET_PRIORITIES.find(p => p.value === priority) || TICKET_PRIORITIES[0]
}

export const getTicketStatusColor = (status: TicketStatus): string => {
  const info = getTicketStatusInfo(status)
  return info.color
}

export const getTicketPriorityColor = (priority: Priority): string => {
  const info = getTicketPriorityInfo(priority)
  return info.color
}

// Tag utilities
export const getTagById = (tags: Tag[], id: string): Tag | undefined => {
  return tags.find(tag => tag.id === id)
}

export const getTagNamesByIds = (tags: Tag[], ids: string[]): string[] => {
  return ids
    .map(id => getTagById(tags, id)?.name)
    .filter(Boolean) as string[]
}

// Search utilities
export const highlightSearchTerm = (text: string, searchTerm: string): string => {
  if (!searchTerm.trim()) return text

  const regex = new RegExp(`(${searchTerm.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')})`, 'gi')
  return text.replace(regex, '<mark>$1</mark>')
}

export const escapeRegExp = (string: string): string => {
  return string.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')
}

// Validation utilities
export const validateEmail = (email: string): boolean => {
  const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/
  return emailRegex.test(email)
}

export const validateUrl = (url: string): boolean => {
  try {
    new URL(url)
    return true
  } catch {
    return false
  }
}

export const validateRequired = (value: string): boolean => {
  return value.trim().length > 0
}

export const validateMinLength = (value: string, minLength: number): boolean => {
  return value.trim().length >= minLength
}

export const validateMaxLength = (value: string, maxLength: number): boolean => {
  return value.trim().length <= maxLength
}

// String utilities
export const truncateText = (text: string, maxLength: number): string => {
  if (text.length <= maxLength) return text
  return text.substring(0, maxLength) + '...'
}

export const slugify = (text: string): string => {
  return text
    .toLowerCase()
    .trim()
    .replace(/[^\w\s-]/g, '')
    .replace(/[\s_-]+/g, '-')
    .replace(/^-+|-+$/g, '')
}

// Array utilities
export const uniqueArray = <T>(array: T[]): T[] => {
  return Array.from(new Set(array))
}

export const groupBy = <T, K extends keyof any>(array: T[], key: (item: T) => K): Record<K, T[]> => {
  return array.reduce((groups, item) => {
    const groupKey = key(item)
    groups[groupKey] = groups[groupKey] || []
    groups[groupKey].push(item)
    return groups
  }, {} as Record<K, T[]>)
}

// Color utilities
export const hexToRgb = (hex: string): { r: number; g: number; b: number } | null => {
  const result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex)
  return result ? {
    r: parseInt(result[1], 16),
    g: parseInt(result[2], 16),
    b: parseInt(result[3], 16)
  } : null
}

export const getContrastColor = (hexColor: string): string => {
  const rgb = hexToRgb(hexColor)
  if (!rgb) return '#000000'

  const brightness = (rgb.r * 299 + rgb.g * 587 + rgb.b * 114) / 1000
  return brightness > 128 ? '#000000' : '#ffffff'
}