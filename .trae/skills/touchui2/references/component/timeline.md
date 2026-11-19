# 时间轴（c_timeline）
通过时间节点来串联一组相关信息。

## HTML

<a href="#/example/component/timeline" target="_blank">查看演示</a>

```html
<div class="c_timeline">
	<ul>
		<li>
			<div class="ico">
				<span class="e_ico-file"></span>
			</div>
			<div class="main">
				<div class="date">2017-10-12</div>
				<div class="content">计划阶段</div>
			</div>
		</li>
		<li>
			<div class="ico">
				<span class="e_ico-tool"></span>
			</div>
			<div class="main">
				<div class="date">2018-01-12</div>
				<div class="content">实施阶段</div>
			</div>
		</li>
		<li>
			<div class="ico">
				<span class="e_ico-diamond"></span>
			</div>
			<div class="main">
				<div class="date">2018-10-12</div>
				<div class="content">收尾阶段</div>
			</div>
		</li>
		<li>
			<div class="ico">
				<span class="e_ico-ok"></span>
			</div>
			<div class="main">
				<div class="date">2018-10-22</div>
				<div class="content">完成</div>
			</div>
		</li>
	</ul>
</div>
```

## 插件

### 日期节点

<a href="#/example/component/timeline-text" target="_blank">查看演示</a>

```html
<div class="c_timeline">
	<ul>
		<li>
			<div class="ico">
				<div class="text">2018</div>
			</div>
			<div class="main">
				<div class="date">03月</div>
				<div class="content"></div>
			</div>
		</li>
		<li class="level-2">
			<div class="ico"></div>
			<div class="main">
				<div class="date">06月</div>
				<div class="content"></div>
			</div>
		</li>
		<li class="level-2">
			<div class="ico"></div>
			<div class="main">
				<div class="date">12月</div>
				<div class="content"></div>
			</div>
		</li>
		<li>
			<div class="ico">
				<div class="text">2019</div>
			</div>
			<div class="main">
				<div class="date">03月</div>
				<div class="content"></div>
			</div>
		</li>
		<li class="level-2">
			<div class="ico"></div>
			<div class="main">
				<div class="date">06月</div>
				<div class="content"></div>
			</div>
		</li>
		<li class="level-2">
			<div class="ico"></div>
			<div class="main">
				<div class="date">12月</div>
				<div class="content"></div>
			</div>
		</li>
	</ul>
</div>
```

### 二级

<a href="#/example/component/timeline-level" target="_blank">查看演示</a>

```html
<div class="c_timeline">
	<ul>
		<li>
			<div class="ico">
				<span class="e_ico-date"></span>
			</div>
			<div class="main">
				<div class="date">2015-10-12</div>
				<div class="content">内容内容内容</div>
			</div>
		</li>
		<li class="level-2">
			<div class="ico"></div>
			<div class="main">
				<div class="date">2015-10-12</div>
				<div class="content">内容内容内容</div>
			</div>
		</li>
		<li class="level-2">
			<div class="ico"></div>
			<div class="main">
				<div class="date">2015-10-12</div>
				<div class="content">内容内容内容</div>
			</div>
		</li>
		<li>
			<div class="ico">
				<span class="e_ico-date"></span>
			</div>
			<div class="main">
				<div class="date">2015-10-12</div>
				<div class="content">内容内容内容</div>
			</div>
		</li>
	</ul>
</div>
```

## 样式

| 参数名              | 值                  | 默认值 | 描述         |演示         |
| ------------------- | ------------------- | ------ | ------------ |--- |
| c_timeline-twoSides | c_timeline-twoSides | 无     | 支持双边     |<a href="#/example/component/timeline-twoSides" target="_blank">查看</a>|
| c_timeline-sideTime | c_timeline-sideTime | 无     | 单侧日期     |<a href="#/example/component/timeline-sideTime" target="_blank">查看</a>|
| c_timeline-h | c_timeline-h | 无     | 横向     |<a href="#/example/component/timeline-h" target="_blank">查看</a>|

## 局部样式

### &lt;li&gt;

| 参数名     | 值           |  默认值 | 描述     |演示     |
| ---------- | ------------ | ------ | -------- |--- |
| on |on |  无     | 设置当前状态 |<a href="#/example/component/timeline-state" target="_blank">查看</a>|
| link |link |  无     | 支持鼠标滑过||