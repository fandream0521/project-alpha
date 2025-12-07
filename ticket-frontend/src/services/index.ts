// Export all API modules
export { apiClient } from './client'
export { healthApi } from './health'
export { ticketApi, type TicketFilters, type TicketListResponse } from './tickets'
export { tagApi } from './tags'

// Legacy exports for backward compatibility
export { healthApi as healthCheck } from './health'