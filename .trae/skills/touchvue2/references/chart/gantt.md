# 甘特图（c_gantt）


## HTML
<a href="#/example/chart/gantt" target="_blank">查看演示</a>						

```html
<div class="c_gantt">
	<div class="bg">
		<div class="month">
			<ul>
				<li>2018-03</li>
				<!-- 更多 <li> -->
			</ul>
		</div>
		<div class="week">
			<ul>
				<li></li>
				<!-- 更多 <li> -->
			</ul>
		</div>
	</div>
	<div class="content">
		<ul>
			<li style="margin-left:0.9em;">
				<div class="bar" style="width:6.9em;">
					<div class="progress" style="width:6.9em;"></div>
					<div class="title">需求分析需求分析需求分析需求分析</div>
				</div>
				<div class="info">负责人：<a href="#nogo">朱希</a></div>
				<div class="info">时间：2018-03-21～2018-04-20</div>
			</li>
			<!-- 更多 <li> -->
		</ul>
	</div>
</div>
```

## 样式

| 参数名       | 值           | 默认值 | 描述     |演示     |
| ------------ | ------------ | ------ | -------- |---- |
| c_gantt-border | c_gantt-border | 无     | 设置边框 |<a href="#/example/component/gantt-border" target="_blank">查看</a>|
| c_gantt-info | c_gantt-info | 无     | 显示信息部分 |<a href="#/example/component/gantt-info" target="_blank">查看</a>|
