# 查询（l_query）
基本的查询 / 维护页面布局方式，可以更加简便地实现全屏自适应的查询布局，并自动适应手机客户端。 

## HTML

<a href="#/example/layout/query" target="_blank">查看演示</a>

```html
<div class="l_query">
	<div class="l_queryFn">
		<!-- 操作功能 -->
	</div>
	<div class="l_queryResult">
		<!-- 表格数据 -->
	</div>
	<div class="l_queryMsg">
		<!-- 提示信息（无数据提示/loading） -->
	</div>
	<div class="l_queryPage">
		<!-- 分页 -->
	</div>
</div>
```

## 插件

### 插入提交按钮

<a href="#/example/layout/query-submit" target="_blank">查看演示</a>

```html
<div class="l_query l_query-submit">
	<div class="l_queryFn">
		<!-- 操作功能 -->
	</div>
	<div class="l_queryResult">
		<!-- 表格数据 -->
	</div>
	<div class="l_queryMsg">
		<!-- 提示信息（无数据提示/loading） -->
	</div>
	<div class="l_queryPage">
		<!-- 分页 -->
	</div>
	<span class="l_querySubmit">
		<div class="c_submit c_submit-full">
			<button type="button" class="e_button-r e_button-l e_button-blue">查询</button>
		</div>
	</span>
</div>
```

### 插入用户信息

<a href="#/example/layout/query-user" target="_blank">查看演示</a>

```html
<div class="l_query l_query-user">
	<div class="l_queryUser">
		<!-- 用户信息 -->
	</div>
	<div class="l_queryResult">
		<!-- 表格数据 -->
	</div>
	<div class="l_queryMsg">
		<!-- 提示信息（无数据提示/loading） -->
	</div>
	<div class="l_queryPage">
		<!-- 分页 -->
	</div>
</div>
```

### 插入分页标签

<a href="#/example/layout/query-tab" target="_blank">查看演示</a>

```html
<div class="l_query l_query-tab">
	<div class="l_queryFn" id="queryCond">
		<!-- 此处是 fn 组件，具体使用方法请查阅相关文档 -->
		<div class="c_fn">
			<div class="left">
				<button type="button"><span class="e_ico-add"></span><span>新增</span></button>
				<button type="button"><span class="e_ico-delete"></span><span>删除</span></button>
			</div>
			<div class="right">
				<div class="e_mix" ontap="showPopup('myPopup','myPopup_item')">
					<input type="text" />
					<button type="button" class="e_button-blue"><span class="e_ico-search"></span></button>
				</div>
			</div>
		</div>
	</div>
	<div id="queryPart" jwcid="@Part">
		<div class="l_queryResult">
			<div name="myTab" class="c_tab c_tab-full" jwcid="@Tabset">
				<div title="Tab Label A" class="content">
					<table jwcid="@Table" name="myTable1" class="c_table">
						<thead>
							<tr>
								<th col="MY_COL_NAME">名称</th>
								<th>规格</th>
								<th>数量</th>
							</tr>
						</thead>
						<tbody>
							<tr jwcid="@Foreach" source="ognl:row10" value="ognl:info" element="tr">
								<td><span jwcid="@Insert" value="ognl:info.text" /></td>
								<td><span jwcid="@Insert" value="ognl:info.spec" /></td>
								<td><span jwcid="@Insert" value="ognl:info.num" /></td>
							</tr>
						</tbody>
					</table>
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
			</div>
		</div>
	</div>
	<div class="l_queryPage">
		<div name="navbar" jwcid="@NavBar" cond="queryCond" count="ognl:infoCount" listener="queryList" part="queryPart" pageSize="5"></div>
	</div>
</div>
```

### 复合查询

<a href="#/example/layout/query-mix" target="_blank">查看演示</a>

```html
<div class="l_query l_query-tab">
	<div class="l_queryFn">
		<!-- 操作功能 -->
	</div>
	<div class="c_tab c_tab-box c_tab-full" id="tab">
		<div class="tab">
			<div class="list">
				<ul></ul>
			</div>
		</div>
		<div class="page">
			<div class="content" title="服务列表">
				<div class="l_queryResult">
					<!-- 表格数据 -->
				</div>
			</div>
			<div class="content" title="梦网开关">
				<div class="l_queryResult">
					<!-- 表格数据 -->
				</div>
			</div>
		</div>
	</div>
	<div class="l_queryPage">
		<!-- 分页 -->
	</div>
</div>
```


## 样式

| 参数名              | 值                  | 默认值 | 描述         |
| ------------------- | ------------------- | ------ | ------------ |
| l_query-nopage      | l_query-nopage      | 无     | 无分页       |
| l_query-nofn        | l_query-nofn        | 无     | 无功能       |
| l_query-phone-notop | l_query-phone-notop | 无     | 手机中全屏   |

## 局部样式

### &lt;div class="l_queryMsg"&gt;

| 参数名     | 值           |  默认值 | 描述     |
| ---------- | ------------ | ------ | -------- |
| l_queryMsg-notop |l_queryMsg-notop |  无     | 设置全屏提示 |

## 兼容手机

### 步骤一：加头部与功能区域
头部的功能区域已经预留出来了，我们得给手机客户端加上一个 c_header 区域，并注意以下几点。

加入 e_show-phone 样式，保证此区域仅手机端显示。

加入 fn 部分，及“查询”与“功能组”的控制按钮，与 l_queryFn 中的功能项一一对应。

```html
<div class="c_header e_show-phone">
	<div class="back">标题</div>
	<div class="fn">
		<span class="e_ico-search"></span>
		<span class="e_ico-menu"></span>
	</div>
</div>
<!-- 查询模版 开始 -->
<div class="l_query">
	...
</div>
<!-- 查询模版 结束 -->
```
### 步骤二：优化数据显示

给每组单元格设置一个 key，挑你认为可以作为记录标题的项（最好是名称，尽可能不要选择流水号之类的无意义字段）；

设置 td data-title="字段名"（使用了表格组件就不必做这一步了）。

如果表格中有 用来多选的checkbox 组，请务必给 c_table 加入参数即div class="c_table c_table-checkbox" 

```html
<tr>
	<td data-title="客户标识">0001472</td>
	<td data-title="客户姓名" class="key">Jet Li</td>
</tr>
```
### 步骤三：加入各种 js

使用手机端 + PC端兼容事件

一律采用点击事件：tap="yourFunction()"，如需使用滑动等其他事件请咨询平台组。

事件绑定的方式： 
```js
$("#id").tap(function);
```

设置当前菜单关闭按钮

手机的外框上没有设置界面的关闭按钮，关闭在页面中做，请使用closeNav()方法，加在 c_header 的 fn 区域 

```html
<div class="c_header e_show-phone">
	<div class="back" ontap="closeNav();">敢死队名单查询</div>
	<div class="fn">
		<span class="e_ico-search"></span>
		<span class="e_ico-menu"></span>
	</div>
</div>
```

设置表格控制按钮

如果控制按钮只有“导入、导出、增、删、改、查”，且总数不超过 3 个（包括 3 个），把 c_fn 区域中的按钮在 c_header 里再写一遍，分别配一个图标就可以了。

【注】按钮请按次序排列：导入、导出、增、删、改、查、更多功能。
```html
<div class="c_header e_show-phone">
	<div class="back">标题</div>
	<div class="fn">
		<span class="e_ico-search" ontap="alert('查询')"></span>
		<span class="e_ico-add" ontap="alert('删除')"></span>
		<span class="e_ico-delete" ontap="alert('新增')"></span>
	</div>
</div>
```

如果控制按钮含有“导入、导出、增、删、改、查”以外的内容，或项目大于 3 个（不包括 3 个），则需要添加一个 menu 图标，使用 showFloatLayer() 方法关联 c_fn 中的 left 层悬浮展示全部功能。 

```html
<div class="c_header e_show-phone">
	<div class="back">标题</div>
	<div class="fn">
		<span class="e_ico-search" ontap="alert('查询')"></span>
		<span class="e_ico-menu" ontap="showFloatLayer('funcArea','block')"></span>
	</div>
</div>
<div class="l_query">
	<div class="l_queryFn">
		<div class="c_fn">
			<div class="left" id="funcArea">
				<button type="button" ontap="alert('新增')"><span class="e_ico-add"></span><span>新增</span></button>
				<button type="button" ontap="alert('导入')"><span class="e_ico-import"></span><span>导入</span></button>
				<button type="button" ontap="alert('导出')"><span class="e_ico-export"></span><span>导出</span></button>
			</div>
			<div class="right">
				...
			</div>
		</div>
	</div>
	...
</div>
```