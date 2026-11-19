# 对话框（c_dialog/@Dialog）


放置预隐藏的次级信息的载体，展示时位于屏幕中央。 

## JWC 方式
<a href="#/example/component/dialog" target="_blank">查看演示</a>
```html
<div jwcid="@Dialog" class="c_dialog" name="myDialog" onhideAction="alert('hideAction')" onafterAction="alert('afterAction')">
	<!-- Dialog 弹窗内容 -->
</div>
```

## JS 方式

html

```html
<div class="c_dialog" id="myDialog" style="display:none;">
	<div class="wrapper">
		<div class="header">
			<div class="text">标题文字</div>
			<div class="fn">
				<div class="close"></div>
			</div>
		</div>
		<div class="content">
			<!-- 内容 -->
		</div>
	</div>
</div>
```
js
```js
window["myDialog"] = new Wade.Dialog("myDialog");
```

注：@Dialog 组件的结构必须写在滚动条之外，以免受其影响。

## 属性

|参数名|参数类型|是否必填|默认值|描述|
|-|-|-|-|-|
|name|java.lang.String|是|无|元素 id 和 name 属性的值，也是提交表单数据时的字段名字|
|class|java.lang.String|否|无|自定义样式名|
|style|java.lang.String|否|无|自定义样式|
|title|java.lang.String|否|无|弹窗标题|
|width|int|否|无|弹窗宽度，单位为 em|
|height|int|否|无|弹窗高度，单位为 em|
|closeable|boolean|否|true|是否允许关闭（为 true 时弹窗上有关闭按钮）|

## JS API

打开弹窗

```js
myDialog.show();
```

关闭弹窗

```js
myDialog.hide();
```

打开 iframe 弹窗

```js
//API：
popupDialog(title, page, listener, params, subsys, width, height, closeable, afterAction, hideAction);

//示例：
popupDialog('弹出标题', 'examples.PopupFrame', 'init', '&a=1&b=2', null, '60', '30', true, afterAct, hideAct);
```

| 参数名      | 参数类型           | 是否必须 | 默认值 | 描述                                                         |
| ----------- | ------------------ | -------- | ------ | ------------------------------------------------------------ |
| title       | string             | 是       | 无     | 弹窗标题，弹窗页面里有 c_header 时会自动设置，相同的标题只会弹一个弹窗 |
| page        | string             | 是       | 无     | application 文件中定义的页面名                               |
| listener    | string             | 否       | 无     | 页面 java 类监听方法名                                       |
| params      | string             | 否       | 无     | 打开页面携带的 url 参数                                      |
| subsys      | string             | 否       | 无     | 子系统相对路径（例："/saleserv/saleserv"），跨模块打开页面的时候，需要传入此参数。 |
| width       | number             | 是       | 无     | 弹窗宽度，默认单位为 rem                                     |
| height      | number             | 是       | 无     | 弹窗高度，默认单位为 rem                                     |
| closeable   | boolean            | 否       | 无     | 弹窗是否有关闭按钮，默认为 true                              |
| afterAction | string 或 function | 否       | 无     | 在 setPopupReturnValue 时触发的动作                          |
| hideAction  | string 或 function | 否       | 无     | 在隐藏弹窗时触发的动作                                       |

绑定 afterAction 事件

```js
$("#myDialog").afterAction(function(){
	// todo
	console.log("Dialog afterAction");
});
```

绑定 hideAction 事件

```js
$("#myDialog").hideAction(function(){
	// todo
	console.log("Dialog hideAction");
});
```

设置返回值并触发 afterAction

```js
//API
setPopupReturnValue(node, jsonObject, close);

//示例：
setPopupReturnValue(this, { 'RET_VAL1' : (new Date()).format('yyyy-MM-dd HH:mm:ss'), 'RET_VAL2' : (new Date()).format('yyyy-MM-dd HH:mm:ss') })
```

| 参数名     | 参数类型      | 是否必须 | 默认值 | 描述                                                         |
| ---------- | ------------- | -------- | ------ | ------------------------------------------------------------ |
| node       | 原生 DOM 对象 | 是       | 无     | 通常是指点击的按钮对象，如果在 js 中直接调用，可传一个目标弹窗对应界面中的一个 DOM 对象。 |
| jsonObject | json 对象     | 否       | 无     | 设置上级弹窗元素值，例（{'custName':'谢小星', 'birthday':'1983-12-22'}） |
| close      | boolean       | 是       | true   | 是否关闭弹窗，默认关闭                                       |

## 设置白色背景标题

js 方式
```html
<div class="c_dialog" id="myDialog" style="display:none;">
	<div class="wrapper">
		<div class="header header-white">
			<div class="text">标题文字</div>
			<div class="fn">
				<div class="close"></div>
			</div>
		</div>
		<div class="content">
			<!-- 内容 -->
		</div>
	</div>
</div>
```

## 设置无遮罩

js 方式
```html
<div class="c_dialog c_dialog-nobg" id="myDialog" style="display:none;">
	<div class="wrapper">
		<div class="header">
			<div class="text">标题文字</div>
			<div class="fn">
				<div class="close"></div>
			</div>
		</div>
		<div class="content">
			<!-- 内容 -->
		</div>
	</div>
</div>
```

## 样式

| 参数名     | 值           |  默认值 | 描述     |
| ---------- | ------------ | ------ | -------- |
| c_dialog-nobg |c_dialog-nobg |  无     | 设置无遮罩 |

## 局部样式

### &lt;div class="header"&gt;

| 参数名     | 值           |  默认值 | 描述     |
| ---------- | ------------ | ------ | -------- |
| header-white |header-white |  无     | 设置白色背景标题 |

## 响应规则

| 参数名                 | 值|描述 |演示 |
| -------------------- | -|--- |--- |
| c_dialog-phone-full | c_param-phone-full|手机端全屏显示|<a href="#/example/component/dialog-phone-full" target="_blank">查看</a>|