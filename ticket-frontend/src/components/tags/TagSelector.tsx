import { useState, useEffect } from 'react'
import { Tag } from './Tag'
import { ChevronDown } from 'lucide-react'
import { useTagStore } from '@/stores'
import { cn } from '@/lib/utils'

interface TagSelectorProps {
  selectedTagIds: string[]
  onChange: (tagIds: string[]) => void
  placeholder?: string
  className?: string
}

export function TagSelector({ selectedTagIds, onChange, placeholder = '选择标签', className }: TagSelectorProps) {
  const [isOpen, setIsOpen] = useState(false)
  const { tags, fetchTags } = useTicketStore()

  useEffect(() => {
    fetchTags()
  }, [fetchTags])

  const selectedTags = tags.filter(tag => selectedTagIds.includes(tag.id))
  const availableTags = tags.filter(tag => !selectedTagIds.includes(tag.id))

  const handleTagSelect = (tagId: string) => {
    onChange([...selectedTagIds, tagId])
    setIsOpen(false)
  }

  const handleTagRemove = (tagId: string) => {
    onChange(selectedTagIds.filter(id => id !== tagId))
  }

  // Close dropdown when clicking outside
  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      const target = event.target as Element
      if (!target.closest('.tag-selector')) {
        setIsOpen(false)
      }
    }

    if (isOpen) {
      document.addEventListener('mousedown', handleClickOutside)
      return () => {
        document.removeEventListener('mousedown', handleClickOutside)
      }
    }
  }, [isOpen])

  return (
    <div className={cn('relative tag-selector', className)}>
      {/* Selected Tags Display */}
      <div
        className="min-h-10 px-3 py-2 border border-input bg-background rounded-md cursor-text flex flex-wrap gap-2 items-center hover:border-ring focus-within:border-ring focus-within:ring-2 focus-within:ring-ring focus-within:ring-offset-2"
        onClick={() => setIsOpen(!isOpen)}
      >
        {selectedTags.length === 0 ? (
          <span className="text-muted-foreground">{placeholder}</span>
        ) : (
          selectedTags.map(tag => (
            <Tag
              key={tag.id}
              tag={tag}
              removable
              onRemove={handleTagRemove}
              size="sm"
            />
          ))
        )}
        <ChevronDown className={cn(
          "h-4 w-4 text-muted-foreground ml-auto transition-transform",
          isOpen && "rotate-180"
        )} />
      </div>

      {/* Dropdown */}
      {isOpen && (
        <div className="absolute z-10 mt-1 w-full bg-popover border border-input rounded-md shadow-lg">
          <div className="max-h-60 overflow-auto py-1">
            {availableTags.length === 0 ? (
              <div className="px-3 py-2 text-sm text-muted-foreground">
                没有可用的标签
              </div>
            ) : (
              availableTags.map(tag => (
                <button
                  key={tag.id}
                  className="w-full px-3 py-2 text-left hover:bg-accent hover:text-accent-foreground flex items-center gap-2 transition-colors"
                  onClick={() => handleTagSelect(tag.id)}
                >
                  <Tag tag={tag} size="sm" />
                </button>
              ))
            )}
          </div>
        </div>
      )}
    </div>
  )
}