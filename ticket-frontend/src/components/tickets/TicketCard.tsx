import { Ticket } from '@/types'
import { formatDistanceToNow } from 'date-fns'
import { zhCN } from 'date-fns/locale'
import { Button } from '@/components/ui/Button'
import { Badge } from '@/components/ui/Badge'
import { Card, CardContent } from '@/components/ui/Card'
import { Check, Edit2, Trash2, RotateCcw } from 'lucide-react'
import { memo, useMemo, useCallback } from 'react'

interface TicketCardProps {
  ticket: Ticket
  onEdit: (ticket: Ticket) => void
  onDelete: (id: string) => void
  onToggleStatus: (ticket: Ticket) => void
}

const TicketCard = memo(({ ticket, onEdit, onDelete, onToggleStatus }: TicketCardProps) => {
  // 添加防御性空值检查
  if (!ticket) {
    return null
  }

  // 使用 useMemo 缓存计算结果
  const isCompleted = useMemo(() => ticket.status === 'resolved', [ticket.status])

  // 缓存优先级和状态的样式映射
  const priorityVariant = useMemo((): "default" | "destructive" | "outline" | "secondary" => {
    if (!ticket.priority) return 'outline'
    const variants: Record<string, "default" | "destructive" | "outline" | "secondary"> = {
      'urgent': 'destructive',
      'high': 'destructive',
      'medium': 'default',
      'low': 'secondary'
    }
    return variants[ticket.priority] || 'outline'
  }, [ticket.priority])

  const statusVariant = useMemo((): "default" | "destructive" | "outline" | "secondary" => {
    if (!ticket.status) return 'outline'
    const variants: Record<string, "default" | "destructive" | "outline" | "secondary"> = {
      'resolved': 'default',
      'in_progress': 'secondary',
      'open': 'outline'
    }
    return variants[ticket.status] || 'outline'
  }, [ticket.status])

  const priorityText = useMemo(() => {
    if (!ticket.priority) return '未知'
    const texts: Record<string, string> = {
      'urgent': '紧急',
      'high': '高',
      'medium': '中',
      'low': '低'
    }
    return texts[ticket.priority] || ticket.priority
  }, [ticket.priority])

  const statusText = useMemo(() => {
    if (!ticket.status) return '未知'
    const texts: Record<string, string> = {
      'resolved': '已完成',
      'in_progress': '进行中',
      'open': '待办'
    }
    return texts[ticket.status] || ticket.status
  }, [ticket.status])

  // 缓存时间格式化
  const createdAtFormatted = useMemo(() => {
    return formatDistanceToNow(new Date(ticket.created_at), {
      addSuffix: true,
      locale: zhCN
    })
  }, [ticket.created_at])

  // 使用 useCallback 避免重复创建函数
  const handleToggleStatus = useCallback(() => {
    if (ticket && ticket.id) {
      onToggleStatus(ticket)
    }
  }, [ticket, onToggleStatus])

  const handleEdit = useCallback(() => {
    if (ticket && ticket.id) {
      onEdit(ticket)
    }
  }, [ticket, onEdit])

  const handleDelete = useCallback(() => {
    if (ticket?.id) {
      onDelete(ticket.id)
    }
  }, [ticket.id, onDelete])

  return (
    <Card className="hover:shadow-md transition-shadow">
      <CardContent className="p-4">
        <div className="flex items-start justify-between">
          <div className="flex-1 min-w-0">
            <div className="flex items-center gap-2 mb-2">
              <h3 className={`text-lg font-medium truncate ${
                isCompleted ? 'line-through text-muted-foreground' : 'text-foreground'
              }`}>
                {ticket.title}
              </h3>
              <Badge variant={priorityVariant} className="shrink-0">
                {priorityText}
              </Badge>
            </div>

            {ticket.description && (
              <p className="text-sm text-muted-foreground mb-3 line-clamp-2">
                {ticket.description}
              </p>
            )}

            <div className="flex items-center gap-3 text-xs text-muted-foreground mb-3">
              <Badge variant={statusVariant} className="text-xs">
                {statusText}
              </Badge>
              <span>
                创建于 {createdAtFormatted}
              </span>
            </div>

            {ticket.tags && ticket.tags.length > 0 && (
              <div className="flex flex-wrap gap-1">
                {ticket.tags.map((tag) => (
                  <Badge
                    key={tag.id}
                    variant="outline"
                    className="text-xs"
                    style={{ backgroundColor: tag.color, borderColor: tag.color }}
                  >
                    {tag.name}
                  </Badge>
                ))}
              </div>
            )}
          </div>

          <div className="flex items-center gap-1 ml-4">
            <Button
              variant="outline"
              size="sm"
              onClick={handleToggleStatus}
              title={isCompleted ? '重新打开' : '标记完成'}
              className="h-8 w-8 p-0"
            >
              {isCompleted ? <RotateCcw className="h-3 w-3" /> : <Check className="h-3 w-3" />}
            </Button>
            <Button
              variant="outline"
              size="sm"
              onClick={handleEdit}
              title="编辑"
              className="h-8 w-8 p-0"
            >
              <Edit2 className="h-3 w-3" />
            </Button>
            <Button
              variant="outline"
              size="sm"
              onClick={handleDelete}
              title="删除"
              className="h-8 w-8 p-0 text-destructive hover:text-destructive"
            >
              <Trash2 className="h-3 w-3" />
            </Button>
          </div>
        </div>
      </CardContent>
    </Card>
  )
})

TicketCard.displayName = 'TicketCard'

export { TicketCard }