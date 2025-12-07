import { apiClient } from './client'

export const healthApi = {
  // Basic health check
  check: () => apiClient.get('/health'),

  // Detailed health check
  detailed: () => apiClient.get('/api/health'),

  // Database stats
  getDbStats: () => apiClient.get('/api/db/stats'),

  // Database optimization
  optimizeDb: () => apiClient.post('/api/db/optimize'),
}