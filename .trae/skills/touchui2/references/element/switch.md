# 开关（e_switch / @Switch）
开关是最基本的表单元素之一，用于一键切换布尔型的参数的值。 

## JWC 方式
<a href="#/example/element/switch" target="_blank">查看演示</a>

```html
<span name="mySwitch" jwcid="@Switch" value="off" onchange="alert(this.value)"></span>
```

## JS 方式

html
```html
<div class="e_switch">
	<div class="e_switchOn"></div>
	<div class="e_switchOff"></div>
	<input type="hidden" id="mySwitch" />
</div>
```
js
```js
window["mySwitch"] = new Wade.Switch("mySwitch",{
	switchOn:false,
	onValue:"on",
	offValue:"off"
});
```

## 属性

| 参数名   | 参数类型         | 是否必须 | 默认值 | 描述                               |演示     |
| -------- | ---------------- | -------- | ------ | ---------------------------------- |----- |
| name     | java.lang.String | 是       | 无     | 组件实例名，同一个页面里不能重复   ||
| class    | java.lang.String | 否       | 无     | 自定义样式名                       ||
| style    | java.lang.String | 否       | 无     | 自定义样式                         ||
| disabled | boolean          | 否       | 无     | 是否禁用                           ||
| value    | java.lang.String | 否       | 无     | 当前值（必须匹配开值或关值才有效） ||
| onValue  | java.lang.String | 否       | 无     | 开值                               |<a href="#/example/element/switch-color" target="_blank">查看</a>|
| offValue | boolean          | 否       | 无     | 关值                               |<a href="#/example/element/switch-color" target="_blank">查看</a>|
| onText   | java.lang.String | 否       | 无     | 开状态文本                         ||
| offText  | java.lang.String | 否       | 无     | 关状态文本                         ||
| onColor  | java.lang.String | 否       | 无     | 开状态颜色                         |<a href="#/example/element/switch-color" target="_blank">查看</a>|
| offColor | java.lang.String | 否       | 无     | 关状态颜色                         |<a href="#/example/element/switch-color" target="_blank">查看</a>|

## JS API

设置默认值
```js
$("#mySwitch").val("on");
```

获取开关的当前值

```js
$("#mySwitch").val();
```

绑定 change 事件

```js
$("#mySwitch").change(function(){
	alert(this.value); // this.value 获取开关组件当前值
});
```

设置开关的值（必须匹配开值或关值才有效）

```js
$("#mySwitch").val(value);
```

## 显示文本设置
jwc 组件
```html
<div name="mySwitch" jwcid="@Switch" value="off" onText="On" offText="Off" />
```
js组件
```html
<div class="e_switch">
	<div class="e_switchOn">On</div>
	<div class="e_switchOff">Off</div>
	<input type="hidden" id="mySwitch" />
</div>
```

## 色彩设置

jwc 组件
```html
<div name="mySwitch" jwcid="@Switch" value="off" onColor="blue" offColor="red" />
```
js 组件

```js
window["mySwitch"] = new Wade.Switch("mySwitch",{
	onColor:"blue",
	offColor:"red"
});
```
支持：blue、red、green、gray 

## 复合案例

jwc 组件

```html
<div name="mySwitch" jwcid="@Switch" value="off" onColor="blue" offColor="red" onText="男" offText="女" />
```
js 组件
```html
<div class="e_switch">
	<div class="e_switchOn">男</div>
	<div class="e_switchOff">女</div>
	<input type="hidden" id="mySwitch" />
</div>
```

```js
window["mySwitch"] = new Wade.Switch("mySwitch",{
	onColor:"blue",
	offColor:"red"
});
```


