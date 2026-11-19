# TouchUI2 技能自测用例

本文件用于对 TouchUI2 技能的“触发准确性、输出规范性、跨模块协同”进行自测。每条用例包含：输入、期望命中模块、期望输出结构与关键点。

## 用例 1：多端响应式表单布局

- 输入：实现一个查询表单，PC 三列，手机一列；需要最小 HTML 骨架
- 期望主模块：`component/form.md`
- 期望辅助模块：`use/displayScale.md`、`layout/col.md`（如需）
- 关键点：
  - 输出结构包含：目标 / 方案 / 实现 / 边界与兼容 / 引用
  - 给出响应规则类（例如 `c_form-col-3`、`c_form-phone-col-1`）的使用方式

## 用例 2：表单校验失败提示

- 输入：TextField 必填 + 最大长度 + datatype；如何触发校验并定位错误
- 期望主模块：`js-api/verify.md`
- 期望辅助模块：`element/textField.md`、`component/tip.md`（如需）
- 关键点：
  - 输出提供 `nullable/datatype/maxsize` 属性用法
  - 输出提供 `$.validate.verifyAll` 与 `$.validate.verifyField` 的分支建议

## 用例 3：点击查询触发局部刷新表格

- 输入：点击按钮后只刷新表格区域，不整页刷新；给出最小实现
- 期望主模块：`logical/part.md`
- 期望辅助模块：`js-api/ajax.md`、`component/table.md`、`logical/foreach.md`
- 关键点：
  - 输出给出 `@Part` 的容器与 `partids` 参数的对应关系
  - 输出解释“数据渲染（Foreach）”与“局部刷新（Part）”的组合顺序

## 用例 4：弹窗点击穿透

- 输入：popup 弹窗打开后，底层按钮仍可点击；如何处理
- 期望主模块：`faq/popupClickThroughProblem.md`
- 期望辅助模块：`component/popup.md`
- 关键点：
  - 输出必须先索取可复现条件（触发方式、弹窗类型、是否 iframe、终端/浏览器）
  - 输出给出遮罩、事件阻止、层级策略等“可执行下一步”

## 用例 5：上传组件 + 前台上传链路

- 输入：页面要做上传；需要组件用法与后台接口约定要点
- 期望主模块：`component/upload.md`
- 期望辅助模块：`use/reception-fileUpload.md`
- 关键点：
  - 输出区分“组件 API 使用”与“链路/接口约定”
  - 输出提示必须确认 upload/delete/query/download URL 配置来源（例如 config.js）

## 用例 6：移动端原生能力调用失败（退化方案）

- 输入：iOS 原生方法调用无响应；要求给出排查与 Web 退化方案
- 期望主模块：`mobile/fn-ios.md` 或 `mobile/native-ios.md`
- 期望辅助模块：`mobile/web.md`
- 关键点：
  - 输出先确认是否在原生容器内、调用方法名与入参
  - 输出提供 Web 侧替代路径或能力降级建议

