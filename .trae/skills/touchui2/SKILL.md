---
name: touchui2
description: Touch 组件库 touch（TouchUI2）的技术栈技能：JWC 纯前端组件开发规范、CSS 类名体系、页面初始化模板、高频链路配方与组件文档导航。生成涉及 TouchUI2（touch/JWC）写法的代码前必读（按需参考：组件库为储备，工程可同时用多种库）。
---

# TouchUI2（touch 库）技术栈技能

TouchUI2 是 Touch 组件库的 JWC 形态（别名 touch），响应式多端 UI 框架（网页/Web 应用/触屏 App），核心是 **CSS 类名体系 + JWC 前端模板组件 + Wade JS 运行时**。本技能描述 TouchUI2 库的写法；工程若同时使用其他组件库，按各库分别参考对应技能，**同一组件/页面内不混用本库与他库的组件 API**。组件明细按需读本技能 `references/` 下对应文档，禁止一次性读完整个文档库。

## 1. 识别证据与按需参考

**按需参考**：当上游 design、package/静态资源、目标文件或相邻实现的真实证据表明本次使用 TouchUI2/touch/JWC 时，对涉及本库的代码采用本技能。续接上游 change 时禁止为了判断组件库读取任何 index 文件。

判定证据：
- 入口 html 引入 `TouchUI/js/jcl.js`、`jcl-ui.js`、`jcl-jwc.js` 等脚本
- 模板中出现 `jwcid="@组件名"` 属性、`ognl:` 绑定
- 样式类名大量使用 `c_*`（组件）/ `e_*`（元素）/ `l_*`（布局）前缀
- 存在 `assets/js/config.js` 定义 `CSS_PATH / SCRIPT_PATH / FILE_UPLOAD_URL` 等

## 2. 核心心智模型

### 2.1 CSS 类名体系（三前缀 + 响应规则）

| 前缀 | 含义 | 示例 |
|------|------|------|
| `c_` | 复合组件 | `c_form`、`c_table`、`c_card`、`c_list` |
| `e_` | 最小元素 | `e_button-blue`、`e_ico` |
| `l_` | 布局容器 | `l_col`、`l_center`、`l_query` |

- 修饰类叠加：`c_form c_form-col-3 c_form-label-6`（3 列、标签宽 6）
- **响应规则**：`-phone-` 中缀声明手机端形态，如 `c_form-phone-col-1`（手机单列）、`l_col-phone-split-3`（手机递进分列）。多端适配优先用响应规则，不写媒体查询。
- 控件矢量缩放：`style="font-size:139%"` 可整体等比缩放组件。

### 2.2 JWC 模板组件

以 HTML 属性声明组件实例：`jwcid="@组件类型"`，组件参数写在同元素属性上，动态值用 `ognl:` 绑定：

```html
<input jwcid="@TextField" type="text" name="CUST_ID" desc="客户号" nullable="no" maxsize="30" datatype="text" />
<span jwcid="@Select" name="STATUS" source="ognl:statusOptions" addDefault="true" textField="text" valueField="value" desc="状态"></span>
<tr jwcid="@Foreach" source="ognl:rows" value="ognl:row" element="tr">
  <td><span jwcid="@Insert" value="ognl:row.CUST_ID"></span></td>
</tr>
```

常用 JWC：`@Body`、`@Form`、`@TextField`、`@Select`、`@Button`、`@Table`、`@Part`（局部刷新区）、`@Foreach`、`@Insert`、`@Conditional`。

### 2.3 Wade 运行时（页尾必需）

```html
<script type="text/javascript">
  Wade.setRatio();       // 界面矢量比例，页面可用时执行
  Wade.JWC.init(true);   // 前端 JWC 模板解析
</script>
```

## 3. 页面初始化模板（新页面骨架）

```html
<!DOCTYPE HTML>
<html size="s">
<head>
  <meta http-equiv="X-UA-Compatible" content="IE=edge,chrome=1" />
  <meta name="viewport" content="width=device-width, initial-scale=1, minimum-scale=1, maximum-scale=1" />
  <meta http-equiv="Content-Type" content="text/html; charset=UTF-8" />
  <link href="TouchUI/css/base.css" rel="stylesheet" type="text/css" />
  <script type="text/javascript" src="TouchUI/js/jq/jquery-1.9.1.js"></script>
  <script type="text/javascript" src="TouchUI/js/jq/adapter-1.9.1.js"></script>
  <script type="text/javascript" src="TouchUI/js/jcl.js"></script>
  <script type="text/javascript" src="TouchUI/js/i18n/code.zh_CN.js"></script>
  <script type="text/javascript" src="TouchUI/js/jcl-plugins.js"></script>
  <script type="text/javascript" src="TouchUI/js/jcl-ui.js"></script>
  <script type="text/javascript" src="TouchUI/js/jcl-jwc.js"></script>
  <script type="text/javascript" src="assets/js/config.js"></script>
</head>
<body jwcid="@Body">
  <!-- 页面内容 -->
  <script type="text/javascript">
    Wade.setRatio();
    Wade.JWC.init(true);
  </script>
</body>
</html>
```

> 资源相对路径以工程实际为准，优先看 design 指向的目标/相邻文件与 `assets/js/config.js` 的 `CSS_PATH / SCRIPT_PATH`，不得用 index 替代真实代码证据。

## 4. 高频链路配方

| 链路 | 关键做法 | 详读文档 |
|------|----------|----------|
| 表单 + 校验 | `c_form` 布局 + `@TextField` 声明 `nullable/maxsize/datatype/desc` + `$.validate.verifyAll(dom)` | `references/component/form.md`、`references/js-api/verify.md` |
| 表格渲染 | `c_table` + `@Table` + `@Foreach source="ognl:rows"` 渲染行 | `references/component/table.md`、`references/logical/foreach.md` |
| 查询 + 局部刷新 | 结果区包 `@Part`（有 id），`$.ajax.submit(formId, action, data, partIds, ok, err)` 提交并刷新 | `references/js-api/ajax.md`、`references/logical/part.md` |
| 弹窗/对话框 | `c_popup` / `c_dialog`，JS 控制见 dom API | `references/component/popup.md`、`references/component/dialog.md`、`references/js-api/dom.md` |
| 上传/导入导出 | 组件 API 看 component，链路与接口约定看 use/reception-* | `references/component/upload.md`、`references/use/reception-fileUpload.md`、`references/use/reception-importAndExport.md` |
| 端到端完整示例 | 查询表单→提交→局部刷新→表格渲染 全链路 | `references/examples/e2e-query-table-part.md`（**新页面开发前建议通读**） |

事件绑定用 `$("#id").tap(fn)`（触屏兼容），不用 `click`。

## 5. 组件文档索引（按需读取）

- **component/（复合组件 42 个）**：form 表单、table 表格、popup 弹出层、dialog 对话框、upload/simpleUpload 上传、import/export 导入导出、list 列表、card 卡片、tabset 页签、navbar 导航、tree 树、calendar 日历、filter 筛选、messageBox/tip/tipBox 提示、float/floatBox 悬浮、slide 轮播、scroll 滚动、step 步骤、timeline 时间轴、menu 菜单、chat 会话、editor 编辑器、keyboard 键盘、guide 引导、head/header/title/navbar 头部体系、frame 框架、box/space/line 结构件、param/fn/any/submit 辅助件 等 → `references/component/<名>.md`
- **element/（最小元素 30 个）**：button 按钮、textField 文本框、textarea 多行、select 下拉、checkbox 复选、radio 单选、dateField 日期、switch 开关、progress 进度、segment 分段、suggest 联想、moneyField 金额、ipAddressField IP、increaseReduce 步进、star 评分、tag 标签、pic 图片、ico 图标、label/font/align/size/space/help/dis/ellipsis/group/insert/mix/range 等 → `references/element/<名>.md`
- **layout/（布局 8 个）**：col 分列、center 居中、absolute 绝对定位、float 浮动、padding 边距、query 查询区、edit 编辑区、roll 卷轴 → `references/layout/<名>.md`
- **logical/（模板逻辑 6 个）**：foreach 循环、part 局部刷新、conditional 条件、bodycontent/bodyscript/bodystyle → `references/logical/<名>.md`
- **js-api/（运行时 9 个）**：ajax 请求、dom 操作、data 数据结构、verify 校验、format 格式化、global 全局、basicData 基础数据、client 客户端信息、redirect 跳转 → `references/js-api/<名>.md`
- **chart/（图表 7 个）**：barChart、lineChart、pieChart、doughnutChart、gantt、kChart、mapChart（ECharts 封装） → `references/chart/<名>.md`
- **mobile/**：Android/iOS 原生交互；**faq/**：排障经验；**use/**：框架机制（`about.md` 概览、`jwc.md` JWC 机制、`displayScale.md` 显示比例、`ui*.md` 配色定制、`reception-*.md` 前台接待链路）

### 同名主题选择规则

- `float`：布局浮动看 `layout/float.md`；悬浮层交互看 `component/float.md`、`component/floatBox.md`
- `space`：基础间距看 `element/space.md`；页面布局间距块看 `component/space.md`
- `upload`：组件 API 看 `component/upload.md`；上传链路与接口约定看 `use/reception-fileUpload.md`
- `import/export`：组件 API 看 `component/import.md`、`component/export.md`；整体流程看 `use/reception-importAndExport.md`

## 6. 排查要点（自检/联调常见坑）

1. `Wade.setRatio()` 未执行 → 界面比例异常；`Wade.JWC.init(true)` 未执行 → `jwcid` 组件不渲染。
2. `$.ajax.submit` 的 partids 必须传 `@Part` 容器的 id，否则局部刷新不生效。
3. 提交前必须 `$.validate.verifyAll(...)` 返回 true 才继续，失败要中止并让校验提示展示。
4. 响应规则类名遗漏 → 手机端布局塌陷；每个 `c_form/l_col` 布局都应确认手机端形态。
5. 配色定制走框架皮肤/配色方案（`use/ui0.md`~`ui2.md`），不要散落覆写组件内部样式。
