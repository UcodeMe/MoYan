---
name: fe-project-index
description: 仅供无上游 change 产物的 C 级现场开发路径使用的项目索引技能：维护 openspec/.project-frontend/index.md 与 index.yaml，提供首次初始化挖掘、场景参考查询和交付后增量回写。A/B 级续接需求/技术设计分身产物时严禁触发或读取本技能。
---

# fe-project-index：项目索引挖掘 / 查询 / 回写

> **硬性适用边界**：仅 C 级“无上游产物 / 现场对齐”路径可以使用本技能。一旦主流程判定为 A/B 级续接路径，立即停止，不得读取、创建、查询、修正或回写 `index.md`、`index.yaml`、`index.ymal`；也不得把本技能作为组件库、接口、场景或技能根目录的间接信息源。UI 存量配色请独立使用 fe-theme-palette。

`openspec/.project-frontend/` 是前端开发分身在目标工程内的导航产物目录，由本技能维护的**文件族**：

| 文件 | 角色 |
|------|------|
| `index.md` | **项目框架概要，第一入口**：项目标识、组件库清单（在用组件库 + 各自技能，技术栈关键字段）、工程约定速览、产物索引（索引 index.yaml 与 theme-palette.md） |
| `index.yaml` | **结构化导航**：技术栈证据、接口 api 声明位置、路由、场景参考列表（场景名→entry）、配色指针、术语表 |
| `theme-palette.md` | 配色令牌（由 fe-theme-palette 技能生成维护，本技能只负责在 index.md/index.yaml 中索引它） |

本技能定义它们的格式、首次挖掘流程、查询决策与回写规则。

## 0. 文件定位与兼容

- 标准路径：`<工程根>/openspec/.project-frontend/index.md` 与 `index.yaml`
- 兼容规则：若该目录下只存在 `index.ymal`（历史拼写），直接使用并**沿用该文件名**回写，不要另建新文件造成双份索引
- 只有 index.yaml 没有 index.md（旧版工程）→ 从 index.yaml 生成 index.md 概要，经用户确认后落盘
- 目录不存在 → 创建目录后进入首次初始化挖掘
- **一致性规则**：index.md 是概要与路由入口，结构化数据以 index.yaml 为准；两处不一致 → 以 index.yaml 为准并问询用户修正 index.md；「组件库清单」以 index.yaml `tech_stack.component_libs[]` 为权威（每库 `skill` 字段给出对应技能名），index.md 第 2 节列出同清单，两处保持一致

## 1. 格式规范

### 1.1 index.md（项目框架概要）

章节固定为：**1 项目标识**（名称/一句话定位/工程根特征）、**2 组件库清单**（在用组件库：每库 name + 对应技能路径 + 判定依据 + 工程形态备注；可多库共存）、**3 工程约定速览**（页面目录/接口声明摘要/路由菜单/构建命令）、**4 产物索引**（index.yaml、theme-palette.md、self-check/、dev-notes/ 的路径与用途表）、**5 一致性规则**。骨架以随包分发的 `openspec/.project-frontend/index.md` 占位模板为准。

### 1.2 index.yaml（结构化导航）

```yaml
meta:
  version: 1
  initialized: true            # false = 尚未完成首次挖掘
  project_name: ""             # 项目名
  updated_at: ""               # 最近回写日期 YYYY-MM-DD
  skills_root: ""              # 技能根目录（相对工程根），标准位置 .trae/skills/agent-frontend-architect

tech_stack:
  framework: unknown           # 基础运行框架（与组件库正交）：vue2 | vue3 | react | unknown
  component_libs:              # 在用组件库清单（储备，可多个/可混合）。纯原生工程留空数组
    - name: touchui2           # touchui2=TouchUI2(touch/JWC) | touchvue2=TouchVue2(touch2) | element-ui | ant-design-vue | vant | ...
      skill: touchui2          # 对应 <SKILLS_ROOT> 下技能名；无专属技能留空→生成时遵循工程既有用法
      present: true
      evidence: []             # 该库的判定证据（文件路径+特征）
  entry_html: ""               # 入口页面/模板路径
  assets_root: ""              # Touch UI 静态资源根（如 TouchUI/）

api:
  style: ""                    # 集中式 | 按模块分散 | mock | 页面内联
  declaration_paths: []        # 接口 api 声明位置（glob 或目录），如 src/api/**
  mock_paths: []               # mock 数据位置
  request_wrapper: ""          # 请求封装入口（如 $.ajax.submit / src/utils/request.js）
  conventions: ""              # 命名与返回结构约定（一句话）

routing:
  config_paths: []             # 路由/菜单配置文件位置
  pattern: ""                  # 页面接入方式：路由注册 | 静态html直链 | iframe挂载

scenes:                        # 场景参考列表 —— 前端开发时的主体导航
  - name: 营业厅融合业务办理流程    # 业务场景名（用业务口径命名）
    entry: src/views/hall/fusion-accept.vue   # 场景入口文件
    routes: [/hall/fusion]     # 相关路由
    apis: [queryFusionOffer, submitFusionOrder]  # 涉及接口
    components: []             # 场景内沉淀的可复用组件
    notes: 多步骤表单+提交回执   # 一句话特征（供关键词匹配）
    status: example            # example=示例占位 | mined=挖掘发现 | delivered=分身交付
    change_id: ""              # 关联 openspec change（delivered 时填写）

theme:
  palette_file: openspec/.project-frontend/theme-palette.md
  source: ""                   # mined | ui-ux-pro-max | manual

glossary: []                   # 业务术语↔代码命名对照，如 "融合业务: fusion"
```

## 2. 首次初始化挖掘（meta.initialized 非 true 或文件不存在时）

按以下顺序挖掘，每步记录证据；**挖掘只读不改工程代码**：

1. **工程识别**：读 package.json（name/依赖）、根目录结构 → project_name、框架线索
2. **组件库清单判定**（组件库是储备，工程可同时用多个库、可混合；结果写入 index.yaml `tech_stack.component_libs[]`，每库记 name/skill/present/evidence，并同步 index.md 第 2 节）：
   - base 框架（`framework`）：按 `package.json`（vue@2.x / vue@3.x / react）或入口形态判定 vue2/vue3/react/unknown
   - touchui2（TouchUI2/touch）：入口 html 引 `TouchUI/js/jcl*.js`、模板含 `jwcid=`/`ognl:`、类名 `c_*`/`e_*`/`l_*`、`assets/js/config.js` → 命中即记一条 `skill: touchui2`
   - touchvue2（TouchVue2/touch2）：vue@2.x + `.vue` 页面 + Touch UI 类名/资源 → 命中即记一条 `skill: touchvue2`
   - 其他库（element-ui / ant-design-vue / vant 等）：按 `package.json` 依赖探测，命中即记一条 `skill: ""`（无专属技能，生成时遵循工程既有用法）
   - **多库共存合法**：上述多组证据同时命中 → 各记一条、不强制二选一；都不充分且无配置 → `component_libs` 留空数组（纯原生工程），`framework` 仍尽量判定，拿不准列入待确认项问询用户
3. **接口声明位置**：依次探测 `src/api/`、`src/apis/`、`src/services/`、`api/`、`assets/js/data.js`（TouchUI mock 惯例）、grep 请求封装调用点（`$.ajax.submit` / axios / fetch）→ api.style / declaration_paths / request_wrapper
4. **路由与菜单**：探测 `src/router/`、`router.js`、菜单配置 json/js、静态 html 清单 → routing
5. **场景发现**：遍历 views/pages 目录（或静态 html 清单），结合路由表与菜单名，为每个页面组生成候选场景条目（name 用目录名/菜单名的业务口径，status: mined）；一个场景 = 一个业务闭环（列表+详情+办理流程归入同一场景）
6. **主题线索**：探测 css 变量 / scss variables / Touch UI 皮肤定制文件 → 只记录线索，配色模板生成交给 fe-theme-palette 技能
7. **产出与确认**：组装候选 **index.md + index.yaml**（概要与结构化两份同时生成、内容对应；`meta.skills_root` 填入当前已解析的技能根目录，标准位置 `.trae/skills/agent-frontend-architect`），向用户展示摘要（组件库清单 component_libs + 证据、api 位置、场景清单）并标注拿不准的项（尤其**场景命名**是业务口径，机器猜测容易失准）；用户确认/修正后两份一并写盘，`initialized: true`，删除 example 占位条目
8. **空工程/新项目**：无可挖掘内容 → 写入两份骨架（initialized: true，scenes 为空，component_libs 为空、组件库随用随记），告知用户场景将随交付逐步沉淀

## 3. 查询决策（开发前的场景参考挖掘）

1. 提取本次需求关键词：业务名、页面名、路由词干、接口名（结合 glossary 做术语归一）
2. 与 `scenes[]` 的 `name / notes / routes / apis` 匹配（status 为 example 的条目不参与匹配）：

| 匹配结果 | 行为 |
|----------|------|
| 唯一明确命中 | **直接采用**：读 entry 及关联文件作参考基线，声明「参考场景：<name>」后继续，不问询 |
| 多个候选（无法从需求判定） | **列表问询**：每项一句差异说明 + 建议项，等用户确定 |
| 无命中 | **直接生成**：告知「未找到可参考场景，按上游产物直接开发」，不阻塞 |

3. 同时用 `api.declaration_paths` 定位本次涉及接口的既有声明；缺失则在约定位置新建声明并标注 `TODO(接口对齐)`

## 4. 交付后回写（阶段 5）

1. **可直接回写**（信息可从本次交付客观得出）：新场景条目的 entry/routes/apis/components、change_id、status: delivered、api 新增声明路径、theme.source、meta.updated_at、meta.skills_root（为空或已失效时填入当前解析出的技能根目录；已失效 = 所指目录下无 fe-project-index/SKILL.md）
2. **须问询后回写**（主观判断）：场景 name 的业务命名、新页面应并入既有场景还是独立成条、glossary 新词条 → 给出建议值让用户确认/修改
3. **合并规则**：
   - 只做**增量合并**：新增条目、补全空字段、追加数组元素；**不得删除或改写既有条目的人工内容**
   - 同名场景已存在 → 补全其 routes/apis/components 并更新 change_id，不重复建条
   - index.yaml 回写完成后**同步刷新 index.md**：组件库清单（component_libs）、工程约定速览、产物索引若有变化一并更新（如首次生成 theme-palette.md 后补索引行）
   - 回写后输出变更摘要（两个文件各新增/修改了哪些键）
4. 用户明确拒绝回写 → 跳过并在交付总结注明「索引未更新」
