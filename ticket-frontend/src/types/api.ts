export interface ApiResponse<T> {
  data: T
}

export interface ApiError {
  message: string
  status: number
}

export interface PaginationParams {
  page?: number
  limit?: number
}

export interface PaginationResponse<T> {
  data: T[]
  total: number
  page: number
  limit: number
}