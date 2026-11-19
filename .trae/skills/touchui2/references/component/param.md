# 参数列表（c_param）
布局参数信息的列表。

## HTML
<a href="#/example/component/param" target="_blank">查看</a>
```html
<div class="c_param">
	<ul>
		<li>
			<div class="label">导演：</div>
			<div class="value">乔斯·韦登</div>
		</li>
	</ul>
</div>
```

## 插件
### 纵向右边区域
<a href="#/example/component/param-side" target="_blank">查看演示</a>

```html
<div class="c_param c_param-col-3 c_param-v">
	<div class="content">
		<ul>
			<li>
				<div class="label">参数</div>
				<div class="value">值</div>
			</li>
			<li>
				<div class="label">参数</div>
				<div class="value">值</div>
			</li>
			<li>
				<div class="label">参数</div>
				<div class="value">值</div>
			</li>
		</ul>
	</div>
	<div class="side">
		<button class="e_button-r"><span class="e_ico-change"></span><span>切换用户</span></button>
	</div>
</div>
```

## 样式

| 参数名          | 值                                                           | 默认值          | 描述             |  演示    |
| --------------- | ------------------------------------------------------------ | --------------- | ---------------- |---- |
| c_param-label-6 | c_param-label-2 ~ c_param-label-18<br />c_param-label-auto       | c_param-label-4 | 设置参数中文长度 |<a href="#/example/component/param-label" target="_blank">查看</a>|
| c_param-col-2   | c_param-col-2<br />c_param-col-3<br />c_param-col-4<br />c_param-col-5<br />c_param-col-6 | 无              | 设置分列         |<a href="#/example/component/param-col" target="_blank">查看</a>|
| c_param-v | c_param-v | 无  | 设置纵向 |<a href="#/example/component/param-v" target="_blank">查看</a>|
| c_param-avg | c_param-avg | 无  | 设置纵向平均分内容区域 |<a href="#/example/component/param-avg" target="_blank">查看</a>|
| c_param-border | c_param-border | 无  | 设置外边框 |<a href="#/example/component/param-border" target="_blank">查看</a>|
| c_param-nopadding | c_param-nopadding | 无  | 取消边距 |<a href="#/example/component/param-nopadding" target="_blank">查看</a>|
| c_param-blue | c_param-blue<br />c_param-gray | 无  | 设置背景色 |<a href="#/example/component/param-bg" target="_blank">查看</a>|
| c_param-s | c_param-s<br />c_param-m<br />c_param-l | c_param-m | 设置大小 ||
| c_param-nowrap | c_param-nowrap | 无  | 不换行设置 |<a href="#/example/component/param-nowrap" target="_blank">查看</a>|

## 局部样式

### &lt;li&gt;

| 参数名     | 值           |  默认值 | 描述     |
| ---------- | ------------ | ------ | -------- |
| merge-2 |merge-2<br />merge-3<br />merge-4 |  无     | 列分区设置 |
| wrap |wrap |  无     | 自动换行设置|

## 响应规则

| 参数名                 | 值|描述 |演示 |
| -------------------- | -|--- |--- |
| c_param-phone-col-1 | c_param-phone-col-1<br />c_param-phone-col-2<br />|手机端分列列数设置|<a href="#/example/component/param-phone-col" target="_blank">查看</a>|
| c_param-phone-h     | c_param-phone-h  | 手机端横向设置   |<a href="#/example/component/param-phone-h" target="_blank">查看</a>|

注意：c_param-phone-h 是针对 c_param-v 而言的，如果 PC 端设置了 c_param-v，则可以通过 c_param-phone-h 重置回来。 