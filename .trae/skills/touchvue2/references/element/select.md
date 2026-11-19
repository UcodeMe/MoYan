# 下拉选择框（e_select / @Select）

选择框是基本的表单元素之一，用于从一组数据（4-30 条）中选取其中一条，给提交的数据提供来源。

## JWC 方式
<a href="#/example/element/select" target="_blank">查看演示</a>
```html
<span jwcid="@Select"
	name="mySelect"
	source="ognl:infos"
	addDefault="true"
	onchange="alert('change action:' + this.value + '|' + this.selectedText)"
	nullable="no"
	desc="选择项目"
></span>
```

## JS 方式一
html
```html
<span id="mySelectContainer"></span>
```

注意：VALUE 的值必须是字符串，不能使用数值
说明：在对象中插入 select 的方法，共有 4 种：

$.Select.append：
将代码插入在指定对象 innerHTML 的最后的位置

$.Select.prepend：
将代码插入在指定对象 innerHTML 的最前的位置

$.Select.before：
将代码插入在指定对象之前

$.Select.after：
将代码插入在指定对象之后

## JS 方式二

html
```html
<span class="e_select">
	<span>--请选择--</span>
	<input type="hidden" id="mySelect" value="" nullable="yes" desc="选择项目" />
</span>
```

选项 html
```html
<div id="mySelect_float" class="c_float">
	<div class="bg"></div>
	<div class="content">
		<div class="c_scrollContent">
			<div class="c_list c_list-pc-s c_list-phone-line ">
				<ul>
					<li class="link" idx="0" title="--请选择--" val="">
						<div class="main">--请选择--</div>
					</li>
					<li class="link" idx="1" title="title_0" val="value_0">
						<div class="main">选项0</div>
					</li>
					<li class="link" idx="2" title="title_1" val="value_1">
						<div class="main">选项1</div>
					</li>
					<li class="link" idx="3" title="title_2" val="value_2">
						<div class="main">选项2</div>
					</li>
				</ul>
			</div>
		</div>
	</div>
</div>
```
js
```js
$.Select.append(
	// 对应元素，在 el 元素下生成下拉框，el 可以为元素 id，或者原生 dom 对象
	"mySelectContainer",
	// 参数设置
	{
		id:"mySelect",
		name:"mySelect"
	},
	// 数据源，可以为 JSON 数组，或 JS 的 DatasetLsit 对象
	[
		{TEXT:"Tony Stark", VALUE:"0"},
		{TEXT:"Steve Rogers", VALUE:"1"},
		{TEXT:"Thor", VALUE:"2"}
	]
);
```

注：选项的 html 必须放在 body 的 append 的位置，id 与 select 中的隐藏域（input[type="hidden"]）的 id 相关联，多加一个 _float 的后缀。 



 ## 属性

| 参数名       | 参数类型         | 是否必须 | 默认值       | 描述                                                         |演示     |
| ------------ | ---------------- | -------- | ------------ | ------------------------------------------------------------ |---------- |
| name         | java.lang.String | 是       | 无           | 组件实例名，同一个页面内不能重复                             ||
| source       | java.lang.Object | 是       | 无           | 绑定的数据源, 必须实现 List 接口                             ||
| textField    | java.lang.String | 否       | 无           | 文本字段名                                                   ||
| valueField   | java.lang.String | 否       | 无           | 值字段名                                                     ||
| titleField   | java.lang.String | 否       | 无           | 标题域                                                       ||
| defaultText  | java.lang.String | 否       | "--请选择--" | 默认选项文本                                                 |<a href="#/example/element/select-defaultText" target="_blank">查看</a>|
| addDefault   | boolean          | 否       | true         | 是否显示默认选项                                             |<a href="#/example/element/select-addDefault" target="_blank">查看</a>|
| value        | java.lang.String | 否       | 无           | 值                                                           ||
| disabled     | boolean          | 否       | 无           | 是否禁用                                                     ||
| inputable    | boolean          | 否       | false        | 是否输选两用模式，开启后可以在输入框中输入关键字对选项进行检索排序，删除全部关键字后，select 值也为空 |<a href="#/example/element/select-inputable" target="_blank">查看</a>|
| class        | java.lang.String | 否       | 无           | 自定义样式名                                                 ||
| style        | java.lang.String | 否       | 无           | 自定义样式                                                   ||
| optionColumn | int              | 否       | 1            | 选项列表分列数                                               |<a href="#/example/element/select-optionColumn" target="_blank">查看</a>|
| optionWidth  | float            | 否       | 无           | 选项列表宽度，单位为“em”                                     |<a href="#/example/element/select-optionWidth" target="_blank">查看</a>|
| optionAlign  | java.lang.String | 否       | "left"       | 选项列表对齐方式                                             |<a href="#/example/element/select-optionAlign" target="_blank">查看</a>|
| showmatch    | boolean | 否       | true      | 如输入不是option的值,是否显示无匹配项，关闭设置成false                                       ||

## JS API
<a href="#/example/element/select-jsAPI" target="_blank">查看演示</a>
设置默认值

```js
$("#mySelect").val("2");
```

绑定 change 事件

```js
$("#mySelect").bind("change", function(){
	alert(this.value); // 当前值
	alert(this.selectedText); // 当前选中文本
});
```

绑定下拉框的 afterFilter 事件（仅当下拉框为输选两用时有效）

```js
$("#mySelect").bind('afterFilter', function(e, match){
	// todo
});
// 形参 e 为事件对象，match 为是否有匹配项 true|false
```

获取/设置下拉框的值

```js
$("#mySelect").val();
$("#mySelect").val(value);
```

全局允许通过 JS 动态设置选项中不存在的值
注：局部允许设置不存在的值，只需要将 showmatch 属性设置为 false 即可
```js
$.ENABLE_SELECT_NOMATCH = true;
$("#mySelect").val(value);
```

获取选中项序号

```js
mySelect.selectedIndex
```

获取选中项文本

```js
mySelect.selectedText
```

增加 select 项

```js
mySelect.append("text1","value1");
```

增加 select 项 同时新增title

```js
mySelect.append("text1","value1","value1");
```



删除 select 项

```js
mySelect.remove("value1");
```

清空 select 项（注：addDefault 为 true 时，默认选项不会被清除）

```js
mySelect.empty();
```


取消 select 禁用 -- 仅当设置了disabled="true"生效

```js
$('mySelect').attr('disabled', false); // 在页面加载完成以后执行
```


## 样式

| 参数名     | 值           |  默认值 | 描述     |演示     |
| ---------- | ------------ | ------ | -------- |-------- |
| e_select-r |e_select-r<br />e_select-left<br />e_select-right |  无     | 圆角设置 |<a href="#/example/element/select-r" target="_blank">查看</a>|


