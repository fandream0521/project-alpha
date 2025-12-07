import { apiClient } from './client'
import type {
  Tag,
  CreateTagRequest,
  UpdateTagRequest,
  ApiResponse
} from '@/types'

export const tagApi = {
  // Get all tags
  getAll: async (): Promise<Tag[]> => {
    const response = await apiClient.get<{data: Tag[], total: number}>('/api/v1/tags')
    return response.data.data
  },

  // Get tag by ID
  getById: async (id: string): Promise<Tag> => {
    const response = await apiClient.get<ApiResponse<Tag>>(`/api/v1/tags/${id}`)
    return response.data.data
  },

  // Create tag
  create: async (data: CreateTagRequest): Promise<Tag> => {
    const response = await apiClient.post<Tag>('/api/v1/tags', data)
    return response.data
  },

  // Update tag
  update: async (id: string, data: UpdateTagRequest): Promise<Tag> => {
    const response = await apiClient.put<ApiResponse<Tag>>(`/api/v1/tags/${id}`, data)
    return response.data.data
  },

  // Delete tag
  delete: async (id: string): Promise<void> => {
    await apiClient.delete(`/api/v1/tags/${id}`)
  },
}