import { useState, useEffect } from 'react'
import { Plus, Edit2, Trash2, Tag as TagIcon } from 'lucide-react'
import { Tag } from '@/types'
import { Button } from '@/components/ui/Button'
import { Input } from '@/components/ui/Input'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/Card'
import { Tag as TagComponent } from '@/components/Tag'
import { useTicketStore } from '@/stores/ticketStore'
import toast from 'react-hot-toast'
import { cn } from '@/lib/utils'

const PRESET_COLORS = [
  '#3B82F6', '#EF4444', '#10B981', '#F59E0B',
  '#8B5CF6', '#EC4899', '#6B7280', '#14B8A6',
  '#F97316', '#06B6D4', '#84CC16', '#A855F7',
  '#F43F5E', '#8B5A2B', '#0EA5E9', '#22C55E'
]

export function TagManagePage() {
  const [showForm, setShowForm] = useState(false)
  const [editingTag, setEditingTag] = useState<Tag | undefined>()
  const [formData, setFormData] = useState({
    name: '',
    color: PRESET_COLORS[0],
  })

  const { tags, loading, fetchTags, createTag, deleteTag } = useTicketStore()

  useEffect(() => {
    fetchTags()
  }, [fetchTags])

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()

    if (!formData.name.trim()) {
      toast.error('标签名称不能为空')
      return
    }

    try {
      if (editingTag) {
        // Update tag - need to implement updateTag in store
        toast.success('标签更新功能暂未实现')
      } else {
        await createTag({
          name: formData.name.trim(),
          color: formData.color,
        })
        toast.success('标签创建成功')
      }

      setShowForm(false)
      setEditingTag(undefined)
      setFormData({ name: '', color: PRESET_COLORS[0] })
    } catch (error) {
      // Error is already handled in the store
    }
  }

  const handleEdit = (tag: Tag) => {
    setEditingTag(tag)
    setFormData({
      name: tag.name,
      color: tag.color || PRESET_COLORS[0],
    })
    setShowForm(true)
  }

  const handleDelete = async (id: string) => {
    if (window.confirm('确定要删除这个标签吗？删除后所有tickets上的此标签也会被移除。')) {
      try {
        await deleteTag(id)
        toast.success('标签删除成功')
      } catch (error) {
        // Error is already handled in the store
      }
    }
  }

  const getTicketCountForTag = (tagId: string) => {
    const { tickets } = useTicketStore.getState()
    return tickets.filter(ticket => ticket.tags?.some(tag => tag.id === tagId)).length
  }

  return (
    <div className="max-w-4xl mx-auto px-4">
      {/* Header */}
      <div className="mb-8 flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold">标签管理</h1>
          <p className="text-muted-foreground mt-1">管理和组织你的标签</p>
        </div>
        <Button
          onClick={() => {
            setEditingTag(undefined)
            setFormData({ name: '', color: PRESET_COLORS[0] })
            setShowForm(true)
          }}
          className="flex items-center gap-2"
        >
          <Plus className="h-4 w-4" />
          创建标签
        </Button>
      </div>

      {/* Tag Form */}
      {showForm && (
        <Card className="mb-6">
          <CardHeader>
            <CardTitle>
              {editingTag ? '编辑标签' : '创建新标签'}
            </CardTitle>
          </CardHeader>
          <CardContent>
            <form onSubmit={handleSubmit} className="space-y-4">
              <div>
                <label htmlFor="tagName" className="block text-sm font-medium mb-1">
                  标签名称
                </label>
                <Input
                  id="tagName"
                  value={formData.name}
                  onChange={(e) => setFormData({ ...formData, name: e.target.value })}
                  placeholder="输入标签名称"
                  disabled={loading}
                  required
                />
              </div>

              <div>
                <label className="block text-sm font-medium mb-2">
                  选择颜色
                </label>
                <div className="flex flex-wrap gap-2">
                  {PRESET_COLORS.map(color => (
                    <button
                      key={color}
                      type="button"
                      onClick={() => setFormData({ ...formData, color })}
                      className={cn(
                        'w-8 h-8 rounded-full border-2 transition-all hover:scale-105',
                        formData.color === color
                          ? 'border-foreground ring-2 ring-offset-2 ring-foreground'
                          : 'border-input'
                      )}
                      style={{ backgroundColor: color }}
                      title={color}
                    />
                  ))}
                </div>
                <div className="mt-3">
                  <span
                    className="inline-flex items-center gap-2 px-3 py-1 rounded-full text-sm font-medium border"
                    style={{
                      backgroundColor: `${formData.color}20`,
                      color: formData.color,
                      borderColor: `${formData.color}40`
                    }}
                  >
                    <TagIcon className="h-4 w-4" />
                    {formData.name || '预览'}
                  </span>
                </div>
              </div>

              <div className="flex gap-3">
                <Button type="submit" disabled={loading || !formData.name.trim()}>
                  {loading ? '保存中...' : (editingTag ? '更新' : '创建')}
                </Button>
                <Button
                  type="button"
                  variant="outline"
                  onClick={() => {
                    setShowForm(false)
                    setEditingTag(undefined)
                    setFormData({ name: '', color: PRESET_COLORS[0] })
                  }}
                  disabled={loading}
                >
                  取消
                </Button>
              </div>
            </form>
          </CardContent>
        </Card>
      )}

      {/* Tag List */}
      <Card>
        <CardHeader>
          <CardTitle>所有标签 ({tags.length})</CardTitle>
        </CardHeader>
        <CardContent>
          {tags.length === 0 ? (
            <div className="text-center py-8">
              <TagIcon className="h-12 w-12 text-muted-foreground mx-auto mb-4" />
              <p className="text-muted-foreground">还没有创建任何标签</p>
              <p className="text-sm text-muted-foreground mt-1">
                点击上方的"创建标签"按钮来创建你的第一个标签
              </p>
            </div>
          ) : (
            <div className="space-y-3">
              {tags.map(tag => {
                const ticketCount = getTicketCountForTag(tag.id)
                return (
                  <div
                    key={tag.id}
                    className="flex items-center justify-between p-4 border rounded-lg hover:bg-accent/50 transition-colors"
                  >
                    <div className="flex items-center gap-4">
                      <TagComponent tag={tag} />
                      <div className="text-sm text-muted-foreground">
                        <span>{ticketCount} 个 tickets</span>
                        <span className="mx-2">•</span>
                        <span>创建于 {new Date(tag.created_at).toLocaleDateString('zh-CN')}</span>
                      </div>
                    </div>
                    <div className="flex items-center gap-2">
                      <Button
                        variant="outline"
                        size="sm"
                        onClick={() => handleEdit(tag)}
                        className="h-8 w-8 p-0"
                        title="编辑标签"
                      >
                        <Edit2 className="h-3 w-3" />
                      </Button>
                      <Button
                        variant="outline"
                        size="sm"
                        onClick={() => handleDelete(tag.id)}
                        className="h-8 w-8 p-0 text-destructive hover:text-destructive"
                        title="删除标签"
                      >
                        <Trash2 className="h-3 w-3" />
                      </Button>
                    </div>
                  </div>
                )
              })}
            </div>
          )}
        </CardContent>
      </Card>
    </div>
  )
}