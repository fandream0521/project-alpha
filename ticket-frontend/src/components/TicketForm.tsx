import { useState, useEffect } from 'react'
import { useForm } from 'react-hook-form'
import { zodResolver } from '@hookform/resolvers/zod'
import { z } from 'zod'
import { Ticket, CreateTicketRequest, UpdateTicketRequest } from '@/types'
import { Button } from '@/components/ui/Button'
import { Input } from '@/components/ui/Input'
import { Textarea } from '@/components/ui/Textarea'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/Card'
import { TagSelector } from '@/components/TagSelector'
import { X } from 'lucide-react'

const ticketSchema = z.object({
  title: z.string().min(1, '标题不能为空').max(255, '标题不能超过255个字符'),
  description: z.string().optional(),
  priority: z.enum(['low', 'medium', 'high', 'urgent']).optional(),
  status: z.enum(['open', 'in_progress', 'resolved']).optional(),
})

type TicketFormData = z.infer<typeof ticketSchema>

interface TicketFormProps {
  ticket?: Ticket
  onSubmit: (data: CreateTicketRequest | UpdateTicketRequest) => Promise<void>
  onCancel: () => void
  loading?: boolean
}

export function TicketForm({ ticket, onSubmit, onCancel, loading = false }: TicketFormProps) {
  const [selectedTags, setSelectedTags] = useState<string[]>(ticket?.tags?.map(tag => tag.id) || [])

  const {
    register,
    handleSubmit,
    formState: { errors },
    reset,
  } = useForm<TicketFormData>({
    resolver: zodResolver(ticketSchema),
    defaultValues: {
      title: ticket?.title || '',
      description: ticket?.description || '',
      priority: ticket?.priority || 'medium',
      status: ticket?.status || 'open',
    },
  })

  useEffect(() => {
    if (ticket) {
      reset({
        title: ticket.title,
        description: ticket.description || '',
        priority: ticket.priority,
        status: ticket.status,
      })
      setSelectedTags(ticket.tags?.map(tag => tag.id) || [])
    }
  }, [ticket, reset])

  const handleFormSubmit = async (data: TicketFormData) => {
    try {
      const submitData = {
        ...data,
        tagIds: selectedTags.length > 0 ? selectedTags : undefined,
      }
      await onSubmit(submitData)
      if (!ticket) {
        reset()
        setSelectedTags([])
      }
    } catch (error) {
      // Error is handled by parent component
    }
  }

  
  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center p-4 z-50">
      <Card className="w-full max-w-2xl max-h-[90vh] overflow-y-auto">
        <CardHeader>
          <div className="flex items-center justify-between">
            <CardTitle>
              {ticket ? '编辑 Ticket' : '创建新 Ticket'}
            </CardTitle>
            <Button
              variant="outline"
              size="sm"
              onClick={onCancel}
              disabled={loading}
              className="h-8 w-8 p-0"
            >
              <X className="h-4 w-4" />
            </Button>
          </div>
        </CardHeader>

        <CardContent>
          <form onSubmit={handleSubmit(handleFormSubmit)} className="space-y-4">
            <div>
              <label htmlFor="title" className="block text-sm font-medium mb-1">
                标题 <span className="text-destructive">*</span>
              </label>
              <Input
                id="title"
                placeholder="输入Ticket标题"
                {...register('title')}
                disabled={loading}
                className={errors.title ? 'border-destructive' : ''}
              />
              {errors.title && (
                <p className="mt-1 text-sm text-destructive">{errors.title.message}</p>
              )}
            </div>

            <div>
              <label htmlFor="description" className="block text-sm font-medium mb-1">
                描述
              </label>
              <Textarea
                id="description"
                placeholder="输入Ticket描述（可选）"
                rows={4}
                {...register('description')}
                disabled={loading}
              />
            </div>

            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div>
                <label htmlFor="priority" className="block text-sm font-medium mb-1">
                  优先级
                </label>
                <select
                  id="priority"
                  {...register('priority')}
                  disabled={loading}
                  className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
                >
                  <option value="low">低</option>
                  <option value="medium">中</option>
                  <option value="high">高</option>
                  <option value="urgent">紧急</option>
                </select>
              </div>

              <div>
                <label htmlFor="status" className="block text-sm font-medium mb-1">
                  状态
                </label>
                <select
                  id="status"
                  {...register('status')}
                  disabled={loading}
                  className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
                >
                  <option value="open">待办</option>
                  <option value="in_progress">进行中</option>
                  <option value="resolved">已完成</option>
                </select>
              </div>
            </div>

            <div>
              <label className="block text-sm font-medium mb-2">
                标签
              </label>
              <TagSelector
                selectedTagIds={selectedTags}
                onChange={setSelectedTags}
                placeholder="选择相关标签"
              />
            </div>

            <div className="flex gap-3 pt-4">
              <Button
                type="submit"
                disabled={loading}
                className="flex-1"
              >
                {loading ? '保存中...' : (ticket ? '更新' : '创建')}
              </Button>
              <Button
                type="button"
                variant="outline"
                onClick={onCancel}
                disabled={loading}
                className="flex-1"
              >
                取消
              </Button>
            </div>
          </form>
        </CardContent>
      </Card>
    </div>
  )
}