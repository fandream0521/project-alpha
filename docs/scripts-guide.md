# 项目脚本使用指南

本项目提供了一系列Bash脚本来自动化常见的开发任务。

## 脚本列表

### 1. setup.sh - 环境设置脚本
检查开发环境是否满足项目要求。

```bash
./scripts/setup.sh
```

**检查内容：**
- Rust 工具链
- Node.js 环境
- PostgreSQL 客户端
- Git 版本控制

### 2. init-database.sh - 完整数据库初始化脚本
创建数据库和完整的表结构（推荐使用）。

```bash
./scripts/init-database.sh
```

**功能：**
- 创建数据库（如果不存在）
- 创建完整的表结构
- 创建索引和触发器
- 验证表创建成功

### 3. init-complete-db.sql - 完整数据库结构SQL脚本
包含创建所有表、索引和触发器的SQL语句。

```bash
psql $DATABASE_URL -f ./scripts/init-complete-db.sql
```

### 2. init-db.sql - 数据库初始化脚本
创建数据库和基础表结构。

```bash
psql $DATABASE_URL -f ./scripts/init-db.sql
```

**功能：**
- 创建数据库（如果不存在）
- 创建基础表结构框架
- 设置用户权限（可选）

### 3. seed-data.sql - 种子数据SQL脚本
包含测试和开发环境的示例数据。

```bash
psql $DATABASE_URL -f ./scripts/seed-data.sql
```

**包含数据：**
- 10个预设标签（bug、feature等）
- 8个示例工单
- 16个工单-标签关联
- 9条示例评论

### 4. seed-data.sh - 种子数据执行脚本
自动化种子数据插入过程。

```bash
./scripts/seed-data.sh
```

**功能：**
- 环境变量检查
- 执行种子数据SQL脚本
- 显示插入结果统计

### 5. test-db.sh - 数据库测试脚本
全面的数据库连接和功能测试。

```bash
./scripts/test-db.sh
```

**测试内容：**
- PostgreSQL 连接测试
- 数据库表结构验证
- Rust应用程序数据库操作测试
- 数据模型CRUD功能验证

## 使用流程

### 首次设置
```bash
# 1. 检查环境
./scripts/setup.sh

# 2. 完整数据库初始化（推荐）
./scripts/init-database.sh

# 3. 插入测试数据
./scripts/seed-data.sh

# 4. 验证安装
./scripts/test-db.sh

# 5. 启动应用
cd ticket-backend && cargo run
```

**替代方案（使用应用程序迁移）：**
```bash
# 1. 检查环境
./scripts/setup.sh

# 2. 启动应用（会自动运行迁移）
cd ticket-backend && cargo run

# 3. 插入测试数据
./scripts/seed-data.sh

# 4. 验证安装
./scripts/test-db.sh
```

### 开发调试
```bash
# 测试数据库连接
./scripts/test-db.sh

# 重置测试数据
psql $DATABASE_URL -f ./scripts/seed-data.sql
```

## 环境变量

脚本需要以下环境变量：

- `DATABASE_URL`: PostgreSQL数据库连接字符串
  ```bash
  export DATABASE_URL="postgresql://username:password@localhost/ticket_db"
  ```

- `DATABASE_MAX_CONNECTIONS`: 最大连接数（可选，默认10）
  ```bash
  export DATABASE_MAX_CONNECTIONS="20"
  ```

## 脚本特性

### 跨平台兼容
- 使用标准Bash语法
- 兼容Linux、macOS、Windows（WSL/Git Bash）

### 错误处理
- 环境变量验证
- 数据库连接检查
- 详细的错误信息输出

### 用户友好
- 彩色输出提示
- 进度状态显示
- 清晰的成功/失败反馈

## 故障排除

### 权限问题
```bash
# 确保脚本有执行权限
chmod +x scripts/*.sh
```

### 环境变量未设置
```bash
# 设置临时环境变量
export DATABASE_URL="postgresql://user:pass@localhost/db"

# 或创建 .env 文件
echo "DATABASE_URL=postgresql://user:pass@localhost/db" > .env
```

### PostgreSQL连接失败
1. 确认PostgreSQL服务正在运行
2. 检查数据库URL格式是否正确
3. 验证用户名和密码
4. 确认数据库存在

### 脚本执行失败
1. 检查Bash是否安装：`bash --version`
2. 确认脚本路径正确
3. 查看详细错误信息
4. 逐步手动执行排查问题

## 最佳实践

1. **开发前**：先运行 `setup.sh` 检查环境
2. **数据库变更后**：运行 `test-db.sh` 验证
3. **测试环境**：使用 `seed-data.sh` 快速准备数据
4. **持续集成**：在CI/CD中集成这些脚本
5. **文档更新**：修改脚本后同步更新文档