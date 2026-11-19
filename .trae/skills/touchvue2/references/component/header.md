# 标题栏（c_header）
示意某区域内容性质，一般展示在容器性组件（如 popup、box）顶部。

## HTML
<a href="#/example/component/header" target="_blank">查看演示</a>
### 一般模式

```html
<div class="c_header">
	<div class="left">
		<!-- 内容 -->
	</div>
	<div class="right">
		<!-- 内容 -->
	</div>
	<div class="center">
		<!-- 内容 -->
	</div>
</div>
```
以 header 组件为载体，分左中右三个部分，各自插入不同的内容。

内容可以包含一些表单元素，比如：button、segment、switch 等，另外还可以插入以下特殊元素：

纯文字标题或说明

<a href="#/example/component/header" target="_blank">查看演示</a>

```html
<div class="text">页面标题</div>
```

带返回按钮的纯文字标题

<a href="#/example/component/header-button" target="_blank">查看演示</a>

```html
<div class="back">页面标题</div>
```

纯图标

```html
<span class="e_ico-edit"></span>
<span class="e_ico-delete"></span>
```

图文结合

<a href="#/example/component/header-texticon" target="_blank">查看演示</a>
	
```html
<div class="pt">
	<div class="ico"><span class="e_ico-scan"></span></div>
	<div class="text">扫一扫</div>
</div>
```

### 弹性模式
<a href="#/example/component/header-auto" target="_blank">查看演示</a>

左右两侧插入所需内容，auto 部分分配剩余宽度，内容部分参考一般模式的内容说明。

```html
<div class="c_header">
	<div class="left">
		<!-- 内容 -->
	</div>
	<div class="right">
		<!-- 内容 -->
	</div>
	<div class="auto">
		<!-- 内容 -->
	</div>
</div>
```

### 简写模式
<a href="#/example/component/header-lite" target="_blank">查看演示</a>

带返回标志

```html
<div class="c_header">
	<div class="back">页面标题</div>
	<div class="fn">
		<!-- 表单元素或纯图标 -->
	</div>
</div>
```

不带返回标志

```html
<div class="c_header">
	<div class="text">页面标题</div>
	<div class="fn">
		<!-- 表单元素或纯图标 -->
	</div>
</div>
```

## 样式

| 参数名     | 值           |  默认值 | 描述     |演示     
| ---------- | ------------ | ------ | -------- |---- |
| c_header-white |c_header-white |  无     | 白的背景 |<a href="#/example/component/header-white" target="_blank">查看</a>|
| c_header-nav |c_header-nav |  无     | 导航模式 |<a href="#/example/component/header-nav" target="_blank">查看</a>|
