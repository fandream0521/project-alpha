import type { ReactNode } from 'react'

export interface BaseComponentProps {
  className?: string
  children?: ReactNode
}

export interface ButtonProps extends BaseComponentProps {
  variant?: 'primary' | 'secondary' | 'outline' | 'ghost' | 'danger'
  size?: 'sm' | 'md' | 'lg'
  disabled?: boolean
  loading?: boolean
  onClick?: () => void
  type?: 'button' | 'submit' | 'reset'
}

export interface InputProps extends BaseComponentProps {
  type?: string
  placeholder?: string
  value?: string
  disabled?: boolean
  error?: string
  onChange?: (value: string) => void
  onBlur?: () => void
  onFocus?: () => void
}

export interface TextareaProps extends BaseComponentProps {
  placeholder?: string
  value?: string
  disabled?: boolean
  error?: string
  rows?: number
  onChange?: (value: string) => void
  onBlur?: () => void
  onFocus?: () => void
}

export interface CardProps extends BaseComponentProps {
  title?: string
  subtitle?: string
  footer?: ReactNode
  hoverable?: boolean
}

export interface BadgeProps extends BaseComponentProps {
  variant?: 'default' | 'primary' | 'secondary' | 'success' | 'warning' | 'danger'
  size?: 'sm' | 'md' | 'lg'
}