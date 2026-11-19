# 组合框（e_group）
与 e_mix 组合框性质一样，都是组合各种表单元素的载体，其差异请参考<a href="#/doc/faq/groupOrMix" target="_blank">group 和 mix 有何区别</a>

## HTML
<a href="#/example/element/group" target="_blank">查看演示</a>
```html
<span class="e_group">
	<!-- 左侧显示内容 -->
	<span class="e_groupLeft">
		<span class="e_ico-search"></span>
	</span>
	<!-- 右侧显示内容 -->
	<span class="e_groupRight">
		<span class="e_ico-search"></span>
	</span>
	<!-- 自适应宽度的内容 -->
	<span class="e_groupMain">
		<input type="text" />
	</span>
</span>
```

## 插件
### 插入进度条

```html
<span class="e_group">
	<!-- 左侧显示内容 -->
	<span class="e_groupLeft">
		<span class="e_ico-search"></span>
	</span>
	<!-- 右侧显示内容 -->
	<span class="e_groupRight">
		<span class="e_ico-search"></span>
	</span>
	<!-- 自适应宽度的内容 -->
	<span class="e_groupMain">
		<input type="text" />
	</span>
	<!-- 进度条 -->
	<span class="e_groupProgress" style="width:60.9%;"></span>
</span>
```

## 样式

| 参数名     | 值           |  默认值 | 描述     |演示     |
| ---------- | ------------ | ------ | -------- |-------- |
| e_group-avg-2 |e_group-avg-2 ～ e_group-avg-13 |  无     | 等宽类型 |<a href="#/example/element/group-avg" target="_blank">查看</a>|
| e_group-r |e_group-r<br />e_group-left<br />e_group-right |  无     | 圆角设置 |<a href="#/example/element/group-r" target="_blank">查看</a>|
| e_group-full |e_group-full |  无     | 宽度100%设置 |<a href="#/example/element/group-full" target="_blank">查看</a>|

