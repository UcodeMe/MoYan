# 组合框（e_mix）
以顺序的方式组合各种表单元素的载体。

## HTML
<a href="#/example/element/mix" target="_blank">查看演示</a>
```html
<span class="e_group">
      <span class="e_groupRight">
            <span class="e_ico-date"></span>
      </span>
      <span class="e_groupMain">
            <input type="text" id="myDatefield" />
      </span>
</span>
```

## 样式

| 参数名     | 值           |  默认值 | 描述     |演示     |
| ---------- | ------------ | ------ | -------- |-------- |
| e_mix-prefix |e_mix-prefix |  无     | 前置元素固宽类：</br>在混合框（e_mix）的默认定义中，后置元素总是具备一定的宽度，由其内容的多少而决定，比如按钮、图标、下拉框等，而前置的元素则是根据整体宽度与后置元素宽度计算而来</br>即：前置元素宽度 = 整体宽度 - 后置元素宽度型，而“前置元素固宽”的方法，就是将这个规则逆转过来，使得前置元素可以使用按钮、图标、下拉框等，以适应更多的使用场景。 |<a href="#/example/element/mix-prefix" target="_blank">查看</a>|
| e_mix-avg |e_mix-avg |  无     | 等宽类型 |<a href="#/example/element/mix-avg" target="_blank">查看</a>|
| e_mix-search |e_mix-search |  无     | 美化版搜索框 |<a href="#/example/element/mix-search" target="_blank">查看</a>|
| e_mix-s |e_mix-s<br />e_mix-m<br />e_mix-l | e_mix-m | 大小设置 ||
| e_mix-r |e_mix-r |  无     | 圆角设置 |<a href="#/example/element/mix-r" target="_blank">查看</a>|
| e_mix-full |e_mix-full |  无     | 宽度100%设置 |<a href="#/example/element/mix-full" target="_blank">查看</a>|

## 插件
### 插入进度条

```html
<span class="e_mix">
	<input type="text" value="通讯录.xls" />
	<span class="e_ico-close"></span>
	<span class="e_mixProgress">
		<span class="e_mixBar" style="width:32.4%;"></span>
	</span>
</span>
```

