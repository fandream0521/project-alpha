# 环境配置指南

## 概述

本项目需要配置数据库连接和其他环境变量才能正常运行。本指南将帮助您快速设置环境。

## 快速开始

### 1. 创建环境配置文件

```bash
# 在项目根目录创建 .env 文件
cp .env.example .env

# 编辑 .env 文件，填入您的数据库配置
nano .env  # 或使用您喜欢的编辑器
```

### 2. 配置数据库连接

在 `.env` 文件中，更新 `DATABASE_URL`：

```bash
# 格式: postgresql://username:password@host:port/database
DATABASE_URL=postgresql://your_username:your_password@localhost:5432/ticket_db
```

## 数据库配置示例

### Windows PostgreSQL

```bash
DATABASE_URL=postgresql://postgres:your_password@localhost:5432/ticket_db
```

### macOS (Homebrew)

```bash
DATABASE_URL=postgresql://your_username@localhost/ticket_db
```

### Linux

```bash
DATABASE_URL=postgresql://your_username:your_password@localhost/ticket_db
```

### Docker PostgreSQL

```bash
DATABASE_URL=postgresql://postgres:docker@localhost:5432/ticket_db
```

### 云数据库 (AWS RDS, Google Cloud SQL等)

```bash
# AWS RDS PostgreSQL
DATABASE_URL=postgresql://username:password@your-rds-endpoint.rds.amazonaws.com:5432/ticket_db

# Google Cloud SQL
DATABASE_URL=postgresql://username:password@your-cloud-sql-ip:5432/ticket_db
```

## 完整环境变量说明

### 必需配置

```bash
# 数据库连接 (必需)
DATABASE_URL=postgresql://username:password@localhost/ticket_db
```

### 可选配置

```bash
# 服务器配置
HOST=0.0.0.0                    # 服务器监听地址
PORT=3000                       # 服务器端口

# 数据库连接池
DATABASE_MAX_CONNECTIONS=10     # 最大数据库连接数

# 日志配置
RUST_LOG=debug                  # 日志级别: debug, info, warn, error

# JWT配置 (后续阶段)
JWT_SECRET=your-secret-key-change-this-in-production
JWT_EXPIRES_IN=24h

# 前端配置
VITE_API_URL=http://localhost:3000
VITE_APP_TITLE=Ticket Management System
VITE_APP_VERSION=0.1.0
```

## 配置步骤详解

### 步骤1: 安装PostgreSQL

如果您还没有安装PostgreSQL：

**Windows:**
1. 从 [postgresql.org](https://www.postgresql.org/download/windows/) 下载安装
2. 记住安装时设置的密码

**macOS:**
```bash
brew install postgresql
brew services start postgresql
```

**Ubuntu/Debian:**
```bash
sudo apt update
sudo apt install postgresql postgresql-contrib
sudo systemctl start postgresql
```

### 步骤2: 创建数据库用户和数据库

```sql
-- 连接到PostgreSQL
psql -U postgres

-- 创建用户 (替换 'your_username' 和 'your_password')
CREATE USER your_username WITH PASSWORD 'your_password';

-- 创建数据库
CREATE DATABASE ticket_db OWNER your_username;

-- 授予权限
GRANT ALL PRIVILEGES ON DATABASE ticket_db TO your_username;

-- 退出
\q
```

### 步骤3: 更新.env文件

```bash
# 创建.env文件
cp .env.example .env

# 编辑文件
nano .env
```

更新以下行：
```bash
DATABASE_URL=postgresql://your_username:your_password@localhost:5432/ticket_db
```

### 步骤4: 验证配置

```bash
# 测试数据库连接
./scripts/test-db.sh

# 如果连接成功，初始化数据库
./scripts/init-database.sh
```

## 常见问题解决

### 问题1: 认证失败

**错误:** `FATAL: password authentication failed for user`

**解决方案:**
1. 检查用户名和密码是否正确
2. 确保PostgreSQL服务正在运行
3. 检查pg_hba.conf配置文件

### 问题2: 连接被拒绝

**错误:** `connection to server at "localhost", port 5432 failed`

**解决方案:**
1. 启动PostgreSQL服务
2. 检查端口是否正确 (默认5432)
3. 检查防火墙设置

### 问题3: 数据库不存在

**错误:** `FATAL: database "ticket_db" does not exist`

**解决方案:**
```bash
# 连接到PostgreSQL
psql -U postgres

# 创建数据库
CREATE DATABASE ticket_db;

# 退出
\q
```

### 问题4: 权限不足

**错误:** `permission denied for database ticket_db`

**解决方案:**
```sql
-- 授予权限
GRANT ALL PRIVILEGES ON DATABASE ticket_db TO your_username;

-- 如果需要超级用户权限
ALTER USER your_username SUPERUSER;
```

## 开发环境最佳实践

### 1. 使用环境特定的配置

```bash
# 开发环境
DATABASE_URL=postgresql://dev_user:dev_pass@localhost:5432/ticket_dev

# 测试环境
DATABASE_URL=postgresql://test_user:test_pass@localhost:5432/ticket_test

# 生产环境
DATABASE_URL=postgresql://prod_user:secure_pass@prod-host:5432/ticket_prod
```

### 2. 使用Docker (可选)

```bash
# 启动PostgreSQL Docker容器
docker run --name ticket-postgres \
  -e POSTGRES_USER=your_username \
  -e POSTGRES_PASSWORD=your_password \
  -e POSTGRES_DB=ticket_db \
  -p 5432:5432 \
  -d postgres:15

# 然后在.env中使用
DATABASE_URL=postgresql://your_username:your_password@localhost:5432/ticket_db
```

### 3. 环境变量管理

```bash
# 在shell配置文件中设置 (~/.bashrc, ~/.zshrc等)
export DATABASE_URL="postgresql://your_username:your_password@localhost:5432/ticket_db"

# 或者使用direnv (推荐)
echo "DATABASE_URL=postgresql://your_username:your_password@localhost:5432/ticket_db" > .envrc
direnv allow
```

## 安全注意事项

1. **永远不要** 将包含真实密码的 `.env` 文件提交到版本控制
2. **使用强密码** 并定期更换
3. **限制数据库用户权限** ，不要在生产环境使用超级用户
4. **使用SSL连接** 在生产环境中
5. **定期备份** 数据库

## 验证安装

配置完成后，运行以下命令验证：

```bash
# 1. 检查环境
./scripts/setup.sh

# 2. 测试数据库连接
./scripts/test-db.sh

# 3. 初始化数据库
./scripts/init-database.sh

# 4. 插入种子数据
./scripts/seed-data.sh
```

如果所有命令都成功执行，您的环境配置就完成了！