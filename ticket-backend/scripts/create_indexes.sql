-- 数据库性能优化索引脚本
-- Phase 7: 搜索优化与部署

-- 为 tickets 表创建索引
CREATE INDEX IF NOT EXISTS idx_tickets_created_at ON tickets(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_tickets_status ON tickets(status);
CREATE INDEX IF NOT EXISTS idx_tickets_priority ON tickets(priority);
CREATE INDEX IF NOT EXISTS idx_tickets_title ON tickets USING gin(to_tsvector('english', title));
CREATE INDEX IF NOT EXISTS idx_tickets_description ON tickets USING gin(to_tsvector('english', description));

-- 为 tags 表创建索引
CREATE INDEX IF NOT EXISTS idx_tags_name ON tags(name);
CREATE INDEX IF NOT EXISTS idx_tags_created_at ON tags(created_at);

-- 为 ticket_tags 关联表创建索引
CREATE INDEX IF NOT EXISTS idx_ticket_tags_ticket_id ON ticket_tags(ticket_id);
CREATE INDEX IF NOT EXISTS idx_ticket_tags_tag_id ON ticket_tags(tag_id);
CREATE INDEX IF NOT EXISTS idx_ticket_tags_composite ON ticket_tags(ticket_id, tag_id);

-- 为 comments 表创建索引（如果存在）
CREATE INDEX IF NOT EXISTS idx_comments_ticket_id ON comments(ticket_id);
CREATE INDEX IF NOT EXISTS idx_comments_created_at ON comments(created_at);

-- 复合索引用于常见查询组合
CREATE INDEX IF NOT EXISTS idx_tickets_status_created_at ON tickets(status, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_tickets_priority_created_at ON tickets(priority, created_at DESC);

-- 分析表以更新统计信息
ANALYZE tickets;
ANALYZE tags;
ANALYZE ticket_tags;
ANALYZE comments;

-- 查看索引创建结果
SELECT
    schemaname,
    tablename,
    indexname,
    indexdef
FROM pg_indexes
WHERE schemaname = 'public'
    AND tablename IN ('tickets', 'tags', 'ticket_tags', 'comments')
ORDER BY tablename, indexname;