# 如何给日期选择器等元素加上圆角

## 问题分析
日期选择器、IP 地址输入框等元素在 HTML 上是由多个表单元素组合而成的，因此其基本结构是一个组合框（e_group）：

```html
<span class="e_group">
	<span class="e_groupMain">
		<input type="text" />
	</span>
	<span class="e_groupMain">
		<span class="e_ico-date"></span>
	</span>
</span>
```

## 解决方法
给这些元素加上圆角，需要加上 ``e_group-r`` 参数：
```html
<input jwcid="@DateField" 
	class="e_group e_group-r"
	name="myDateField" 
	dropDown="true"
	value="1988-03-09"
	format="yyyy-MM-dd"
/>
```
