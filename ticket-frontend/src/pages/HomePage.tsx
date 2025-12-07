import { useState, useEffect, useMemo, useCallback } from 'react'
import { Plus } from 'lucide-react'
import { Ticket } from '@/types'
import { Button } from '@/components/ui/Button'
import { Card, CardContent } from '@/components/ui/Card'
import { TicketCard } from '@/components/TicketCard'
import { TicketForm } from '@/components/TicketForm'
import { AdvancedSearch } from '@/components/AdvancedSearch'
import { useTicketStore } from '@/stores/ticketStore'
import toast from 'react-hot-toast'

export function HomePage() {
  const [showForm, setShowForm] = useState(false)
  const [editingTicket, setEditingTicket] = useState<Ticket | undefined>()
  const [searchFilters, setSearchFilters] = useState({
    search: '',
    status: '',
    priority: '',
    tagIds: [] as string[]
  })

  const {
    tickets,
    tags,
    loading,
    error,
    fetchTickets,
    fetchTags,
    createTicket,
    updateTicketApi,
    deleteTicket,
    clearError,
  } = useTicketStore()

  useEffect(() => {
    const loadData = async () => {
      try {
        await Promise.all([fetchTickets(), fetchTags()])
      } catch (error) {
        // Error handling is done in the store
      }
    }
    loadData()
  }, [fetchTickets, fetchTags])

  useEffect(() => {
    if (error) {
      toast.error(error)
      clearError()
    }
  }, [error, clearError])

  // 使用 useCallback 优化事件处理函数
  const handleCreateTicket = useCallback(async (data: any) => {
    try {
      await createTicket(data)
      setShowForm(false)
      toast.success('Ticket创建成功')
    } catch (error) {
      // Error is already handled in the store
    }
  }, [createTicket])

  const handleUpdateTicket = useCallback(async (data: any) => {
    if (!editingTicket) return

    try {
      await updateTicketApi(editingTicket.id, data)
      setEditingTicket(undefined)
      toast.success('Ticket更新成功')
    } catch (error) {
      // Error is already handled in the store
    }
  }, [editingTicket, updateTicketApi])

  const handleDeleteTicket = useCallback(async (id: string) => {
    if (window.confirm('确定要删除这个Ticket吗？')) {
      try {
        await deleteTicket(id)
        toast.success('Ticket删除成功')
      } catch (error) {
        // Error is already handled in the store
      }
    }
  }, [deleteTicket])

  const handleToggleStatus = useCallback(async (ticket: Ticket) => {
    try {
      if (!ticket || !ticket.id) {
        console.error('Invalid ticket data:', ticket)
        return
      }

      let newStatus: string
      if (ticket.status === 'open') {
        newStatus = 'in_progress'
      } else if (ticket.status === 'in_progress') {
        newStatus = 'resolved'
      } else {
        newStatus = 'open'
      }

      await updateTicketApi(ticket.id, { status: newStatus as any })

      const statusText = newStatus === 'resolved' ? '完成' :
                       newStatus === 'in_progress' ? '开始进行' : '重新打开'
      toast.success(`Ticket已${statusText}`)
    } catch (error) {
      // Error is already handled in the store
    }
  }, [updateTicketApi])

  // 统计数据（基于当前显示的tickets）
  const ticketStats = useMemo(() => ({
    total: tickets.length,
    todo: tickets.filter(t => t.status === 'open').length,
    inProgress: tickets.filter(t => t.status === 'in_progress').length,
    done: tickets.filter(t => t.status === 'resolved').length,
  }), [tickets])

  // 根据搜索过滤器筛选tickets
  const filteredTickets = useMemo(() => {
    console.log('筛选tickets，总数:', tickets.length, '筛选条件:', searchFilters)

    const filtered = tickets.filter((ticket) => {
      // 搜索关键词筛选
      if (searchFilters.search) {
        const searchLower = searchFilters.search.toLowerCase()
        if (!ticket.title.toLowerCase().includes(searchLower) &&
            !ticket.description?.toLowerCase().includes(searchLower)) {
          return false
        }
      }

      // 状态筛选
      if (searchFilters.status && ticket.status !== searchFilters.status) {
        console.log(`状态筛选失败: ticket.status=${ticket.status}, filter=${searchFilters.status}`)
        return false
      }

      // 优先级筛选
      if (searchFilters.priority && ticket.priority !== searchFilters.priority) {
        console.log(`优先级筛选失败: ticket.priority=${ticket.priority}, filter=${searchFilters.priority}`)
        return false
      }

      // 标签筛选
      if (searchFilters.tagIds.length > 0) {
        const ticketTagIds = ticket.tags?.map(tag => tag.id) || []
        const hasMatchingTag = searchFilters.tagIds.some(tagId => ticketTagIds.includes(tagId))
        if (!hasMatchingTag) {
          console.log(`标签筛选失败: ticket标签=${ticketTagIds}, 筛选标签=${searchFilters.tagIds}`)
          return false
        }
      }

      return true
    })

    console.log('筛选后数量:', filtered.length)
    return filtered
  }, [tickets, searchFilters])

  // 处理搜索过滤器变化
  const handleFiltersChange = useCallback((filters: typeof searchFilters) => {
    console.log('HomePage接收到筛选器变化:', filters)
    setSearchFilters(filters)
  }, [])

  return (
    <div className="max-w-6xl mx-auto px-4">
        {/* Header */}
        <div className="mb-8">
          <h1 className="text-3xl font-bold mb-4">Ticket管理系统</h1>

          {/* Stats Cards */}
          <div className="grid grid-cols-1 md:grid-cols-4 gap-4 mb-6">
            <Card>
              <CardContent className="p-4">
                <div className="text-2xl font-bold">{ticketStats.total}</div>
                <div className="text-sm text-muted-foreground">总Tickets</div>
              </CardContent>
            </Card>
            <Card>
              <CardContent className="p-4">
                <div className="text-2xl font-bold">{ticketStats.todo}</div>
                <div className="text-sm text-muted-foreground">待办</div>
              </CardContent>
            </Card>
            <Card>
              <CardContent className="p-4">
                <div className="text-2xl font-bold">{ticketStats.inProgress}</div>
                <div className="text-sm text-muted-foreground">进行中</div>
              </CardContent>
            </Card>
            <Card>
              <CardContent className="p-4">
                <div className="text-2xl font-bold">{ticketStats.done}</div>
                <div className="text-sm text-muted-foreground">已完成</div>
              </CardContent>
            </Card>
          </div>

          {/* Advanced Search and Create */}
          <div className="flex gap-4">
            <AdvancedSearch
              tags={tags}
              onFiltersChange={handleFiltersChange}
              loading={loading}
            />
            <Button
              onClick={() => setShowForm(true)}
              className="flex items-center gap-2"
              disabled={loading}
            >
              <Plus className="h-4 w-4" />
              创建Ticket
            </Button>
          </div>
        </div>

        {/* Loading State */}
        {loading && (
          <div className="text-center py-12">
            <div className="text-muted-foreground">加载中...</div>
          </div>
        )}

        {/* Ticket List */}
        {!loading && (
          <div className="space-y-4">
            {filteredTickets.length === 0 ? (
              <Card>
                <CardContent className="text-center py-12">
                  <p className="text-muted-foreground">
                    {tickets.length === 0 ? '还没有Tickets，创建一个吧！' : '没有找到符合条件的Tickets'}
                  </p>
                </CardContent>
              </Card>
            ) : (
              filteredTickets.map((ticket) => (
                <TicketCard
                  key={ticket.id}
                  ticket={ticket}
                  onEdit={setEditingTicket}
                  onDelete={handleDeleteTicket}
                  onToggleStatus={handleToggleStatus}
                />
              ))
            )}
          </div>
        )}

        {/* Forms */}
        {showForm && (
          <TicketForm
            onSubmit={handleCreateTicket}
            onCancel={() => setShowForm(false)}
            loading={loading}
          />
        )}

        {editingTicket && (
          <TicketForm
            ticket={editingTicket}
            onSubmit={handleUpdateTicket}
            onCancel={() => setEditingTicket(undefined)}
            loading={loading}
          />
        )}
      </div>
  )
}