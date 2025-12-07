# 项目结构说明

## 根目录结构

```
project-alpha/
├── ticket-backend/         # Rust 后端项目
├── ticket-frontend/        # React 前端项目
├── docs/                   # 项目文档
├── scripts/               # 部署和设置脚本
├── specs/                 # 需求和设计文档
├── .gitignore            # Git忽略文件配置
└── README.md             # 项目说明文档
```

## 后端项目结构 (ticket-backend/)

```
ticket-backend/
├── Cargo.toml            # Rust项目配置和依赖
├── .env.example          # 环境变量示例
└── src/
    ├── main.rs          # 程序入口点
    ├── lib.rs           # 库文件
    ├── handlers.rs      # API处理器（待实现）
    ├── models.rs        # 数据模型（待实现）
    └── database.rs      # 数据库连接（待实现）
```

### 主要依赖
- axum: Web框架
- tokio: 异步运行时
- sqlx: 数据库工具
- serde: 序列化/反序列化
- chrono: 时间处理
- uuid: UUID生成
- tracing: 日志记录

## 前端项目结构 (ticket-frontend/)

```
ticket-frontend/
├── package.json          # Node.js项目配置和依赖
├── tsconfig.json        # TypeScript配置
├── vite.config.ts       # Vite构建工具配置
├── tailwind.config.js   # Tailwind CSS配置
├── postcss.config.js    # PostCSS配置
├── index.html           # HTML入口文件
├── .env.example         # 环境变量示例
└── src/
    ├── main.tsx         # React应用入口
    ├── App.tsx          # 主应用组件
    ├── index.css        # 全局样式
    └── vite-env.d.ts    # Vite类型定义
```

### 主要依赖
- React 19: UI框架
- TypeScript: 类型系统
- Vite: 构建工具
- Tailwind CSS: 样式框架
- React Router: 路由管理
- TanStack Query: 数据获取

## 脚本目录 (scripts/)

```
scripts/
├── setup.sh             # 环境检查和配置脚本
├── init-db.sh           # 数据库初始化脚本
├── seed-db.sh           # 种子数据插入脚本
├── test.sh              # 系统测试脚本
└── create-tables.sql    # 数据库表创建SQL脚本
```

## 文档目录 (docs/)

```
docs/
└── project-structure.md  # 项目结构说明（本文件）
```

## 规格目录 (specs/)

```
specs/
├── 0001-spec.md          # 需求规格文档
└── 0002-implementation-plan.md  # 实现计划文档
```

## 阶段一完成情况

✅ **已完成的任务：**
1. 项目目录结构创建
2. 后端项目初始化和依赖配置
3. 前端项目初始化和依赖配置
4. Git仓库配置
5. 环境配置文件创建
6. 数据库初始化脚本准备

## 下一步计划

- 阶段二：数据库设计和创建
- 阶段三：后端API实现
- 阶段四：前端组件开发
- 阶段五：功能集成和测试

## 开发环境要求

- Rust 1.70+
- Node.js 18+
- PostgreSQL 14+
- Git