# 数据库初始化故障排除指南

## 常见问题及解决方案

### 问题1：关系 "tags" 不存在

**错误信息：**
```
psql:scripts/seed-data.sql:15: 错误:  关系 "tags" 不存在
```

**原因：** 数据库表结构还没有创建。

**解决方案：**
```bash
# 方法1：使用完整初始化脚本（推荐）
./scripts/init-database.sh

# 方法2：通过应用程序运行迁移
cd ticket-backend && cargo run

# 然后再插入种子数据
./scripts/seed-data.sh
```

### 问题2：编码错误

**错误信息：**
```
错误:  编码"GBK"的字符0x0xb2 0x0a在编码"UTF8"没有相对应值
```

**原因：** SQL文件编码与数据库编码不匹配。

**解决方案：** 所有SQL文件已修复为UTF-8编码，直接使用：
```bash
./scripts/seed-data.sh
```

### 问题3：DATABASE_URL 环境变量未设置

**错误信息：**
```
✗ DATABASE_URL environment variable is not set
```

**解决方案：**
```bash
# 临时设置
export DATABASE_URL="postgresql://username:password@localhost/ticket_db"

# 或创建 .env 文件
echo "DATABASE_URL=postgresql://username:password@localhost/ticket_db" > .env
```

### 问题4：PostgreSQL 连接失败

**错误信息：**
```
psql: connection to server at "localhost", port 5432 failed
```

**解决方案：**
1. 检查PostgreSQL服务是否运行：
   ```bash
   # Windows
   pg_ctl status

   # Linux/macOS
   brew services list | grep postgresql
   ```

2. 启动PostgreSQL服务：
   ```bash
   # Windows
   pg_ctl start

   # Linux/macOS
   brew services start postgresql
   ```

3. 检查数据库URL格式是否正确。

### 问题5：权限不足

**错误信息：**
```
permission denied for database
```

**解决方案：**
1. 确保用户有创建数据库的权限：
   ```sql
   CREATE USER ticket_user WITH SUPERUSER;
   ```

2. 或使用已有的超级用户账户。

### 问题6：数据库已存在

**错误信息：**
```
database "ticket_db" already exists
```

**解决方案：** 这是正常的，初始化脚本会处理这种情况。继续执行即可。

## 验证步骤

### 1. 检查数据库连接
```bash
psql $DATABASE_URL -c "SELECT version();"
```

### 2. 验证表结构
```bash
psql $DATABASE_URL -c "\dt"
```

### 3. 检查表是否存在
```bash
psql $DATABASE_URL -c "SELECT COUNT(*) FROM information_schema.tables WHERE table_name = 'tags';"
```

### 4. 运行完整测试
```bash
./scripts/test-db.sh
```

## 初始化流程

### 推荐的完整初始化流程：

```bash
# 1. 环境检查
./scripts/setup.sh

# 2. 设置环境变量
export DATABASE_URL="postgresql://username:password@localhost/ticket_db"

# 3. 完整数据库初始化
./scripts/init-database.sh

# 4. 插入种子数据
./scripts/seed-data.sh

# 5. 验证安装
./scripts/test-db.sh
```

### 替代流程（使用应用程序迁移）：

```bash
# 1. 环境检查
./scripts/setup.sh

# 2. 设置环境变量
export DATABASE_URL="postgresql://username:password@localhost/ticket_db"

# 3. 启动应用程序（会自动运行迁移）
cd ticket-backend
cargo run

# 4. 插入种子数据
./scripts/seed-data.sh

# 5. 验证安装
./scripts/test-db.sh
```

## 脚本说明

### init-database.sh
- **功能：** 完整的数据库初始化
- **特点：** 创建数据库、表结构、索引、触发器
- **推荐：** 新项目首次初始化使用

### init-complete-db.sql
- **功能：** 纯SQL的表结构创建
- **用途：** 手动执行或与init-database.sh配合

### seed-data.sh
- **功能：** 插入测试数据
- **前置条件：** 表结构必须已存在
- **特点：** 自动检查表存在性

### test-db.sh
- **功能：** 全面的数据库测试
- **内容：** 连接测试、表结构验证、CRUD操作测试

## 最佳实践

1. **首次使用：** 使用 `init-database.sh` 进行完整初始化
2. **开发调试：** 使用 `test-db.sh` 验证数据库状态
3. **重置数据：** 重新运行 `seed-data.sh` 清除并插入新数据
4. **环境隔离：** 为不同环境使用不同的数据库名称
5. **备份重要：** 在生产环境操作前备份数据库

## 获取帮助

如果遇到其他问题：

1. 检查日志输出中的具体错误信息
2. 验证PostgreSQL服务状态
3. 确认环境变量设置正确
4. 查看相关文档：[scripts-guide.md](scripts-guide.md)
5. 运行 `./scripts/setup.sh` 检查环境配置