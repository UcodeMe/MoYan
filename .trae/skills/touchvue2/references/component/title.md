# 标题（c_title）
一个页面通常包含多个模块，标题用于展示各模块的名称，及放置各模块的整体功能。

## HTML

<a href="#/example/component/title" target="_blank">查看演示</a>

```html
<div class="c_title">
	<div class="text">标题名称</div>
</div>
```

## 样式

| 参数名     | 值           |  默认值 | 描述     |演示     |
| ---------- | ------------ | ------ | -------- |-- |
| c_title-level-2 |c_title-level-2 |  无     | 二级标题 |<a href="#/example/component/title-level" target="_blank">查看</a>|
| c_title-nopadding |c_title-nopadding |  无     | 无内边距 |<a href="#/example/component/title-nopadding" target="_blank">查看</a>|
| c_title-s |c_title-s、c_title-m、c_title-l | c_title-m | 设置大小 ||

## 插入

### 插入操作（fn）

<a href="#/example/component/title-fn" target="_blank">查看演示</a>

```html
<div class="c_title">
	<div class="text">标题名称</div>
	<div class="fn">
		<ul>
			<li><span class="e_ico-add"></span>新增</li>
		</ul>
	</div>
</div>
```

### 插入类型

<a href="#/example/component/title-sort" target="_blank">查看演示</a>

```html
<div class="c_title">
	<div class="text">手机</div>
	<div class="sort">
		<ul>
			<li>Apple</li>
			<li>Huawei</li>
			<li>Xiaomi</li>
		</ul>
	</div>
</div>
```

### 右边插入文字

<a href="#/example/component/title-tip" target="_blank">查看演示</a>

```html
<div class="c_title">
	<div class="text">标题</div>
	<div class="tip">文字</div>
</div>
```

### 右边插入 select

<a href="#/example/component/title-select" target="_blank">查看演示</a>

```html
<div class="c_title">
	<div class="text">文字</div>
	<div class="fn">
		<ul>
			<li><span class="e_select">其他证件</span></li>
		</ul>
	</div>
</div>
```

### 右边插入按钮

<a href="#/example/component/title-button" target="_blank">查看</a>

```html
<div class="c_title">
	<div class="text">标题</div>
	<div class="fn">
		<button type="button">按钮</button>
	</div>
</div>
```

## 响应规则

| 参数名                |值| 描述 |演示 |
| ------------------- | ---|---- |--- |
| fn-phone-float |fn-phone-float|手机端隐藏并悬浮功能区域|<a href="#/example/component/title-fn-phone-float" target="_blank">查看</a>|


