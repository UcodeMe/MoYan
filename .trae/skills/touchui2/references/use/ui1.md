# UI 设计-初级篇


## 一、Touch UI 中的命名秘法
+ e_ 开头的是元素（Element）
+ c_ 开头的是组件（Component）
+ l_ 开头的是布局 （Layout）
+ @开头的是jwc动态控件

类\_组件名-参数名-参数值<br/>
如：c\_list-col-2<br/>
c（组件）_ list(列表)-col(分列)-2(2列)<br/>


## 二、小实验
复制如下代码：
```html
<button class="e_button e_button-blue"  type="button">你好，我是一个按钮</button>
```
界面效果如下：<br/>
![IE8IconPosition](../static/img/doc/blueButton.png) <br/>
把代码修改成
```html
<button class="e_button e_button-red" type="button">你好，我是一个按钮</button>
```
界面效果如下：<br/>
![IE8IconPosition](../static/img/doc/redButton.png) <br/>

“e_button-blue” 改成 “e_button-red” 按钮颜色就不一样了，这个就是我们的**样式参数**，是不是有点意思了。

## 三、一般构建流程
布局（楼房结构）+ 组件（套内结构）+ 元素（门窗油漆），按照这个步骤，从 “文档” 里，选择需要的。

## 四、快速构建流程
 把主站 “实例” 中的 “模版” 浏览一遍，再看下图 <br/>
![IE8IconPosition](../static/img/doc/example.png) <br/>
### 1. 选择一个你需要的模版，把 “HTML 区域” 的代码，复制到你的 HTML 中。
接下来我以 “受理 / 默认模版” 为例.

### 2. “模拟数据”的代码，在项目资源中 “asset/js/” 新建一个 JS 文件放数据。

```html
<script type="text/javascript" src="../asset/js/data.js"></script>
```
现在你就可以访问本机的界面看看效果了。

### 3. 基于模版改成目标界面
#### A. 新增一个输入框

```html
<li class="link">
	<div class="label">职业</div>
	<div class="value">
		<span jwcid="@Select" name="SEL" source="ognl:vocation" textField="text"></span>
	</div>
</li>
```
新加一条 li 就行（<a href="#/doc/component/list" target="_blank">查看 c_list 文档</a>），如下代码：
```html
<li class="link">
	<div class="label">职业</div>
	<div class="value">
		<span jwcid="@Select" name="SEL" source="ognl:vocation" textField="text"></span>
	</div>
</li>
<li class="link">
	<div class="label">我的新增</div>
	<div class="value">
		<input type="text" placeholder="输入框"/>
	</div>
</li>
```
#### B. 修改模拟数据

```html
<li class="link">
	<div class="label">职业</div>
	<div class="value">
		<span jwcid="@Select" name="SEL" source="ognl:vocation" textField="text"></span>
	</div>
</li>
```
看到关键字 ``ognl``，在刚刚新建模拟数据的 JS 中，查询 “vocation”，进行你需要的修改就可以了。“vocation” 是可以自定义的，能与定义数据集名称一致就行。

```js
window.vocation = [{
		text: "UI设计师",
		value: "value1"
	},
	{
		text: "JAVA工程师",
		value: "value2"
	},
	{
		text: "架构师",
		value: "value3"
	},
	{
		text: "我修改的",
		value: "value4"
	},
	{
		text: "我新增的一条数据",
		value: "value5"
	}
];
```















