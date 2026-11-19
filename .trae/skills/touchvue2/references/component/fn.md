# 功能区（c_fn）
功能性质的表单元素进行独立布局的特属区域，便于分左右区域分置不同类型的功能。 

## HTML
<a href="#/example/component/fn" target="_blank">查看演示</a>

```html
<div class="c_fn">
	<div class="left">
		<button type="button"><span class="e_ico-add"></span><span>新增</span></button>
	</div>
	<div class="right">
		<span class="e_mix">
			<input type="text" />
			<button type="button" class="e_button-blue"><span class="e_ico-search"></span></button>
		</span>
	</div>
</div>
```
## 插件
插入标题

<a href="#/example/component/fn-text" target="_blank">查看演示</a>

```html
<div class="c_fn">
	<div class="left">
		<div class="text">标题</div>
	</div>
	<div class="right">
		...
	</div>
</div>
```

## 样式

| 参数名     | 值           |  默认值 | 描述     |
| ---------- | ------------ | ------ | -------- |
| c_fn-nav |c_fn-nav |  无   | 菜单模式 |

### &lt;div class="text"&gt;
| 参数名     | 值           |  默认值 | 描述     |
| ---------- | ------------ | ------ | -------- |
| text-nopadding |text-nopadding | 无 | 无边距 |