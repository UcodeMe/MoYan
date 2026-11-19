# 分页标签（c_tab / @Tabset）
通过切换的方式在同一视窗展示多个信息处于平级的界面区域。 

## JWC 方式

<a href="#/example/component/tab" target="_blank">查看演示</a>

```html
<div name="myTab" jwcid="@Tabset">
	<div title="Tab Label A" class="content">
		<!-- Tab Content A -->
	</div>
	<div title="Tab Label B" class="content">
		<!-- Tab Content B -->
	</div>
	<div title="Tab Label C" class="content">
		<!-- Tab Content C -->
	</div>
</div>
```

## JS 方式
html
```html
<div class="c_tab" id="myTab">
	<div class="tab">
		<div class="list">
			<ul></ul>
		</div>
	</div>
	<div class="page">
		<div class="content" title="Tab Label A">
			<!-- Tab Content A -->
		</div>
		<div class="content" title="Tab Label B">
			<!-- Tab Content B -->
		</div>
		<div class="content" title="Tab Label C">
			<!-- Tab Content C -->
		</div>
	</div>
</div>
```
js
```js
//注意实例名必须与 .c_tab 元素的id一致
window["myTab"] = new Wade.Tabset("myTab");
```

## 属性


| 参数名        | 参数类型         | 是否必须 | 默认值 | 描述                                                         |
| ------------- | ---------------- | -------- | ------ | ------------------------------------------------------------ |
| name          | java.lang.String | 是       | 无     | 元素 id 和 name 属性的值，也是提交表单数据时的字段名字       |
| class         | java.lang.String | 否       | 无     | 自定义样式名                                                 |
| style         | java.lang.String | 否       | 无     | 自定义样式                                                   |
| title         | java.lang.String | 否       | 无     | 标题，注意：该属性有值时，tabset 的样式会强制设置为 "c_tab-title c_tab-sp" |
| titleTip      | java.lang.String | 否       | 无     | 标题提示信息，仅在 title 属性不为空时有效                    |
| titleTipColor | java.lang.String | 否       | red    | 标题提示信息颜色，仅在 title 属性不为空时有效                |
| slidable      | boolean          | 否       | true   | 是否采用平滑模式，关闭此模式则在触屏端无法使用滑动触控，同时每个标签页内容的高度均由其内容决定 |

## 关闭滑动模式
jwc 方式
```html
<div name="myTab" slidable="false" jwcid="@Tabset">
	<div title="Tab Label A" class="content">
		<!-- Tab Content A -->
	</div>
	<div title="Tab Label B" class="content">
		<!-- Tab Content B -->
	</div>
	<div title="Tab Label C" class="content">
		<!-- Tab Content C -->
	</div>
</div>
```
js 方式
```js
window["myTab"] = new Wade.Tabset("myTab",{
	slidable:false
});
```




## 插件

### 插入标题 
jwc 方式
```html
<div jwcid="@Tabset"  name="myTab"  title="The tab component" class="c_tab">
	<div title="Tab Label A" class="content">
		<!-- 内容 -->
		<div class="c_msg">
			<div class="wrapper">
				<div class="emote"></div>
				<div class="info">
					<div class="text">
						<div class="title">Tab Content A</div>
					</div>
				</div>
			</div>
		</div>
		<!-- 内容 -->
	</div>
	<div title="Tab Label B" class="content">
		<!-- 内容 -->
		<div class="c_msg">
			<div class="wrapper">
				<div class="emote"></div>
				<div class="info">
					<div class="text">
						<div class="title">Tab Content B</div>
					</div>
				</div>
			</div>
		</div>
		<!-- 内容 -->
	</div>
	<div title="Tab Label C" class="content">
		<!-- 内容 -->
		<div class="c_msg">
			<div class="wrapper">
				<div class="emote"></div>
				<div class="info">
					<div class="text">
						<div class="title">Tab Content C</div>
					</div>
				</div>
			</div>
		</div>
		<!-- 内容 -->
	</div>
</div>
```

js 方式
```html
<div class="c_tab c_tab-sp c_tab-title" id="myTab">
	<div class="tab">
		<div class="title">The tab component</div>
		<div class="list">
			<ul></ul>
		</div>
	</div>
	<div class="page">
		<div class="content" title="Tab Label A">
			<!-- Tab Content A -->
		</div>
		<div class="content" title="Tab Label B">
			<!-- Tab Content B -->
		</div>
		<div class="content" title="Tab Label C">
			<!-- Tab Content C -->
		</div>
	</div>
</div>
```

### 插入功能
jwc 方式
<a href="#/example/component/tab-fn" target="_blank">查看演示</a>

```html
<div jwcid="@Tabset" name="myTab" class="c_tab">
	<div class="fn">
		<button class="e_button-blue"><span class="e_ico-config"></span></button>
	</div>
	<div title="Tab Label A" class="content">
		<!-- 内容 -->
		<div class="c_msg">
			<div class="wrapper">
				<div class="emote"></div>
				<div class="info">
					<div class="text">
						<div class="title">Tab Content A</div>
					</div>
				</div>
			</div>
		</div>
		<!-- 内容 -->
	</div>
	<div title="Tab Label B" class="content">
		<!-- 内容 -->
		<div class="c_msg">
			<div class="wrapper">
				<div class="emote"></div>
				<div class="info">
					<div class="text">
						<div class="title">Tab Content B</div>
					</div>
				</div>
			</div>
		</div>
		<!-- 内容 -->
	</div>
	<div title="Tab Label C" class="content">
		<!-- 内容 -->
		<div class="c_msg">
			<div class="wrapper">
				<div class="emote"></div>
				<div class="info">
					<div class="text">
						<div class="title">Tab Content C</div>
					</div>
				</div>
			</div>
		</div>
		<!-- 内容 -->
	</div>
</div>
```
js 方式

```html
<div class="c_tab" id="myTab">
	<div class="tab">
		<div class="list">
			<ul></ul>
		</div>
		<div class="fn">
			<button>Share</button>
			<button>Edit</button>
		</div>
	</div>
	<div class="page">
		<div class="content" title="Tab Label A">
			<!-- Tab Content A -->
		</div>
		<div class="content" title="Tab Label B">
			<!-- Tab Content B -->
		</div>
		<div class="content" title="Tab Label C">
			<!-- Tab Content C -->
		</div>
	</div>
</div>
```

### 复合案例
jwc 方式
<a href="#/example/component/tab-mix" target="_blank">查看演示</a>

```html
<div class="c_tab c_tab-border c_tab-box" name="myTab" jwcid="@Tabset">
	<div class="fn">
		<span class="e_mix e_mix-r">
			<input type="text" />
			<button class="e_button-blue"><span class="e_ico-search"></span></button>
		</span>
		<span class="e_space"></span>
		<button class="e_button-r e_button-blue"><span class="e_ico-edit"></span><span>Edit</span></button>
	</div>
	<div title="Tab Content A" class="content">
		<!-- 内容 -->
		<div class="c_msg">
			<div class="wrapper">
				<div class="emote"></div>
				<div class="info">
					<div class="text">
						<div class="title">Tab Content A</div>
					</div>
				</div>
			</div>
		</div>
		<!-- 内容 -->
	</div>
	<div title="Tab Content B" class="content">
		<!-- 内容 -->
		<div class="c_msg">
			<div class="wrapper">
				<div class="emote"></div>
				<div class="info">
					<div class="text">
						<div class="title">Tab Content B</div>
					</div>
				</div>
			</div>
		</div>
		<!-- 内容 -->
	</div>
</div>
```

js 方式
html
```html
<div class="c_tab c_tab-border c_tab-box" id="myTab">
	<div class="tab">
		<div class="list">
			<ul>
			</ul>
		</div>
		<div class="fn">
			<span class="e_mix e_mix-r">
				<input type="text" />
				<button class="e_button-blue"><span class="e_ico-search"></span></button>
			</span>
			<span class="e_space"></span>
			<button class="e_button-r e_button-blue"><span class="e_ico-edit"></span><span>Edit</span></button>
		</div>
	</div>
	<div class="page">
		<div class="content" title="resume">
			<!-- resume Content -->
		</div>
		<div class="content" title="estimation">
			<!-- estimation Content -->
		</div>
	</div>
</div>
```
js
```js
window["myTab"] = new Wade.Tabset("myTab");
```

## JS API

<a href="#/example/component/tab-jsApi" target="_blank">查看演示</a>

切换事件

```js
$("#myTab").afterSwitchAction(function(e, idx, title){
	// return window.confirm("确定要切换标签页吗？");
});
```

切换事件（滑动切换时不会触发）

```js
$("#myTab").switchAction(function(e, idx, title){
	// return window.confirm("确定要切换标签页吗？");
});
```

获取当前页签序号

```js
mytab.activeIdx
```

动态增加一个标签页

```js
var idx = myTab.add("新增tab", {
	html: "<div></div>"
});
```

```js
var idx = myTab.add("新增tab", {
	page: "examples.component.Popup",
	listener: "init",
	params: "a=b"
});
```

删除一个标签页

```js
var idx = myTab.remove("tabTitle1");
```

```js
var idx = myTab.removeByIdx(0);
```

按标签页序号切换

```js
myTab.switchTo(idx);
```

按标签页标题切换

```js
myTab.switchToByTitle("new tab");
```

更新标题

```js
myTab.updateTitle("new label text");
```

更新标题提示

```js
myTab.updateTitleTip("new tip");
```

刷新

```js
myTab.adjust();
```

## 样式

| 参数名     | 值 |  默认值 | 描述     |演示     |
| ---------- | --- | ------ | -------- |---- |
| c_tab-v |c_tab-v |  无     | 垂直布局 |<a href="#/example/component/tab-v" target="_blank">查看</a>|
| c_tab-level-2 |c_tab-level-2 |  无     | 嵌套时二级外观 |<a href="#/example/component/tab-level" target="_blank">查看</a>|
| c_tab-sub |c_tab-sub |  无     | 次级外观 |<a href="#/example/component/tab-sub" target="_blank">查看</a>|
| c_tab-sp |c_tab-sp |  无     | 特殊样式 |<a href="#/example/component/tab-sp" target="_blank">查看</a>|
| c_tab-box |c_tab-box |  无     | 适用于块内的样式 |<a href="#/example/component/tab-box" target="_blank">查看</a>|
| c_tab-avg |c_tab-avg |  无     | 页签均分 |<a href="#/example/component/tab-avg" target="_blank">查看</a>|
| c_tab-border |c_tab-border |  无     | 外边框 |<a href="#/example/component/tab-border" target="_blank">查看</a>|
| c_tab-gd |c_tab-gd |  无     | 渐变 |<a href="#/example/component/tab-gd" target="_blank">查看</a>|

