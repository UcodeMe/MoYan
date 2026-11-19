# 过滤（c_filter）
通过标签的方式筛选一组数据。 

## HTML
<a href="#/example/component/filter" target="_blank">查看演示</a>

```html
<div class="c_filter">
	<ul>
		<li>
			<div class="label">品牌：</div>
			<div class="option">
				<ul>
					<li>不限</li>
					<li class="on">选中</li>
				</ul>
			</div>
		</li>
	</ul>
</div>
```
## 插件

### 支持折叠

<a href="#/example/component/filter-fold" target="_blank">查看演示</a>
```html
<div class="c_filter">
	<ul>
		<li>
			<div class="label">品牌：</div>
			<div class="more">
				<button class="e_button-s e_button-r" type="button">
					<span class="e_ico-unfold"></span><span>更多</span>
				</button>
			</div>
			<div class="option option-fold">
				<ul>
					<li>不限</li>
					<li class="on">选中</li>
				</ul>
			</div>
		</li>
	</ul>
</div>
```

## 样式

| 参数名                    | 值                                                     | 默认值           | 描述             |演示             |
| ------------------------- | ------------------------------------------------------ | ---------------- | ---------------- |--- | 
| c_filter-label-8          | c_filter-label-auto c_filter-label-2 ~ c_filter-label-18                   | c_filter-label-2 | 设置提示文字长度 |<a href="#/example/component/filter-label" target="_blank">查看</a>|
| c_filter-border           | c_filter-border                                        | 无               | 设置边框         |<a href="#/example/component/filter-border" target="_blank">查看</a>|


## 局部样式

### &lt;div class="option"&gt;&lt;/div&gt;

| 参数名     | 值           |  默认值 | 描述     |演示     |
| ---------- | ------------ | ------ | -------- |----- |
| option-multiple | option-multiple                                        | 无               | 支持多选         |<a href="#/example/component/filter-checkbox" target="_blank">查看</a>|
| option-fold | option-fold                                        | 无               | 支持折叠         |<a href="#/example/component/filter-fold" target="_blank">查看</a>|