
# Instructions
## project alpha 需求和设计文档
构建一个简单的，使用标签分类和管理 ticket 的工具。它基于 postgres 数据库， 使用 Rust 语言开发后端web， 使用 React/Typescript/Vite/Tailwind/Shadcn 作为前端。无需用户系统，当前用户可以：
- 创建/删除/编辑/完成/取消完成 ticket
- 添加/删除 ticket 的标签
- 按照不同的标签查看 ticket 列表
- 按照 title 搜索 ticket
按照这个想法，帮我详细的生成需求和设计文档，放在./specs/0001-spec.md文件中，输出为中文。

## implementation plan
安装 ./specs/0001-spec.md 中的需求和设计文档，生成一个详细的实现计划， 放在 ./specs/0002-implementation-plan.md 文件中，输出为中文

## phased implementation
按照 ./specs/0002-implementation-plan.md 完整实现这个项目的 phase 1 代码