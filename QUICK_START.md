# 快速开始指南

## 一键设置命令

```bash
# 1. 设置环境
./scripts/setup.sh

# 2. 编辑 .env 文件
nano .env

# 3. 初始化数据库
./scripts/init-db.sh

# 4. 插入测试数据
./scripts/seed-db.sh

# 5. 验证系统
./scripts/test.sh

# 6. 启动后端服务
cd ticket-backend && cargo run
```

## 故障排除

### 如果脚本失败：

```bash
# 1. 检查PostgreSQL是否运行
pg_isready

# 2. 手动测试数据库连接
psql "$DATABASE_URL" -c "SELECT version();"

# 3. 创建数据库（如果不存在）
psql -U postgres -c "CREATE DATABASE ticket_db;"
```

### 常见解决方案：

**PostgreSQL未启动：**
- macOS: `brew services start postgresql`
- Linux: `sudo systemctl start postgresql`
- Windows: `pg_ctl start`

**权限问题：**
```bash
# 使用postgres用户
DATABASE_URL=postgresql://postgres:your_password@localhost:5432/ticket_db

# 或授予权限
psql -U postgres -c "GRANT ALL PRIVILEGES ON DATABASE ticket_db TO your_user;"
```

**常见数据库格式：**
- Windows: `postgresql://postgres:password@localhost:5432/ticket_db`
- macOS/Linux: `postgresql://username:password@localhost:5432/ticket_db`
- Docker: `postgresql://postgres:docker@localhost:5432/ticket_db`

## 项目状态

- ✅ 阶段1: 环境搭建和项目初始化
- ✅ 阶段2: 数据库设计和创建
- ⏳ 阶段3: 后端API实现 (下一步)
- ⏳ 阶段4: 前端组件开发