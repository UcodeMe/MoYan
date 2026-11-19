# 提交（c_submit）
提交页面级表单的按钮专属区域。

## HTML

<a href="#/example/component/submit" target="_blank">查看演示</a>

```html
<div class="c_submit e_center">
	<button type="button" class="e_button-l e_button-navy">取消</button>
	<button type="button" class="e_button-l e_button-blue">确定</button>
</div>
```

说明：c_submit 在不带参数的情况下通常与 e_center 一起使用。

## 插件

### 插入

<a href="#/example/component/submit-price" target="_blank">查看演示</a>

```html
<div class="c_submit c_submit-full">
	<div class="price"><span class="e_size-l e_red">总价：39.00 元</span></div>
	<button type="button" class="e_button-l e_button-navy">加入购物车</button>
	<button type="button" class="e_button-l e_button-green">立即订购</button>
</div>
```
依赖：c_submit-full 

### 插入

<a href="#/example/component/submit-mix" target="_blank">查看演示</a>

```html
<div class="c_submit c_submit-center">
	<span class="e_mix e_mix-r e_mix-l">
		<span class="e_label">发展员工</span>
		<input type="text" />
		<span class="e_ico-check"></span>
	</span>
	<button type="button" class="e_button-l e_button-navy">加入购物车</button>
	<button type="button" class="e_button-l e_button-green">立即订购</button>
</div>
```

## 样式

| 参数名          | 值              | 默认值 | 描述     |演示 | 
| --------------- | --------------- | ------ | -------- |---- |
| c_submit-full   | c_submit-full   | 无     | 两端对齐 |<a href="#/example/component/submit-full" target="_blank">查看</a>|
| c_submit-center | c_submit-center | 无     | 居中     |<a href="#/example/component/submit-center" target="_blank">查看</a>|

