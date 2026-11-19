# 如何实现高度自适应布局

## 描述

所谓高度自适应布局，就是指页面中核心的内容，随着窗体高度的变化或不同设备的不同窗体高度，自动展示为可适配的最大高度。

## 基础方式

最简单的方法如下：

```html
<div style="height:100%; overflow:auto;">
	...
</div>
```

这样一来，``<div>`` 本身的高度便与窗体的高度完全一致，无论窗体如何变化，``<div>`` 始终占满全屏，其中的内容超过 ``div`` 的高度，则会显示一个全屏高度的滚动条。

Touch UI 中，<a href="#/doc/component/scroll">滚动条组件</a>的默认状态就是这样一个自适应窗体高度的区域，也是基于这一原理实现的。

```html
<div class="c_scroll">
	...
</div>
```

## 减去固定高度

高度自适应非常见简单，但往往需求要求页面的部分固定，剩余的部分自适应剩余的高度：

![flexHeight](static/img/doc/flexHeight.png)

而纯粹的 CSS 并不支持去获取某个对象的高度并传入另一条样式规则。

大部分情况，我们是已知需要减去的高度的值，在这一范围内，使用纯粹的 CSS 可以通过 absolute 定位的特性来实现：

```html
<div style="overflow:auto; position:absolute; top:2em; right:0; bottom:0; left:0;">
	...
</div>
```

Touch UI 中，滚动条的 ``c_scroll-float`` 样式参数，则提供了 absolute 同样的能力，而 ``c_scroll-header`` 则表示固定部分的高度 = <a href="#/doc/component/header">header 组件</a>的高度。

```html
<div class="c_scroll c_scroll-float c_scroll-header">
	...
</div>
```

## 其他说明
* 如果顶部的高度并不是固定的，则必须使用 JS 的 resize 事件去适应高度。
* 除了滚动条组件外，<a href="#/doc/layout/float">自定义悬浮布局</a>也支持这种方式，只是悬浮布局没有滚动条。
