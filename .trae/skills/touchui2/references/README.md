# TouchUI2 文档导航

本目录是 TouchUI2 技能包的参考文档库。按“从入门到落地”的阅读路径组织，并补充常见同名主题的选择规则，避免在 `component/`、`element/`、`layout/` 间来回跳转。

## 快速开始（建议先读）

- 框架概览：`use/about.md`
- JWC 使用与运行机制：`use/jwc.md`
- 显示比例/矢量化配置：`use/displayScale.md`（以及 `use/ui*.md`）

## 模块划分（用于快速定位）

- `element/`：最小 UI 元素（输入、选择、展示），通常可以直接放在表单或组件内部使用
- `component/`：由多个元素/逻辑拼装的复合组件（表单、表格、弹窗、上传、导入导出等），通常承载完整业务交互
- `layout/`：纯布局容器与排版能力（列、居中、绝对定位、卷轴、查询区等），用于组织页面结构
- `logical/`：模板逻辑与局部渲染（`@Foreach`、`@Part`、conditional 等）
- `js-api/`：运行时 JS 能力（DOM、AJAX、数据结构、校验、格式化、跳转、客户端信息等）
- `chart/`：ECharts 封装图表组件
- `mobile/`：Android/iOS 原生交互与 Web 适配
- `faq/`：常见问题与排障经验

## 常用场景速查

- 响应式表单 + 校验：`component/form.md` + `js-api/verify.md` + 相关 `element/*`
- 列表/表格渲染：`component/table.md` + `logical/foreach.md`（必要时配合 `js-api/data.md`）
- 局部刷新：`js-api/ajax.md` + `logical/part.md`
- 弹窗/对话框：`component/popup.md` / `component/dialog.md` + `js-api/dom.md`
- 上传/导入/导出：`component/upload.md`、`component/import.md`、`component/export.md` + `use/reception-fileUpload.md`、`use/reception-importAndExport.md`
- 多端适配与比例：`use/displayScale.md` + 对应组件/布局的响应规则说明

## 同名主题选择规则（去重指引）

当出现同名或近似主题时，按语义选择主文档：

- `float`：如果讨论“布局浮动/排版”，优先看 `layout/float.md`；如果讨论“浮动层/悬浮交互”，优先看 `component/float.md` / `component/floatBox.md`（以具体目标为准）
- `space`：如果讨论“基础间距/小控件”，优先看 `element/space.md`；如果讨论“页面间距组件/布局块”，优先看 `component/space.md`
- `upload`：如果讨论组件 API 与页面用法，看 `component/upload.md` / `component/simpleUpload.md`；如果讨论“前台上传链路/接口约定”，看 `use/reception-fileUpload.md`
- `import/export`：如果讨论组件 API，看 `component/import.md` / `component/export.md`；如果讨论“导入导出整体流程与接口约定”，看 `use/reception-importAndExport.md`

## 端到端示例与自测

- 端到端示例：`examples/e2e-query-table-part.md`
- 自测用例：`tests/skill-e2e-cases.md`

