# 如何在表格中实现跨分页的 checkbox

## 描述

表格中如果翻页后，checkbox 的状态会被重置，如果希望保留这个状态，可以采取如下方案。

## 基础方式

最简单的方法如下：

1. `<input type="hidden" />`
2. 勾选后将值放入 input。
3. 翻页后检查 input 中是否有对应的 checkbox。
4. 将值回填到 checkbox。
5. 提交时取 input 的值。