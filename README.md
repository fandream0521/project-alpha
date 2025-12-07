# Ticket Management System

工单管理系统使用 Rust + React 技术栈构建。

## 技术栈

### 后端
- Rust
- axum - Web框架
- sqlx - 数据库工具
- PostgreSQL - 数据库
- tokio - 异步运行时

### 前端
- React 19
- TypeScript
- Vite - 构建工具
- Tailwind CSS - 样式框架
- Shadcn/ui - UI组件库（后续阶段）

## 项目结构

```
.
├── ticket-backend/     # Rust后端项目
├── ticket-frontend/    # React前端项目
├── docs/              # 文档
├── scripts/           # 部署脚本
└── specs/             # 需求和设计文档
```

## 开发阶段

当前处于阶段2：数据库设计和创建 ✅

### 已完成
- ✅ 创建项目目录结构
- ✅ 配置后端项目依赖
- ✅ 配置前端项目依赖
- ✅ 设置Git仓库
- ✅ 设计数据库表结构
- ✅ 创建数据库迁移文件
- ✅ 实现数据模型
- ✅ 创建数据库连接模块
- ✅ 创建种子数据
- ✅ 测试数据库连接

### 下一阶段
- 后端API实现
- 前端组件开发

## 快速开始

### 1. 环境设置
```bash
# 检查环境并创建配置文件
./scripts/setup.sh

# 编辑 .env 文件，设置数据库连接
nano .env
```

### 2. 数据库设置
```bash
# 初始化数据库
./scripts/init-db.sh

# 插入测试数据（可选）
./scripts/seed-db.sh

# 验证安装
./scripts/test.sh
```

**环境变量配置详情请参考:** [docs/environment-setup.md](docs/environment-setup.md)

**或者手动设置：**
```bash
# 仅运行迁移（通过应用程序）
cd ticket-backend && cargo run

# 然后插入种子数据
./scripts/seed-data.sh
```

### 3. 启动后端
```bash
cd ticket-backend
# 确保 .env 文件存在并配置了 DATABASE_URL
cargo run
```

### 4. 启动前端
```bash
cd ticket-frontend
npm install
npm run dev
```

### 5. 测试数据库连接
```bash
./scripts/test-db.sh
```

## 开发计划

详细的开发计划请参考 [specs/0002-implementation-plan.md](specs/0002-implementation-plan.md)