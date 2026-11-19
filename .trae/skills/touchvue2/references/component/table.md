---
trigger: model_decision
description: 表格组件，以栅格形式展示二维数据
---

# 表格（c_table/@Table）
以栅格的形式展示二维数据。

## JWC 方式
<a href="#/example/component/table" target="_blank">查看演示</a>

```html
<table name="myTable" jwcid="@Table" nodata="ognl:infoCount < 1">
	<thead>
		<tr>
			<th col="MY_COL_NAME">表头</th>
			<!-- 更多 <th> -->
		</tr>
	</thead>
	<tbody>
		<tr jwcid="@Foreach" source="ognl:infos" value="ognl:info" element="tr">
			<td>单元格</td>
			<!-- 更多 <td> -->
		</tr>
	</tbody>
</table>
```

## JS 方式

html

```html
<div id="myTable" class="c_table">
	<div class="body">
		<div class="wrapper">
			<table>
				<thead>
					<tr>
						<th>表头 A</th>
						<th>表头 B</th>
						<th>表头 C</th>
						<!-- 更多 <th> -->
					</tr>
				</thead>
				<tbody>
					<tr>
						<td data-title="表头A" class="key">单元格</td>
						<td data-title="表头B">单元格</td>
						<td data-title="表头C">单元格</td>
						<!-- 更多 <td> -->
					</tr>
					<!-- 更多 <tr> -->
				</tbody>
			</table>
		</div>
	</div>
	<div class="top">
		<table><thead></thead><tbody></tbody></table>
	</div>
	<div class="left" style="display:none">
		<table><thead></thead><tbody></tbody></table>
	</div>
	<div class="right" style="display:none">
		<table><thead></thead><tbody></tbody></table>
	</div>
	<div class="leftTop" style="display:none">
		<table><thead></thead><tbody></tbody></table>
	</div>
	<div class="rightTop" style="display:none">
		<table><thead></thead><tbody></tbody></table>
	</div>
</div>
```

js

```js
window["myTable"] = new Wade.Table("myTable", {
	fixedMode:true
});
```

## 属性

| 参数名         | 参数类型         | 是否必须 | 默认值 | 描述                                 |演示 |
| -------------- | ---------------- | -------- | ------ | ------------------------------------ |-------- |
| name           | java.lang.String | 是       | 无     | 组件实例名，同一个页面不能重复       ||
| class          | java.lang.String | 否       | 无     | 自定义样式名                         ||
| style          | java.lang.String | 否       | 无     | 自定义样式                           ||
| nodata         | boolean          | 否       | false  | 表格是否无数据<br />无数据时显示无数据提示 ||
| nodataTitle         | java.lang.String          | 否       | 无  | 表格无数据提示的提示标题 ||
| nodataContent         | java.lang.String          | 否       | 无  | 表格无数据提示的提示标题 ||
| useScroller    | boolean          | 否       | true   | 是否使用模拟scroller                 ||
| fixedMode      | boolean          | 否       | true   | 是否使用固定表头或表列               |<a href="#/example/component/table-fixedMode" target="_blank">查看</a>|
| editMode       | boolean          | 否       | true   | 是否编辑模式<br/>编辑模式响应行选择事件 ||
| fixedLeftCols  | Number           | 否       | 0      | 左侧固定列数                         |<a href="#/example/component/table-fixedLeftCols" target="_blank">查看</a>|
| fixedRightCols | Number           | 否       | 0      | 右侧固定列数                         |<a href="#/example/component/table-fixedLeftCols" target="_blank">查看</a>|
| useScroller | boolean           | 否       | false      | 可拖曳的滚动条                         |<a href="#/example/component/table-useScroller" target="_blank">查看</a>|

## 插件

### 树状模式 
<a href="#/example/component/table-tree" target="_blank">查看演示</a>
```html
<table name="myTable" class="c_table c_table-hasGrid" jwcid="@Table">
	<thead>
		<tr>
			<th col="MY_COL_NAME">表头</th>
			<!-- 更多 <th> -->
		</tr>
	</thead>
	<tbody>
		<tr jwcid="@Foreach" source="ognl:infos" value="ognl:info" element="tr">
			<td class="level level-2">
				<div class="ico"></div>
				<div class="checkbox"><input type="checkbox" /></div>
				<div class="text">CRM 二部</div>
			</td>
			<!-- 更多 <td> -->
		</tr>
	</tbody>
</table>
```
注意：使用表格树时，请同时加入 c_table-hasGrid 参数，防止手机端自动作出响应式处理，破坏树结构。 

### 插入多选框
<a href="#/example/component/table-checkbox" target="_blank">查看演示</a>
```html
<table name="myTable" class="c_table c_table-checkbox" jwcid="@Table">
	<thead>
		<tr>
			<th col="MY_CHECKBOX"><input type="checkbox" /></th>
			<th col="MY_COL_NAME">表头</th>
			<!-- 更多 <th> -->
		</tr>
	</thead>
	<tbody>
		<tr jwcid="@Foreach" source="ognl:infos" value="ognl:info" element="tr">
			<td><input type="checkbox" /></td>
			<td>单元格</td>
			<!-- 更多 <td> -->
		</tr>
	</tbody>
</table>
```

### 插入操作图标
<a href="#/example/component/table-fn" target="_blank">查看演示</a>
```html
<table name="myTable" jwcid="@Table">
	<thead>
		<tr>
			<th col="MY_COL_NAME">表头</th>
			<th col="FN" class="fn">删除</th>
			<!-- 更多 <th> -->
		</tr>
	</thead>
	<tbody>
		<tr jwcid="@Foreach" source="ognl:infos" value="ognl:info" element="tr">
			<td>单元格</td>
			<td class="fn"><span class="e_ico-delete"></span></td>
			<!-- 更多 <td> -->
		</tr>
	</tbody>
</table>
```

## 固定左侧列设置
jwc 方式

```html
<table name="myTable" fixedLeftCols="2" jwcid="@Table">
	<thead>
		<tr>
			<th col="MY_COL_NAME">表头</th>
			<!-- 更多 <th> -->
		</tr>
	</thead>
	<tbody>
		<tr jwcid="@Foreach" source="ognl:infos" value="ognl:info" element="tr">
			<td>单元格</td>
			<!-- 更多 <td> -->
		</tr>
	</tbody>
</table>
```
js 方式

```html
window["myTable"] = new Wade.Table("myTable", {
	fixedMode:true,
	fixedLeftCols:2
});
```

## 固定右侧列设置

jwc 方式
```html
<table name="myTable" fixedRightCols="1" jwcid="@Table">
	<thead>
		<tr>
			<th col="MY_COL_NAME">表头</th>
			<!-- 更多 <th> -->
		</tr>
	</thead>
	<tbody>
		<tr jwcid="@Foreach" source="ognl:infos" value="ognl:info" element="tr">
			<td>单元格</td>
			<!-- 更多 <td> -->
		</tr>
	</tbody>
</table>
```
js 方式

```js
window["myTable"] = new Wade.Table("myTable", {
	fixedMode:true,
	fixedRightCols:1
});
```

## 可拖曳的滚动条

jwc 方式

```html
<table name="myTable" fixedLeftCols="2" useScroller="true" jwcid="@Table">
	<thead>
		<tr>
			<th col="MY_COL_NAME">表头</th>
			<!-- 更多 <th> -->
		</tr>
	</thead>
	<tbody>
		<tr jwcid="@Foreach" source="ognl:infos" value="ognl:info" element="tr">
			<td>单元格</td>
			<!-- 更多 <td> -->
		</tr>
	</tbody>
</table>
```
js 方式

```js
window["myTable"] = new Wade.Table("myTable", {
	fixedMode:true,
	fixedLeftCols:2
	useScroller:true
});
```

## 表格排序

jwc 方式
注意：$.sortTable(obj, type, sortType) 表格排序，obj为th元素，type为排序数据类型，'int'为整型，'float'为浮点型，'date'为日期型，默认为字符串型。 
```html
<table name="myTable" jwcid="@Table">
	<thead>
		<tr>
			<th col="MY_COL_NAME" ontap="$.sortTable(this, 'int')"  >表头</th>
			<!-- 更多 <th> -->
		</tr>
	</thead>
	<tbody>
		<tr jwcid="@Foreach" source="ognl:infos" value="ognl:info" element="tr">
			<td>单元格</td>
			<!-- 更多 <td> -->
		</tr>
	</tbody>
</table>
```

js 方式
```js
function testSort(obj,type){
	//$.sortTable(obj, type, "desc");//可选排序方法1
	//$.sortTable(obj, fn, defCls); //可选排序方法2
	$.sortTable(obj, type, function(tr1,tr2){ //自定义排序方法
		var tr1Data = $(tr1).children("td:nth-child(2)").find("input:first").val();
		var tr2Data = $(tr2).children("td:nth-child(2)").find("input:first").val();
		return tr1Data - tr2Data;
	},"desc");
}
```

## JS API

<a href="#/example/component/table-jsApi" target="_blank">查看演示</a>

获取当前选中行的下标

```js
myTable.selected
```

新增行

```js
//新增行，在jsonData中设置"_className"属性值可以指定行样式
table.addRow(jsonData, afterSelected);

// 以下是参考例子
myTable.addRow({
	"_className":"strong",
	"CUST_ID":"12345",
	"CUST_NAME":"谢小星",
	"AGE" : "34",
	"CBX":"<input type=\"checkbox\" name=\"CBX_CUST_ID\" value=\"12345\" />",
	"FN":"<span ontap=\"deleteTableRow('myTable', this)\" class=\"e_ico-delete\"></span>"
});
```

| 参数名        | 参数类型 | 是否必须 | 默认值 | 描述                                                     |
| ------------- | -------- | -------- | ------ | -------------------------------------------------------- |
| jsonData      | json     | 是       | 无     | 新增的行数据（json 对象的 key 与 table 表头的 col 对应） |
| afterSelected | boolean  | 否       | false  | 新增行插入在选择行之后，否则插入在表格最后               |

注：表格新增的行，td 的 className 将继承对应 th 的 className。

更新表格行数据

```js
//更新下标为 rowIdx 的行的数据
table.updateRow(jsonData, rowIdx);

// 以下是参考例子
myTable.updateRow({"CUST_ID":"54321", "CUST_NAME":"客户姓名222"}, myTable.selected);
```

| 参数名   | 参数类型 | 是否必须 | 默认值 | 描述                                                     |
| -------- | -------- | -------- | ------ | -------------------------------------------------------- |
| jsonData | json     | 是       | 无     | 新增的行数据（json 对象的 key 与 table 表头的 col 对应） |
| rowIdx   | int      | 是       | 无     | 要更新的表格数据行下标                                   |

删除表格行

```js
//删除下标为 rowIdx 的行, remove 为 false 时，不移除行
table.deleteRow(rowIdx, remove);

// 以下是参考例子
myTable.deleteRow(myTable.selected);
myTable.deleteRow(myTable.selected, false);
```

| 参数名   | 参数类型 | 是否必须 | 默认值 | 描述                                                     |
| -------- | -------- | -------- | ------ | -------------------------------------------------------- |
| jsonData | json     | 是       | 无     | 新增的行数据（json 对象的 key 与 table 表头的 col 对应） |
| remove   | boolean  | 否       | false  | 是否移除行                                               |

获取表格全部行数据

```js
//获取表格全部操作行的数据
table.getData();

// 获取表格全部行数据
table.getData(true);

// 以下是参考例子
myTable.getData();
myTable.getData(true);
```

获取表格全部行的指定列数据

```js
//获取表格全部操作行的指定列数据
table.getData(col);

// 获取表格全部行的指定列数据
table.getData(true, col);

// 以下是参考例子
myTable.getData("AGE,CUST_ID");
myTable.getData(true, "AGE,CUST_ID");
```

| 参数名 | 参数类型 | 是否必须 | 默认值 | 描述                                          |
| ------ | -------- | -------- | ------ | --------------------------------------------- |
| isfull | string   | 否       | false  | 是否取全部行的数据，默认为false（可以不传）   |
| col    | string   | 否       | 无     | 列名，支持多个，用“,”隔开，如（"AGE","NAME"） |

获指定行数据

```js
// 获取下标为 rowIdx 的行数据
table.getRowData(rowIdx)

// 获取下标为 rowIdx 的行的指定列数据
table.getRowData(rowIdx, col)

// 以下是参考例子
myTable.getCheckedRowsData(1)
myTable.getCheckedRowsData(1, "AGE")
```

| 参数名 | 参数类型 | 是否必须 | 默认值 | 描述                                                         |
| ------ | -------- | -------- | ------ | ------------------------------------------------------------ |
| rowIdx | int      | 否       | 无     | 行下标，从 0 开始                                            |
| col    | string   | 否       | 无     | 列名，支持多个，用“,”隔开，如（"AGE","NAME"），不传列名时为获取全部数据。 |

获指定 checkboxName 的已勾选全部数据

```js
table.getCheckedRowsData(checkboxName, col)

// 以下是参考例子
myTable.getCheckedRowsData("CBX_CUST_ID", "AGE")
```

| 参数名       | 参数类型 | 是否必须 | 默认值 | 描述                                                         |
| ------------ | -------- | -------- | ------ | ------------------------------------------------------------ |
| checkboxName | string   | 否       | 无     | checkbox 的 name                                             |
| col          | string   | 否       | 无     | 列名，支持多个，用“,”隔开，如（"AGE","NAME"），不传列名时为获取全部数据。 |


表格排序

<a href="#/example/component/table-sortTable" target="_blank">查看演示</a>

```js
$.sortTable(obj, type, fn, defCls)
```

统一排序设置
```js
$.ENABLE_TABLE_ALL_COLUMN_SORT = true
```

特殊的情况（比如表格树），需要此列不支持排序的，可以设置th：``class="preventOrder"``

| 参数名       | 参数类型 | 是否必须 | 默认值 | 描述                                                         |
| ------------ | -------- | -------- | ------ | ------------------------------------------------------------ |
| obj          | DOM 对象 | 是       | 无     | th元素                                             |
| type         | string   | 否       | "string"| 排序数据类型，支持 "int". "float", "date" |
| fn         | function   | 否       | 无| 自定义排序方法 |
|defCls|string|否|无|默认排序样式|

例：
```html
<table name="myTable" jwcid="@Table">
	<thead>
		<tr>
			<th col="MY_COL_NAME" ontap="$.sortTable(this, 'int')">表头</th>
			<!-- 更多 <th> -->
		</tr>
	</thead>
	<tbody>
		<tr jwcid="@Foreach" source="ognl:infos" value="ognl:info" element="tr">
			<td>单元格</td>
			<!-- 更多 <td> -->
		</tr>
	</tbody>
</table>
```

例2：
```js
function testSort(obj,type){
	//$.sortTable(obj, type, "desc");//可选排序方法1
	//$.sortTable(obj, fn, defCls); //可选排序方法2
	$.sortTable(obj, type, function(tr1,tr2){ //自定义排序方法
		var tr1Data = $(tr1).children("td:nth-child(2)").find("input:first").val();
		var tr2Data = $(tr2).children("td:nth-child(2)").find("input:first").val();
		return tr1Data - tr2Data;
	},"desc");
}
```


table 表格绑定 tap 事件，获取选中行数据

注意：表格 tap 事件必须在页面加载事件里绑定

<a href="#/example/component/table-tap" target="_blank">查看演示</a>

```html
<div id="myQueryPart" jwcid="@Part">
	<table name="myTable" jwcid="@Table">
		<thead>
			<tr>
				<th col="MY_COL_NAME">表头</th>
				<!-- 更多 <th> -->
			</tr>
		</thead>
		<tbody>
			<tr jwcid="@Foreach" source="ognl:infos" value="ognl:info" element="tr">
				<td>单元格</td>
				<!-- 更多 <td> -->
			</tr>
		</tbody>
	</table>
</div>
```

```js
$(function(){
	$("#myQueryPart").tap(function(e){
		if(($(e.target).hasClass("fn") && $(e.target).find("span").hasClass("e_ico-delete")) || $(e.target).hasClass("e_ico-delete")) {
			myTable.deleteRow(myTable.selected, true);
		}
	});
});
```

| 参数名       | 参数类型 | 是否必须 | 默认值 | 描述                                                         |
| ------------ | -------- | -------- | ------ | ------------------------------------------------------------ |
| checkboxName | string   | 否       | 无     | checkbox 的 name                                             |
| col          | string   | 否       | 无     | 列名，支持多个，用“,”隔开，如（"AGE","NAME"），不传列名时为获取全部数据。 |

## 样式

| 参数名     | 值           |  默认值 | 描述     |演示     |
| ---------- | ------------ | ------ | -------- |-------- |
| c_table-lite |c_table-lite（ 注意：此参数仅对 pc 端 / pad 端或设置了反响应式指令（c_table-hasGrid）的手机端有效 ） |  无     | 轻量模式 |<a href="#/example/component/table-lite" target="_blank">查看</a>|
| c_table-even |c_table-even |  无     | 隔行换色设置 |<a href="#/example/component/table-even" target="_blank">查看</a>|
| c_table-th-hover |c_table-th-hover |  无     | 表头滑过变色设置 |<a href="#/example/component/table-thHover" target="_blank">查看</a>|
| c_table-hover |c_table-hover |  无     | 表身滑过变色设置 |<a href="#/example/component/table-hover" target="_blank">查看</a>|
| c_table-line |c_table-line |  无     | 分隔线设置 |<a href="#/example/component/table-line" target="_blank">查看</a>|
| c_table-noline |c_table-noline |  无     | 无分隔线设置 |<a href="#/example/component/table-noline" target="_blank">查看</a>|
| c_table-border |c_table-border |  无     | 边框设置 ||<a href="#/example/component/table-border" target="_blank">查看</a>|
| c_table-fixed |c_table-fixed |  无     | 固定列宽设置 |<a href="#/example/component/table-fixed" target="_blank">查看</a>|
| c_table-s |c_table-s<br />c_table-m<br />c_table-l | c_table-m | 大小设置 ||
| c_table-row-10 |c_table-row-3<br />c_table-row-5<br />c_table-row-10<br />c_table-row-15<br />c_table-row-20<br />c_table-full（全屏高度）<br />注意：c_table-full 只在父对象拥有高度时有效。 |  无     | 高度设置 |<a href="#/example/component/table-row" target="_blank">查看</a>|
| c_table-gd |c_table-gd |  无     | 渐变 |<a href="#/example/component/table-gd" target="_blank">查看</a>|
| c_table-head-2 |c_table-head-2<br />c_table-head-3<br />c_table-head-4<br />c_table-head-5<br />c_table-head-6<br /> |  无     | 表头行数 ||

## 局部样式

### &lt;tr&gt;

| 参数名     | 值           |  默认值 | 描述     |演示     |
| ---------- | ------------ | ------ | -------- |------- |
| on |on（当前）</br>deleted—（删除）</br>strong（加强）</br>new（新增） |  无     | 设置行状态|<a href="#/example/component/table-rowColor" target="_blank">查看</a>|

### &lt;th&gt;

| 参数名     | 值           |  默认值 | 描述     |演示     |
| ---------- | ------------ | ------ | -------- |------- |
| green |green |  无     | 列分区设置 |<a href="#/example/component/table-thTd" target="_blank">查看</a>|

### &lt;td&gt;

| 参数名     | 值           |  默认值 | 描述     |演示     |
| ---------- | ------------ | ------ | -------- |------- |
| wrap |wrap |  无     | 自动换行设置|<a href="#/example/component/table-wrap" target="_blank">查看</a>|

## 反响应规则

| 参数名     | 值|描述           |演示|
| ---------- | ------|--|---- |
| c_table-phone-v |c_table-phone-v |手机端自适应为垂直模式显示|<a href="#/example/component/table-phone-v" target="_blank">查看</a>|

