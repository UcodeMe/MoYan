---
name: touchvue2
description: Touch 组件库 touch2（TouchVue2）的技术栈技能：Vue2 工程形态下的 Touch 组件开发规范、类名体系、端到端链路与组件文档导航。生成涉及 TouchVue2（touch2/Vue2）写法的代码前必读（按需参考：组件库为储备，工程可同时用多种库）。
---

# TouchVue2（touch2 库）技术栈技能

TouchVue2 是 Touch 组件库的 Vue2 形态（别名 touch2）：延续 Touch UI 的 **CSS 类名体系（`c_`/`e_`/`l_` + 响应规则）与组件视觉语义**，工程形态上以 **Vue2 单文件组件（.vue）** 组织页面。本技能描述 TouchVue2 库的写法；工程若同时使用其他组件库，按各库分别参考对应技能，**同一组件/页面内不混用本库与他库的组件 API（如不在 .vue 里写 `jwcid` JWC 模板）**。组件明细按需读本技能 `references/` 下对应文档（TouchVue2 文档库），禁止一次性读完整个文档库。

## 1. 识别证据与按需参考

**按需参考**：当上游 design、package 依赖、目标文件或相邻实现的真实证据表明本次使用 TouchVue2/touch2 时，对涉及本库的代码采用本技能。续接上游 change 时禁止为了判断组件库读取任何 index 文件。

判定证据：
- `package.json` 存在 `vue@2.x` 依赖，页面以 `.vue` 单文件组件组织
- 同时工程中出现 Touch UI 类名体系（`c_*`/`e_*`/`l_*`）或 Touch UI 静态资源目录
- 菜单/路由由 vue-router 或工程自有配置承载

**与 touch（TouchUI2）库区分**：touch 工程是静态 html + `jwcid` JWC 模板 + Wade 运行时；touch2 工程是 .vue 组件树。**两类证据同时存在 → 工程同时使用 touch 与 touch2，按页面/场景分别读对应技能（多库共存合法）；都不充分才问询用户**，不得臆测。

## 2. 开发规范

### 2.1 页面组织（Vue2 形态）

- 页面组件放入 design 文件计划指定的目录；未明确时向目标文件同目录或最接近的既有 views/pages 实现对齐
- 模板层使用 Touch UI 类名体系搭建结构与响应式：

```vue
<template>
  <div class="c_form c_form-col-3 c_form-phone-col-1">
    <ul>
      <li>
        <div class="label">客户号：</div>
        <div class="value"><input type="text" v-model="query.custId" /></div>
      </li>
    </ul>
    <div class="submit">
      <button type="button" class="e_button-blue" @click="onQuery">查询</button>
    </div>
  </div>
</template>
```

- 数据绑定、条件与循环用 Vue 指令（`v-model`/`v-if`/`v-for`）承担；references 文档中的 `jwcid="@Foreach"` 等 JWC 模板写法属于 touch 库，**本库（touch2/Vue）内改用 Vue 指令实现相同 DOM 结构**，类名保持文档规定
- 请求层遵循 design 指向模块及其相邻代码中的既有封装；无既有封装时与用户确认，不擅自引入 axios 等新依赖

### 2.2 类名体系与响应规则（与 TouchUI2 一致）

- `c_` 复合组件 / `e_` 最小元素 / `l_` 布局；修饰类叠加（`c_form-col-3`、`c_form-label-6`）
- 手机端形态用 `-phone-` 响应规则类（`c_form-phone-col-1`、`l_col-phone-split-3`），优先于媒体查询
- 组件结构（ul/li/label/value 等 DOM 骨架）**必须按 references 文档的结构写**，类名体系依赖固定 DOM 结构

## 3. 高频链路配方

| 链路 | 关键做法 | 详读文档 |
|------|----------|----------|
| 表单 + 校验 | `c_form` 结构 + 校验规则（工程已有校验封装优先；参考 verify 语义） | `references/component/form.md`、`references/js-api/verify.md` |
| 表格/列表 | `c_table` 结构 + `v-for` 渲染行 | `references/component/table.md`、`references/component/list.md` |
| 弹窗/对话框 | `c_popup` / `c_dialog` 结构与交互语义 | `references/component/popup.md`、`references/component/dialog.md` |
| 上传/导入导出 | 组件结构看 component，链路与接口约定看 use/reception-* | `references/component/upload.md`、`references/use/reception-fileUpload.md`、`references/use/reception-importAndExport.md` |
| 布局/查询区 | 分列 `l_col`、查询区 `l_query`、居中 `l_center` | `references/layout/col.md`、`references/layout/query.md` |

## 4. 组件文档索引（按需读取）

目录结构与 TouchUI2 文档库一致：

- **component/（复合组件 42 个）**：form、table、popup、dialog、upload/simpleUpload、import/export、list、card、tabset、navbar、tree、calendar、filter、messageBox/tip/tipBox、float/floatBox、slide、scroll、step、timeline、menu、chat、editor、keyboard、guide、head/header/title、frame、box/space/line、param/fn/any/submit 等 → `references/component/<名>.md`
- **element/（最小元素 30 个）**：button、textField、textarea、select、checkbox、radio、dateField、switch、progress、segment、suggest、moneyField、ipAddressField、increaseReduce、star、tag、pic、ico 等 → `references/element/<名>.md`
- **layout/（8 个）**：col、center、absolute、float、padding、query、edit、roll → `references/layout/<名>.md`
- **logical/**：模板逻辑语义（Vue 工程内用 Vue 指令实现同等结构）→ `references/logical/<名>.md`
- **js-api/（9 个）**：ajax、dom、data、verify、format、global、basicData、client、redirect → `references/js-api/<名>.md`
- **chart/（7 个）**：barChart、lineChart、pieChart、doughnutChart、gantt、kChart、mapChart → `references/chart/<名>.md`
- **use/**：`about.md` 框架概览、`jwc.md` 运行机制、`displayScale.md` 显示比例、`ui*.md` 配色定制、`reception-*.md` 前台链路；**faq/**：排障经验

### 同名主题选择规则

- `float`：布局浮动看 `layout/float.md`；悬浮层交互看 `component/float.md`、`component/floatBox.md`
- `space`：基础间距看 `element/space.md`；页面布局间距块看 `component/space.md`
- `upload`：组件 API 看 `component/upload.md`；上传链路看 `use/reception-fileUpload.md`
- `import/export`：组件 API 看 `component/import.md`、`component/export.md`；整体流程看 `use/reception-importAndExport.md`

## 5. 端到端示例（查询表单 → 请求 → 表格渲染）

```vue
<template>
  <div>
    <div class="c_form c_form-col-3 c_form-phone-col-1 c_form-submit">
      <ul>
        <li>
          <div class="label">客户号：</div>
          <div class="value"><input type="text" v-model.trim="query.custId" maxlength="30" /></div>
        </li>
        <li>
          <div class="label">状态：</div>
          <div class="value">
            <select v-model="query.status">
              <option value="">全部</option>
              <option v-for="opt in statusOptions" :key="opt.value" :value="opt.value">{{ opt.text }}</option>
            </select>
          </div>
        </li>
      </ul>
      <div class="submit">
        <button type="button" class="e_button-blue" :disabled="loading" @click="onQuery">查询</button>
      </div>
    </div>

    <table class="c_table">
      <thead>
        <tr><th><span>客户号</span></th><th><span>客户名称</span></th></tr>
      </thead>
      <tbody>
        <tr v-for="row in rows" :key="row.CUST_ID">
          <td><span>{{ row.CUST_ID }}</span></td>
          <td><span>{{ row.CUST_NAME }}</span></td>
        </tr>
        <tr v-if="!loading && rows.length === 0">
          <td colspan="2"><span>暂无数据</span></td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script>
export default {
  data() {
    return {
      loading: false,
      query: { custId: "", status: "" },
      statusOptions: [{ text: "正常", value: "N" }, { text: "冻结", value: "F" }],
      rows: []
    };
  },
  methods: {
    onQuery() {
      if (!this.query.custId) { /* 走工程既有校验/提示封装，校验失败必须 return */ return; }
      this.loading = true;
      // 请求走目标模块既有封装，不擅自引入新依赖
      this.$request("customer/query", this.query)
        .then(data => { this.rows = data.rows || []; })
        .catch(() => { /* 走工程既有失败提示封装 */ })
        .finally(() => { this.loading = false; });
    }
  }
};
</script>
```

要点：`c_form/c_table` 的 DOM 骨架与类名严格按 `references/component/form.md`、`references/component/table.md`；防重复提交（loading 禁用）、空态行、校验失败中止是自检「交互逻辑」维度的必查项。

## 6. 排查要点

1. Touch UI 组件的 DOM 骨架被 Vue 组件拆散/改写 → 样式失效；保持文档规定的结构层级。
2. 响应规则类名遗漏 → 手机端布局塌陷；每个布局块确认 `-phone-` 形态。
3. 复用工程既有请求/校验/弹窗封装（以参考场景代码为准），不并行造第二套。
4. 配色走框架配色方案与主题 token（见 `references/use/ui0.md`~`ui2.md` 与 theme-palette.md），禁止散落硬编码色值。
5. 端到端链路结构以本技能第 5 节示例与本库 `references/` 文档为准；本库（touch2/Vue）内不混入 touch 库的 JWC 写法。
