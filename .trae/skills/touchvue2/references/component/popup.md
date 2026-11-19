# 弹出层（c_popup/@Popup）
放置预隐藏的次级信息的载体，展示时从右侧滑入。

## JWC 方式

<a href="#/example/component/popup" target="_blank">查看演示</a>

```html
<div jwcid="@Popup" name="myPopup">
	<div class="c_popupGroup">
		<div id="myPopup_item" class="c_popupItem">
			<!-- 内容 -->
		</div>
	</div>
</div>
```

## JS 方式

html

```html
<div class="c_popup" id="myPopup">
	<div class="c_popupBg" id="myPopup_bg"></div>
	<div class="c_popupBox">
		<div class="c_popupWrapper" id="myPopup_wrapper">
			<div class="c_popupGroup">
				<div class="c_popupItem" id="myPopup_item">
					...
				</div>
			</div>
		</div>
	</div>
</div>
```
注意：c_popupBg 和 c_popupWrapper 的 id 均需要继承 c_popup 的前缀，c_popupItem 的 id 则可以任意命名。 

js
```js
window["myPopup"] = new Wade.Popup("myPopup",{
	visible:false,
	mask:true
});
```

注：@Popup 组件的结构必须写在滚动条之外，以免受其影响。

## 属性

| 参数名      | 参数类型         | 是否必须 | 默认值 | 描述                                                   |
| ----------- | ---------------- | -------- | ------ | ------------------------------------------------------ |
| name        | java.lang.String | 是       | 无     | 元素 id 和 name 属性的值，也是提交表单数据时的字段名字 |
| class       | java.lang.String | 否       | 无     | 自定义样式名                                           |
| style       | java.lang.String | 否       | 无     | 自定义样式                                             |
| mask        | boolean          | 否       | true   | 是否需要遮罩                                           |
| overlap     | boolean          | 否       | false  | 是否支持重叠                                           |
| hideAction  | java.lang.String | 否       | 无     | 隐藏时执行的方法                                       |
| afterAction | java.lang.String | 否       | 无     | 显示时执行的方法                                       |

## 插件

### 多级设置

<a href="#/example/component/popup-popupGroup" target="_blank">查看演示</a>

```html
<div jwcid="@Popup" name="myPopup">
	<div class="c_popupGroup">
		<div id="myPopup_item" class="c_popupItem">
			<!-- 内容 -->
		</div>
	</div>
	<div class="c_popupGroup">
		<div id="myPopup_item" class="c_popupItem">
			<!-- 内容 -->
		</div>
	</div>
</div>
```
### 半屏弹出全屏

<a href="#/example/component/popup-half-full" target="_blank">查看演示</a>

```html
<button type="button" ontap="showPopup('myPopup','myPopupHalfHasBg_item',true)" class="e_button-red">半屏-弹出-全屏</button>
<!-- 半屏带遮罩 开始-->
<div jwcid="@Popup" name="myPopup" class="c_popup c_popup-half c_popup-half-hasBg">
	<div class="c_popupGroup">
		<div class="c_popupItem" id="myPopupHalfHasBg_item">
			<div class="c_header">
				<div class="back" ontap="backPopup(this)">半屏标题君</div>
			</div>
			<div class="c_scroll c_scroll-float c_scroll-header">
				<div class="l_padding">
					<button type="button" ontap="showPopup('myPopup1','myPopupFull_item')">弹出全屏</button>
				</div>
			</div>
		</div>
	</div>
</div>
<!-- 半屏带遮罩 结束-->
<!-- 全屏 开始 -->
<div jwcid="@Popup" name="myPopup1" class="c_popup c_popup-full">
	<div class="c_popupGroup">
		<div class="c_popupItem" id="myPopupFull_item">
			<div class="c_header">
				<div class="back" ontap="backPopup(this)">全屏标题君</div>
			</div>
			<div class="c_scroll c_scroll-float c_scroll-header">
				<div class="l_padding">
					全屏了!
				</div>
			</div>
		</div>
	</div>
</div>
<!-- 全屏 结束 -->
```
### 半屏弹出半屏

<a href="#/example/component/popup-full-half" target="_blank">查看演示</a>

```html
<button type="button" ontap="showPopup('myPopup','myPopupFull_item',true)" class="e_button-red">全屏-弹出-半屏</button>
<!-- 全屏 开始 -->
<div jwcid="@Popup" name="myPopup" class="c_popup c_popup-full">
	<div class="c_popupGroup">
		<div class="c_popupItem" id="myPopupFull_item">
			<div class="c_header">
				<div class="back" ontap="backPopup(this)">全屏标题君</div>
			</div>
			<div class="c_scroll c_scroll-float c_scroll-header">
				<div class="l_padding">
					全屏了!
					<br />
					<br />
					<br />
					<button type="button" ontap="showPopup('myPopup1','myPopupHalfHasBg_item')">弹出半屏</button>
				</div>
			</div>
		</div>
	</div>
</div>
<!-- 全屏 结束 -->
<!-- 半屏带遮罩 开始-->
<div jwcid="@Popup" name="myPopup1" class="c_popup c_popup-half c_popup-half-hasBg">
	<div class="c_popupGroup">
		<div class="c_popupItem" id="myPopupHalfHasBg_item">
			<div class="c_header">
				<div class="back" ontap="backPopup(this)">半屏标题君</div>
			</div>
			<div class="c_scroll c_scroll-float c_scroll-header">
				<div class="l_padding">
				半屏，出现了！
				</div>
			</div>
		</div>
	</div>
</div>
<!-- 半屏带遮罩 开始-->
```

## JS API

<a href="#/example/component/popup-jsApi" target="_blank">查看演示</a>

显示对应 item

```js
showPopup('myPopup','myPopup-item',true);
```

| 参数名 | 参数类型               | 是否必须 | 默认值 | 描述                                                 |
| ------ | ---------------------- | -------- | ------ | ---------------------------------------------------- |
| node   | string / 原生 DOM 对象 | 是       | 无     | 目标 popup 或其 id                                   |
| item   | string / 原生 DOM 对象 | 是       | 无     | 目标 item 或其 id                                    |
| reset  | boolean                | 否       | false  | 是否重置 popup，在切换第一级 item 时建议加上此参数。 |

隐藏对应 item

```js
hidePopup('myPopup');
hidePopup(this); // 当按钮在对应 item 之内时可直接传 this 作为参数
```

显示对应 iframe item

```js
// API：
popupPage(title, page, listener, params, subsys, className, afterAction, hideAction);

// 示例：
popupPage('弹出标题', 'examples.PopupFrame', 'init', '&a=1&b=2', null, 'half', afterAct, hideAct);
```

取消绑定

```js
$("#myPopup").unbind("afterAction", callback);
```

点击背景不隐藏

```js
$(myPopup.bg).unbind("tap");
``



注意：示例中 className 参数传入的 `half` 是缩写形式，完整形式为：`c_popup c_popup-half`，popupPage 方法的 className 参数支持 `half` 与 `full`。

| 参数名   | 参数类型      | 是否必须 | 默认值 | 描述                                                         |
| -------- | ------------- | -------- | ------ | ------------------------------------------------------------ |
| node     | 原生 DOM 对象 | 是       | 无     | 通常是指点击的按钮对象，如果在 js 中直接调用，可传一个目标弹窗对应界面中的一个 DOM 对象。 |
| title    | string        | 是       | 无     | 弹窗标题，弹窗页面里有 c_header 时会自动设置，相同的标题只会弹一个弹窗 |
| page     | string        | 是       | 无     | application 文件中定义的页面名                               |
| listener | string        | 否       | 无     | 页面 java 类监听方法名                                       |
| params   | string        | 否       | 无     | 打开页面携带的 url 参数                                      |
| subsys   | string        | 否       | 无     | 子系统相对路径（例："/saleserv/saleserv"），跨模块打开页面的时候，需要传入此参数。 |

进入下一层级 item

```js
forwardPopup('myPopup','myPopup-item');
forwardPopup(this,'myPopup-item'); // 当按钮在对应 item 之内时可直接传 this 作为参数
```

进入下一层级 iframe item

```js
// API:
forwardPopup(node, title, page, listener, params, subsys);

// 示例：
forwardPopup(this, '测试弹窗 1', 'examples.PopupFrame', '&aa=1');
```

| 参数名   | 参数类型      | 是否必须 | 默认值 | 描述                                                         |
| -------- | ------------- | -------- | ------ | ------------------------------------------------------------ |
| node     | 原生 DOM 对象 | 是       | 无     | 通常是指点击的按钮对象，如果在 js 中直接调用，可传一个目标弹窗对应界面中的一个 DOM 对象。 |
| title    | string        | 是       | 无     | 弹窗标题，弹窗页面里有 c_header 时会自动设置，相同的标题只会弹一个弹窗 |
| page     | string        | 是       | 无     | application 文件中定义的页面名                               |
| listener | string        | 否       | 无     | 页面 java 类监听方法名                                       |
| params   | string        | 否       | 无     | 打开页面携带的 url 参数                                      |
| subsys   | string        | 否       | 无     | 子系统相对路径（例："/saleserv/saleserv"），跨模块打开页面的时候，需要传入此参数。 |

返回上一层级 item

```js
// API
backPopup(node, jsonObject);

// 示例:
backPopup(this);

// 示例:
backPopup(this, {'START_DATE' : (new Date()).format('yyyy-MM-dd HH:mm:ss')});
```

| 参数名     | 参数类型      | 是否必须 | 默认值 | 描述                                                         |
| ---------- | ------------- | -------- | ------ | ------------------------------------------------------------ |
| node       | 原生 DOM 对象 | 是       | 无     | 通常是指点击的按钮对象，如果在 js 中直接调用，可传一个目标弹窗对应界面中的一个 DOM 对象。 |
| jsonObject | json 对象     | 否       | 无     | 设置上级弹窗元素值，例（{'custName':'谢小星', 'birthday':'1983-12-22'}） |

绑定 afterAction 事件

```js
$("#myPopup").afterAction(function(){
	// do something
});
```

绑定 hideAction 事件

```js
$("#myPopup").hideAction(function(){
	// do something
});
```

获取当前popup显示隐藏状态
```js
// API
// return true / false
mypopup.visible
```

设置返回值并触发 afterAction

```js
// API
setPopupReturnValue(node, jsonObject, close);

// 示例：
setPopupReturnValue(this, {'RET_VAL1' : (new Date()).format('yyyy-MM-dd HH:mm:ss'), 'RET_VAL2' : (new Date()).format('yyyy-MM-dd HH:mm:ss') })
```

| 参数名     | 参数类型      | 是否必须 | 默认值 | 描述                                                         |
| ---------- | ------------- | -------- | ------ | ------------------------------------------------------------ |
| node       | 原生 DOM 对象 | 是       | 无     | 通常是指点击的按钮对象，如果在 js 中直接调用，可传一个目标弹窗对应界面中的一个 DOM 对象。 |
| jsonObject | json 对象     | 否       | 无     | 设置上级弹窗元素值，例（{'custName':'谢小星', 'birthday':'1983-12-22'}） |
| close      | boolean       | 是       | true   | 是否关闭弹窗，默认关闭                                       |

## 样式

| 参数名             | 值                  | 默认值 | 描述           |演示           |
| ------------------ | ------------------- | ------ | -------------- |------ |
| c_popup-half       | c_popup-half        | 无     | 半屏设置       |<a href="#/example/component/popup-half" target="_blank">查看</a>|
| c_popup-half-hasBg | c_popup-half-hasBg  | 无     | 带遮罩半屏设置 |<a href="#/example/component/popup-half-hasBg" target="_blank">查看</a>|
| c_popup-wide       | c_popup-wide        | 无     | 宽屏设置（3/4屏）       |<a href="#/example/component/popup-wide" target="_blank">查看</a>|
| c_popup-full       | c_popup-full        | 无     | 全屏设置       |<a href="#/example/component/popup-full" target="_blank">查看</a>|


