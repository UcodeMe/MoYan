# TouchVue2 文档导航

本目录是 TouchVue2 技能包的参考文档库。按“从入门到落地”的阅读路径组织，并补充常见同名主题的选择规则，减少在 `component/`、`element/`、`layout/` 间重复跳转。

## 快速开始（建议先读）

- 框架概览：`use/about.md`
- 基础用法与运行机制：`use/jwc.md`
- 显示比例/响应规则配置：`use/displayScale.md`（以及 `use/ui*.md`）

## 模块划分（用于快速定位）

- `element/`：最小 UI 元素（输入、选择、展示），适合直接放入表单或业务组件内部
- `component/`：由多个元素/逻辑拼装的复合组件（表单、表格、弹窗、上传、导入导出等）
- `layout/`：布局容器与排版能力（列、居中、绝对定位、卷轴、查询区等）
- `logical/`：模板逻辑与局部渲染能力
- `js-api/`：运行时 JS 能力（DOM、AJAX、数据结构、校验、格式化、跳转、客户端信息等）
- `chart/`：图表组件封装
- `mobile/`：Android/iOS 原生交互与 Web 适配
- `faq/`：常见问题与排障经验

## 常用场景速查

- 响应式表单 + 校验：`component/form.md` + `js-api/verify.md` + 相关 `element/*`
- 列表/表格渲染：`component/table.md` + `logical/foreach.md`
- 局部刷新：`js-api/ajax.md` + `logical/part.md`
- 弹窗/对话框：`component/popup.md` / `component/dialog.md` + `js-api/dom.md`
- 上传/导入/导出：`component/upload.md`、`component/import.md`、`component/export.md` + `use/reception-fileUpload.md`、`use/reception-importAndExport.md`
- 多端适配与比例：`use/displayScale.md` + 对应组件/布局的响应规则说明

## 同名主题选择规则（去重指引）

当出现同名或近似主题时，按语义选择主文档：

- `float`：讨论布局浮动/排版时优先看 `layout/float.md`；讨论悬浮层/浮动交互时优先看 `component/float.md` / `component/floatBox.md`
- `space`：讨论基础间距能力时优先看 `element/space.md`；讨论页面布局间距块时优先看 `component/space.md`
- `upload`：如果关注组件 API 与页面用法，看 `component/upload.md` / `component/simpleUpload.md`；如果关注上传链路与接口约定，看 `use/reception-fileUpload.md`
- `import/export`：如果关注组件 API，看 `component/import.md` / `component/export.md`；如果关注整体流程与接口约定，看 `use/reception-importAndExport.md`
