---
name: fe-theme-palette
description: 主题配色技能：自动挖掘当前项目的 UI 存量配色并生成 openspec/.project-frontend/theme-palette.md；新项目无可挖掘时集成 ui-ux-pro-max 生成设计系统。适用于上游 change 续接与现场开发两条路径，不依赖也不读取 index.md/index.yaml；theme-palette.md 缺失、过期或需求涉及新视觉风格时使用。
---

# fe-theme-palette：主题配色挖掘与模板生成

产出物：`openspec/.project-frontend/theme-palette.md`（配色风格主题模板）。编码阶段的所有颜色决策以该文件为准；自检阶段（fe-self-check）用它做颜色配色维度的对照基线。

本技能是两条开发路径唯一通用的存量挖掘技能。执行时只扫描 UI 样式与目标代码证据，**禁止读取、初始化、修正或回写 `index.md`、`index.yaml`、`index.ymal`**。

## 0. 触发时机

- 阶段 3 编码需要套用配色时发现 theme-palette.md 不存在
- theme-palette.md 仍是占位骨架、来源证据为空或与当前目标模块明显不一致
- 用户要求刷新/重挖配色
- 需求涉及全新视觉风格（须用户确认是否覆盖既有模板）

## 1. 路线判定

按顺序探测**存量配色证据**，决定走哪条路线：

| 情形 | 判定 | 路线 |
|------|------|------|
| 存量项目 | 挖到成体系的颜色定义（变量文件/皮肤定制/≥3 个高频复用色） | 路线 A：挖掘生成 |
| 全新项目 | 工程无样式资产或只有框架默认样式 | 路线 B：ui-ux-pro-max 生成 |
| 半新项目 | 只挖到零星颜色（如仅一个主色） | 路线 A+B：挖掘为准，ui-ux-pro-max 补全，补全项标「建议待确认」 |

## 2. 路线 A：存量项目配色挖掘

按优先级扫描（找到高优先级证据后仍要完成低优先级扫描做交叉验证）：

1. **显式变量**：`:root { --* }` CSS 变量、scss/less 的 variables/theme 文件、tailwind.config 的 colors、Touch UI 皮肤/配色定制文件（`ui0.md`~`ui2.md` 机制对应的定制 css）
2. **组件库皮肤类**：工程内实际使用的 Touch UI 配色类（如 `e_button-blue` 出现频次）→ 判断主操作色系
3. **高频色值统计**：grep 全工程 `#hex / rgb(` 色值按出现频次排序，取 TOP 色聚类（近似色合并），区分：品牌主色、功能色（成功/警告/错误/链接）、中性色阶（文字/背景/边框/分割线）
4. **入口与导航**：入口页/导航栏/主按钮的实际用色（业务对主色的真实选择）

产出规则：每个 token 必须带**来源证据**（文件路径+出现处）；仅出现 1-2 次的色值不入 token，列入「待清理硬编码」清单。

## 3. 路线 B：新项目 · ui-ux-pro-max 集成

1. 从 proposal.md / 用户描述提取产品关键词并译为英文检索词：产品类型（如 business hall / dashboard / admin）+ 行业（telecom / fintech）+ 风格词（professional / minimal）
2. 调用（Windows 用 `python`，macOS/Linux 用 `python3`；`<SKILLS_ROOT>` 由主流程按标准目录探测解析，禁止从 index 文件读取；标准位置 `.trae/skills/agent-frontend-architect`）：

```bash
python <SKILLS_ROOT>/ui-ux-pro-max/scripts/search.py "<product industry keywords>" --design-system -p "<项目名>" -f markdown
```

3. 需要补充细节时用领域检索：`--domain color/typography/style/ux`（用法详见 `<SKILLS_ROOT>/ui-ux-pro-max/SKILL.md`）
4. 将输出的设计系统**转写**为 theme-palette.md（token 化，见第 4 节模板），标注 `source: ui-ux-pro-max`
5. **主色必须经用户确认**后才把 theme-palette.md 标为正式基线；用户不确认前模板标注「草案」。C 级现场开发路径如需项目索引，由后续 fe-project-index 单独维护，本技能不碰 index 文件
6. Python 不可用时降级：读取 `<SKILLS_ROOT>/ui-ux-pro-max/data/colors.csv` 按行业关键词人工检索取色，仍不可行则用 Touch UI 默认配色 + 问询用户主色

## 4. theme-palette.md 模板结构

生成的文件必须遵循 `openspec/.project-frontend/theme-palette.md` 现有模板骨架（随物料包分发的占位模板），核心章节：

1. **设计基调**：风格关键词、适用场景、来源（mined / ui-ux-pro-max / manual）
2. **色彩令牌表**：`brand-primary / brand-secondary`、`func-success / func-warning / func-danger / func-info / func-link`、`text-primary / text-secondary / text-disabled`、`bg-page / bg-card / bg-hover`、`border-strong / border-weak` —— 每行：令牌 | 色值 | 用途 | 来源证据
3. **字体与字号**：字族、层级（标题/正文/辅助）
4. **圆角 / 阴影 / 间距基础**
5. **组件映射（Touch UI）**：主按钮→`e_button-blue`（或工程实际皮肤类）、状态色→messageBox/tip 的语义映射等
6. **亮/暗模式**（如适用）
7. **应用规则**：新代码禁止硬编码色号，颜色一律引用令牌或框架皮肤类；例外须注释说明
8. **未确认项 / 待清理硬编码**

## 5. 编码阶段的消费方式

- 功能骨架完成后统一套色（对齐主提示词的交付优先级），不要边写功能边调色
- Touch UI 工程：优先用框架皮肤类与配色方案定制机制（见技术栈技能 `references/use/ui0.md`~`ui2.md`），自定义 css 只做补充且引用令牌
- 每次交付新增颜色 → 回写进 theme-palette.md 令牌表（新增令牌须用户确认），保持单一事实源
