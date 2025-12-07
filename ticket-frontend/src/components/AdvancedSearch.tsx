import { useState, useMemo } from 'react'
import { Search, Filter, X, ChevronDown, ChevronUp } from 'lucide-react'
import { Button } from '@/components/ui/Button'
import { Input } from '@/components/ui/Input'
import { Badge } from '@/components/ui/Badge'
import { Card, CardContent } from '@/components/ui/Card'
import { Tag } from '@/types'

interface SearchFilters {
  search: string
  status: string
  priority: string
  tagIds: string[]
}

interface AdvancedSearchProps {
  tags: Tag[]
  onFiltersChange: (filters: SearchFilters) => void
  loading?: boolean
}

const STATUS_OPTIONS = [
  { value: '', label: '全部状态' },
  { value: 'Todo', label: '待办' },
  { value: 'InProgress', label: '进行中' },
  { value: 'Done', label: '已完成' }
]

const PRIORITY_OPTIONS = [
  { value: '', label: '全部优先级' },
  { value: 'High', label: '高' },
  { value: 'Medium', label: '中' },
  { value: 'Low', label: '低' }
]

export function AdvancedSearch({ tags, onFiltersChange, loading = false }: AdvancedSearchProps) {
  const [isExpanded, setIsExpanded] = useState(false)
  const [filters, setFilters] = useState<SearchFilters>({
    search: '',
    status: '',
    priority: '',
    tagIds: []
  })

  // 计算激活的过滤器数量
  const activeFiltersCount = useMemo(() => {
    let count = 0
    if (filters.search.trim()) count++
    if (filters.status) count++
    if (filters.priority) count++
    if (filters.tagIds.length > 0) count++
    return count
  }, [filters])

  // 处理搜索框变化
  const handleSearchChange = (value: string) => {
    const newFilters = { ...filters, search: value }
    setFilters(newFilters)
    onFiltersChange(newFilters)
  }

  // 处理状态变化
  const handleStatusChange = (value: string) => {
    const newFilters = { ...filters, status: value }
    setFilters(newFilters)
    onFiltersChange(newFilters)
  }

  // 处理优先级变化
  const handlePriorityChange = (value: string) => {
    const newFilters = { ...filters, priority: value }
    setFilters(newFilters)
    onFiltersChange(newFilters)
  }

  // 处理标签选择
  const handleTagToggle = (tagId: string) => {
    const newTagIds = filters.tagIds.includes(tagId)
      ? filters.tagIds.filter(id => id !== tagId)
      : [...filters.tagIds, tagId]

    const newFilters = { ...filters, tagIds: newTagIds }
    setFilters(newFilters)
    onFiltersChange(newFilters)
  }

  // 清除所有过滤器
  const clearFilters = () => {
    const newFilters = {
      search: '',
      status: '',
      priority: '',
      tagIds: []
    }
    setFilters(newFilters)
    onFiltersChange(newFilters)
  }

  // 清除单个过滤器
  const clearFilter = (filterType: keyof SearchFilters) => {
    const newFilters = { ...filters, [filterType]: filterType === 'tagIds' ? [] : '' }
    setFilters(newFilters)
    onFiltersChange(newFilters)
  }

  // 获取选中的标签对象
  const selectedTags = useMemo(() => {
    return tags.filter(tag => filters.tagIds.includes(tag.id))
  }, [tags, filters.tagIds])

  return (
    <Card className="mb-6">
      <CardContent className="p-4">
        {/* 搜索框和过滤器按钮 */}
        <div className="flex items-center gap-4 mb-4">
          <div className="flex-1 relative">
            <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 h-4 w-4 text-muted-foreground" />
            <Input
              placeholder="搜索Tickets（标题或描述）..."
              value={filters.search}
              onChange={(e) => handleSearchChange(e.target.value)}
              className="pl-10"
              disabled={loading}
            />
          </div>
          <Button
            variant="outline"
            onClick={() => setIsExpanded(!isExpanded)}
            className="flex items-center gap-2"
          >
            <Filter className="h-4 w-4" />
            过滤器
            {activeFiltersCount > 0 && (
              <Badge variant="secondary" className="text-xs">
                {activeFiltersCount}
              </Badge>
            )}
            {isExpanded ? (
              <ChevronUp className="h-4 w-4" />
            ) : (
              <ChevronDown className="h-4 w-4" />
            )}
          </Button>
        </div>

        {/* 当前激活的过滤器 */}
        {activeFiltersCount > 0 && (
          <div className="flex items-center gap-2 mb-4 flex-wrap">
            <span className="text-sm text-muted-foreground">当前过滤器:</span>

            {filters.status && (
              <Badge variant="secondary" className="flex items-center gap-1">
                状态: {STATUS_OPTIONS.find(opt => opt.value === filters.status)?.label}
                <X
                  className="h-3 w-3 cursor-pointer hover:text-destructive"
                  onClick={() => clearFilter('status')}
                />
              </Badge>
            )}

            {filters.priority && (
              <Badge variant="secondary" className="flex items-center gap-1">
                优先级: {PRIORITY_OPTIONS.find(opt => opt.value === filters.priority)?.label}
                <X
                  className="h-3 w-3 cursor-pointer hover:text-destructive"
                  onClick={() => clearFilter('priority')}
                />
              </Badge>
            )}

            {selectedTags.map(tag => (
              <Badge
                key={tag.id}
                variant="secondary"
                className="flex items-center gap-1"
                style={{ backgroundColor: `${tag.color}20`, borderColor: tag.color }}
              >
                {tag.name}
                <X
                  className="h-3 w-3 cursor-pointer hover:text-destructive"
                  onClick={() => handleTagToggle(tag.id)}
                />
              </Badge>
            ))}

            <Button
              variant="outline"
              size="sm"
              onClick={clearFilters}
              className="h-6 text-xs"
            >
              清除全部
            </Button>
          </div>
        )}

        {/* 高级过滤器面板 */}
        {isExpanded && (
          <div className="border-t pt-4 space-y-4">
            {/* 状态过滤器 */}
            <div>
              <label className="block text-sm font-medium mb-2">状态</label>
              <div className="flex flex-wrap gap-2">
                {STATUS_OPTIONS.map(option => (
                  <Button
                    key={option.value}
                    variant={filters.status === option.value ? "default" : "outline"}
                    size="sm"
                    onClick={() => handleStatusChange(option.value)}
                    disabled={loading}
                  >
                    {option.label}
                  </Button>
                ))}
              </div>
            </div>

            {/* 优先级过滤器 */}
            <div>
              <label className="block text-sm font-medium mb-2">优先级</label>
              <div className="flex flex-wrap gap-2">
                {PRIORITY_OPTIONS.map(option => (
                  <Button
                    key={option.value}
                    variant={filters.priority === option.value ? "default" : "outline"}
                    size="sm"
                    onClick={() => handlePriorityChange(option.value)}
                    disabled={loading}
                  >
                    {option.label}
                  </Button>
                ))}
              </div>
            </div>

            {/* 标签过滤器 */}
            {tags.length > 0 && (
              <div>
                <label className="block text-sm font-medium mb-2">标签</label>
                <div className="flex flex-wrap gap-2">
                  {tags.map(tag => (
                    <Badge
                      key={tag.id}
                      variant={filters.tagIds.includes(tag.id) ? "default" : "outline"}
                      className="cursor-pointer hover:opacity-80"
                      style={{
                        backgroundColor: filters.tagIds.includes(tag.id) ? tag.color : 'transparent',
                        borderColor: tag.color,
                        color: filters.tagIds.includes(tag.id) ? 'white' : tag.color
                      }}
                      onClick={() => handleTagToggle(tag.id)}
                    >
                      {tag.name}
                    </Badge>
                  ))}
                </div>
              </div>
            )}
          </div>
        )}
      </CardContent>
    </Card>
  )
}