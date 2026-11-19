---
name: agent-frontend-architect
description: "前端开发分身（SDD 流程）的技能包封装入口。内含三个前端专属技能：交付自检（fe-self-check，三路径必跑）、主题配色（fe-theme-palette）、项目索引（fe-project-index，仅 P3 现场开发路径）。本文件只做简单介绍与内部技能清单，不承载执行逻辑；主编排逻辑见上级 agents.md。"
---

# agent-frontend-architect · 前端开发分身技能包

> **定位**：本文件夹是「前端开发分身」的技能封装包（分身标识 `agent-frontend-architect`，目标工程标准技能根目录 `.trae/skills/agent-frontend-architect`）。分身身份与主编排逻辑（路径判定 P1/P2/P3、编码通则、闸口纪律）见上级 [`../agents.md`](../agents.md)。
> **职责**：接续上游 change 产物（proposal/specs/design/tasks），负责前端设计段、前端任务段与前端编码，并对前端交付结果负责；不内置私有组件库知识，具体写法一律来自被识别的栈技能或目标模块相邻真实代码。
> **本文件边界**：只做简单介绍与内部技能清单——执行逻辑一律在各子技能 SKILL.md 内，按需加载、不在本文件重复。

## 一、简单介绍

前端开发分身按「轻重分级（S 走流程 / N 直改）× 环境三态（E1 aispec 全装 / E2 仅 openspec / E3 皆无）」路由三种工作路径：**P1** aispec 全链、**P2** 轻量 openspec、**P3** 直改（P3′ 为 E3+S 用户豁免态）。

本包封装其中三个**前端专属技能**（见下表）；栈技能（touchui2 / touchvue2 等）与视觉补充技能（ui-ux-pro-max）由发现机制识别、不预设，随本包同级分发（见下「协作技能」）。

## 二、内部技能列表清单

| # | 技能 | 编排位置 | 一句话职责 | 产物 |
|---|---|---|---|---|
| 1 | [`fe-self-check`](fe-self-check/SKILL.md) | 编码后 · **三路径必跑** | 从功能点切片、交互逻辑、颜色配色三维度对本次交付自检，驱动修复闭环（blocker/major 必修，最多 3 轮，不收敛停手交用户决策） | 自检报告 → `openspec/.project-frontend/self-check/<日期-change-id>.md` |
| 2 | [`fe-theme-palette`](fe-theme-palette/SKILL.md) | 编码套色前 · 令牌缺失/过期时 | 挖掘项目 UI 存量配色（存量项目）或集成 ui-ux-pro-max 生成设计系统（新项目）；两条开发路径唯一通用的存量挖掘技能，不读不写 index 文件 | `openspec/.project-frontend/theme-palette.md`（配色令牌基线） |
| 3 | [`fe-project-index`](fe-project-index/SKILL.md) | 判定期与 P3 · **仅现场开发路径** | 维护项目导航索引（首次初始化挖掘、场景参考查询、交付后增量回写）；P1/P2 续接路径锁定后**严禁触发或读取** | `openspec/.project-frontend/index.md` + `index.yaml` |

### 协作技能（不在本包内，供参照）

| 技能 | 用途 | 位置 |
|---|---|---|
| `ui-ux-pro-max` | 视觉补充与设计系统检索（fe-theme-palette 路线 B 调用其 `scripts/search.py`） | 本仓库上级 `../ui-ux-pro-max/`；部署后位于同一 `<SKILLS_ROOT>` 下 |
| `touchui2` / `touchvue2` | 私有栈技能（Touch UI 的 JWC / Vue2 形态写法），由组件库发现机制识别 | 本仓库上级 `../touchui2/`、`../touchvue2/`；部署后位于同一 `<SKILLS_ROOT>` 下 |

> 三者均以 `<SKILLS_ROOT>`（标准位置 `.trae/skills/agent-frontend-architect`）为引用根，由主流程按标准目录探测解析，禁止从 index 文件读取。
