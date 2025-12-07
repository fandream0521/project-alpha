import { Tag as TagType } from '@/types'
import { X } from 'lucide-react'
import { cn } from '@/lib/utils'

interface TagProps {
  tag: TagType
  removable?: boolean
  onRemove?: (id: string) => void
  size?: 'sm' | 'md'
  showCount?: boolean
  count?: number
}

export function Tag({ tag, removable, onRemove, size = 'md', showCount = false, count }: TagProps) {
  const sizeClasses = {
    sm: 'px-2 py-0.5 text-xs',
    md: 'px-2.5 py-1 text-sm',
  }

  const hexToRgb = (hex: string) => {
    const result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex)
    return result ? {
      r: parseInt(result[1], 16),
      g: parseInt(result[2], 16),
      b: parseInt(result[3], 16)
    } : { r: 0, g: 0, b: 0 }
  }

  const rgb = hexToRgb(tag.color || '#6B7280')
  const bgColor = `rgba(${rgb.r}, ${rgb.g}, ${rgb.b}, 0.1)`
  const borderColor = `rgba(${rgb.r}, ${rgb.g}, ${rgb.b}, 0.3)`

  return (
    <span
      className={cn(
        'inline-flex items-center gap-1 rounded-full font-medium border transition-colors',
        sizeClasses[size],
      )}
      style={{
        backgroundColor: bgColor,
        color: tag.color || '#6B7280',
        borderColor: borderColor,
      }}
    >
      {tag.name}
      {showCount && count !== undefined && (
        <span className="opacity-75">
          ({count})
        </span>
      )}
      {removable && onRemove && (
        <button
          onClick={() => onRemove(tag.id)}
          className="hover:bg-black/10 rounded-full p-0.5 transition-colors"
          title="移除标签"
        >
          <X className="h-3 w-3" />
        </button>
      )}
    </span>
  )
}