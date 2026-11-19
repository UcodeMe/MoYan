# 如何打开多个 popup

除了多级弹框， ``@Popup`` 弹框默认是不能同时出现多个弹框的，在打开第二个弹框时，会默认隐藏掉前一个弹框。

## 场景分析

在弹框之上叠加弹框，如：在全屏弹框之上继续弹出 popup 弹框，需求：前一个弹框不能被隐藏消失。

![IE8IconPosition](../static/img/doc/morepopup.png) <br/>

## 解决方案

### JWC方法

直接加上 ``overlap="true"`` 属性

### JS方法

```js
window["UI-popup"] = new Wade.Popup("UI-popup", {
	overlap: true
});
```		

加上 overlap 属性以后就不会在打开第二个 Popup 时隐藏前一个 Popup 了
