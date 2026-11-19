# 列表（c_list）
单维度展示一组处于平行关系的循环数据或表单。 

## HTML
<a href="#/example/component/list" target="_blank">查看演示</a>

### 一般模式

```html
<div class="c_list">
	<ul>
		<li>
			<div class="pic"><img src="..." alt="" /></div>
			<div class="main">
				<div class="title">标题文字</div>
				<div class="content">内容文字</div>
			</div>
			<div class="side">类别/属性</div>
			<div class="more"></div>
			<div class="fn">
				<span class="e_ico-delete"></span>
			</div>
		</li>
		<!-- 更多 <li> -->
	</ul>
</div>
```
### 表单模式

<a href="#/example/component/list-form" target="_blank">查看演示</a>

```html
<div class="c_list">
	<ul>
		<li>
			<div class="label">提示文字</div>
			<div class="value">
				<input type="text" />
			</div>
		</li>
		<!-- 更多 <li> -->
	</ul>
</div>
```

注意：

使用表单模式时，最好同时设置分隔线（c_list-line）和外边框（c_list-border），当表单**只有一列时**，可以用 **c_list-form** 的方式快捷的加入分隔线和外边框。

div.value 中可以放置任意表单元素，如输入框、下拉框、组合框等，如需插入纯文字，请使用 `<span class="text">文字</span>` 
**请特别留意，IE8 下最好加上样式属性`c_list-kv`，否则可能出现兼容性问题。** 

## 插件

### 内部分组设置

<a href="#/example/component/list-group" target="_blank">查看演示</a>

```html
<div class="c_list">
	<ul>
		<li>
			<div class="group link">
				<div class="content">
					<div class="pic"><img src="..." alt="" /></div>
					<div class="main">
						<div class="title">标题文字</div>
						<div class="content">内容文字</div>
					</div>
					<div class="side">类别/属性</div>
					<div class="more"></div>
				</div>
			</div>
			<div class="fn"><span class="e_ico-delete"></span></div>
		</li>
		<!-- 更多 <li> -->
	</ul>
</div>
```


### 遮罩（cover）

<a href="#/example/component/list-cover" target="_blank">查看演示</a>

```html
<div class="c_list c_list-v">
	<ul>
		<li class="link">
            <div class="pic"><img src="..." alt="" /></div>
            <div class="main">
                <div class="title">标题文字</div>
                <div class="content">内容文字</div>
            </div>
            <div class="cover">
                <div class="wrapper">
                    <div class="title">当前票数</div>
                    <div class="content">1131</div>
                    <div class="fn"><button type="button" class="e_button-r e_button-orange"><span class="e_ico-like"></span><span>投票</span></button></div>
                </div>
            </div>
		</li>
		<!-- 更多 <li> -->
	</ul>
</div>
```

### 角标（statu）

<a href="#/example/component/list-statu" target="_blank">查看演示</a>

注意：实际使用中一般只在垂直排列模式（c_list-v）下使用角标  说明：支持自定义方向、自定义颜色。 方向定义，支持：statu-left（默认值）、statu-right  颜色定义，支持：statu-green（默认值）、statu-orange、statu-blue、statu-navy、statu-gray、statu-red、statu-off 

```html
<div class="c_list">
	<ul>
		<li>
			<div class="pic"><img src="..." alt="" /></div>
			<div class="main">
				<div class="title">标题文字</div>
				<div class="content">内容文字</div>
			</div>
			<div class="side">类别/属性</div>
			<div class="more"></div>
			<div class="fn">
				<span class="e_ico-delete"></span>
			</div>
			<div class="statu">new</div>
		</li>
		<!-- 更多 <li> -->
	</ul>
</div>
```
ie8 ``li``下 需添加一层``.content``
```html
<div class="c_list">
	<ul>
		<li>
			<div class="content">
				<div class="pic"><img src="..." alt="" /></div>
				<div class="main">
					<div class="title">标题文字</div>
					<div class="content">内容文字</div>
				</div>
				<div class="side">类别/属性</div>
				<div class="more"></div>
				<div class="fn">
					<span class="e_ico-delete"></span>
				</div>
				<div class="statu">new</div>
			</div>
		</li>
	</ul>
</div>
```

### 插入次级区域

<a href="#/example/component/list-sub" target="_blank">查看演示</a>

```html
<div class="c_list">
	<ul>
		<li>
			<div class="content">
				<div class="pic"><img src="..." alt="" /></div>
				<div class="main">
					<div class="title">标题文字</div>
					<div class="content">内容文字</div>
				</div>
				<div class="side">类别/属性</div>
				<div class="more"></div>
				<div class="fn">
					<span class="e_ico-delete"></span>
				</div>
			</div>
			<div class="sub">
				<div class="pic"><img src="..." alt="" /></div>
				<div class="main">
					<div class="title">标题文字</div>
					<div class="content">内容文字</div>
				</div>
			</div>
		</li>
		<!-- 更多 <li> -->
	</ul>
</div>
```
注意：为了使视觉结构不致混乱，插入次级区域时，一般需要设置块间间距（c_list-space）与分隔线（c_list-line）。
说明：次级区域与主区域同样具备 main、side、fn、more 等区域，只是从整体上比主区域要略小。
说明：次级区域可以通过设置 class="sub sub-noline" 去掉分隔线。



### 插入图标

<a href="#/example/component/list-ico" target="_blank">查看演示</a>

```html
<div class="c_list">
	<ul>
		<li>
			<div class="ico"><span class="e_ico-user"></span></div>
			<div class="main">
				<div class="title">标题文字</div>
				<div class="content">内容文字</div>
			</div>
		</li>
		<!-- 更多 <li> -->
	</ul>
</div>
```

### 插入控制区域

<a href="#/example/component/list-op" target="_blank">查看演示</a>

```html
<div class="c_list c_list-space">
	<ul>
		<li>
			<div class="content">
				<div class="pic"><img src="..." alt="" /></div>
				<div class="main">
					<div class="title">标题文字</div>
					<div class="content">内容文字</div>
				</div>
				<div class="side">类别/属性</div>
				<div class="more"></div>
				<div class="fn">
					<span class="e_ico-delete"></span>
				</div>
			</div>
			<div class="op">
				<ul>
					<li>删除</li>
					<li>修改</li>
					<li>分享</li>
				</ul>
			</div>
		</li>
		<!-- 更多 <li> -->
	</ul>
</div>
```
注意：为了使视觉结构不致混乱，插入次级区域时，一般需要设置块间间距（c_list-space）。 

## 样式

| 参数名     | 值           |  默认值 | 描述     |演示     |
| ---------- | ------------ | ------ | -------- |---- |
| c_list-v |c_list-v |  无     | 垂直排列模式 |<a href="#/example/component/list-v" target="_blank">查看</a>|
| c_list-line |c_list-line |  无     | 分隔线设置 |<a href="#/example/component/list-line" target="_blank">查看</a>|
| c_list-border |c_list-border |  无     | 边框设置 |<a href="#/example/component/list-border" target="_blank">查看</a>|
| c_list-space |c_list-space |  无     | 块间间距设置 |<a href="#/example/component/list-space" target="_blank">查看</a>|
| c_list-blue |c_list-green<br />c_list-blue<br />c_list-orange<br />c_list-red<br />c_list-gray<br />c_list-gray-light |  无     | 整体背景色设置 |<a href="#/example/component/list-bg" target="_blank">查看</a>|
| c_list-hideFn |c_list-hideFn |  无     | 隐藏功能（fn） |<a href="#/example/component/list-hideFn" target="_blank">查看</a>|
| c_list-hideSide |c_list-hideSide |  无     | 隐藏侧栏 |<a href="#/example/component/list-hideSide" target="_blank">查看</a>|
| c_list-col-3 |c_list-col-1 ~ c_list-col-6 | c_list-col-1 | 分列设置 |<a href="#/example/component/list-col" target="_blank">查看</a>|
| c_list-auto |c_list-auto |  无     | 自动横排设置 |<a href="#/example/component/list-auto" target="_blank">查看</a>|
| pic-middle |pic-middle |  无     | 图片垂直居中设置 ||
| c_list-m |c_list-xs<br />c_list-s<br />c_list-m<br />c_list-l | c_list-m | 大小设置 ||
| c_list-fixWrapSpace |c_list-fixWrapSpace | 无 | 多列叠加问题修复 |<a href="#/example/component/list-fixWrapSpace" target="_blank">查看</a>|

## 局部样式

### &lt;li&gt;

| 参数名     | 值           |  默认值 | 描述     |演示     |
| ---------- | ------------ | ------ | -------- |------ |
| on |on |  无     | 状态设置 |<a href="#/example/component/list-state" target="_blank">查看</a>|
| checked |checked |  无     | 选定状态 ||
| link |link |  无     | 链接 |<a href="#/example/component/list-link" target="_blank">查看</a>|
| cur |cur |  无     | 当前项 ||
| required |required |  无     | 必填/必选项 |<a href="#/example/component/list-state" target="_blank">查看</a>|
| blue |green<br />blue<br />orange<br />red<br />gray<br />gray-light |  无     | 单元背景色设置 |<a href="#/example/component/list-liBg" target="_blank">查看</a>|
| merge-2 |merge-2<br />merge-3<br />merge-4 |  无     | 合并列<br />支持：小于或等于分列的值<br />如：c_list-col-3 下，支持 merge-2、merge-3 |<a href="#/example/component/list-merge" target="_blank">查看</a>|

### &lt;div class="content"&gt;
| 参数名     | 值           |  默认值 | 描述     |演示     |
| ---------- | ------------ | ------ | -------- |---- |
| content-row-3 |content-row-1 ～ content-row-4<br />content-auto（不限高度） | content-row-1 | 内容高度限制设置 |<a href="#/example/component/list-contentRow" target="_blank">查看</a>|

### &lt;div class="pic"&gt;
| 参数名     | 值           |  默认值 | 描述     |演示 |
| ---------- | ------------ | ------ | -------- |-------- |
| pic-middle |pic-middle | 无 | 图片垂直居中设置 |<a href="#/example/component/list-pic-middle" target="_blank">查看</a>|

### &lt;div class="fn"&gt;
| 参数名     | 值           |  默认值 | 描述     |
| ---------- | ------------ | ------ | -------- |
| fn-lite |fn-lite | 无 | 简化功能图标 |<a href="#/example/component/list-fn-lite" target="_blank">查看</a>|

## 响应规则

| 参数名      |值| 描述     |演示     |
| ---------- | ------------ |----|- |
| c_list-phone-col-1 |c_list-phone-col-1<br />c_list-phone-col-2<br />c_list-phone-col-3<br/>c_list-phone-col-4<br/>c_list-phone-col-5<br/>|手机端的分列列数设置|<a href="#/example/component/list-phone-col" target="_blank">查看</a>|
| c_list-phone-line |c_list-phone-line|手机端下显示分割线|<a href="#/example/component/list-phone-line" target="_blank">查看</a>|