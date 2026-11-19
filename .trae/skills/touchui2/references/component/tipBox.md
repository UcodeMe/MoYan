# 工具提示（c_tooltip/TipBox）
当鼠标在某特定区域上停留时显示的提示信息，信息中包含对特定区域的相关说明，属于事中提醒。

## HTML

<a href="#/example/component/tooltip" target="_blank">查看演示</a>

```html
<button type="button" tip="删除后不可恢复，请谨慎操作">删除</button>
```
注：元素必须是在界面上可见的，且不支持触屏客户端。



如果需要提示换行，加入 \n 即可：

```html
<button type="button" tip="删除后不可恢复\n请谨慎操作">删除</button>
```
使用 tipcolor="red"，可以改变提示颜色：
```html
<button type="button" tip="删除后不可恢复请谨慎操作" tipcolor="red">删除</button>
```

## JS API

<a href="#/example/component/tooltip-js" target="_blank">查看演示</a>

使用 JS API 弹出提示可支持触屏客户端，但需要自己绑定触发事件。

```js
// API
$.TipBox.show(node, tip, color);

// 示例
$.TipBox.show(document.getElementById('myButton'), "提示信息", "red");
```

| 参数名 | 参数类型               | 是否必须 | 默认值  | 描述     |
| ------ | ---------------------- | -------- | ------- | -------- |
| node   | string 或原生 DOM 对象 | 是       | 否      | 节点对象 |
| tip    | string                 | 是       | 否      | 提示信息 |
| color  | string                 | 否       | "black" | 色彩设置 |

## 插件

### 功能插件

<a href="#/example/component/tooltip-close" target="_blank">查看演示</a>

html
```html
<div class="c_tooltip">
	<div class="content">带删除的 tooltip</div>
	<div class="close"></div>
</div>
```
```html
<div class="c_tooltip">
	<div class="content">带设置的 tooltip</div>
	<div class="config"></div>
</div>
```

## 样式

工具提示可以用于帮助提示，并定制特殊的样式，但目前仅支持静态方式，HTML 结构如下：

```html
<div class="c_tooltip">
	<div class="content">提示文字</div>
	<div class="close"></div>
</div>
```

相关参数如下：

| 参数名          | 值                                                           | 默认值           | 描述     |演示     |
| --------------- | ------------------------------------------------------------ | ---------------- | -------- |---- |
| c_tooltip-help |c_tooltip-help<br />c_tooltip-help-l| c_tooltip-help | 帮助提示模式 |<a href="#/example/component/tooltip-help" target="_blank">查看</a>|
| c_tooltip-arrow | c_tooltip-arrow-curve |      无            | 箭头样式 |<a href="#/example/component/tooltip-arrowCurve" target="_blank">查看</a>|
| c_tooltip-bottom | c_tooltip-bottom<br />c_tooltip-top<br />c_tooltip-right<br />c_tooltip-bottom-right<br />c_tooltip-top-right<br />c_tooltip-left<br />c_tooltip-bottom-left<br />c_tooltip-top-left | c_tooltip-bottom | 方向 |<a href="#/example/component/tooltip-left" target="_blank">查看</a>|
| c_tooltip-red | c_tooltip-black<br />c_tooltip-red<br />c_tooltip-orange<br />c_tooltip-green | c_tooltip-black | 提示颜色 |<a href="#/example/component/tooltip-color" target="_blank">查看</a>|

注意：方向的设置并不是指箭头所在的方向，而是 tooltip 相对于目标元素的所在位置，因此刚好与箭头方向相反。 

## 可替换的子结构

### 关闭按钮

```html
<div class="close"></div>
```

可替换为：

```html
<div class="config"></div>
```

