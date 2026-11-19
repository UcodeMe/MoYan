---
trigger: model_decision
description: Touch UI框架中多种间距控制方法的详细说明
---

# 关于间距的控制

Touch UI 框架中存在多种间距控制的方法，以下就其方法及其属性进行详细的阐述。

## 块间间距

使用<a href="/#/doc/component/space">块间间距组件（c_space）</a>可以控制两个组件或大的区域之间的距离。

![块间间距](../static/img/doc/gap1.png)

```html
<div jwcid="@Table">
	...
</div>
<div class="c_space"></div>
<div jwcid="@Navbar">
	...
</div>
```

它支持 c_space-1 ～ c_space-4 四个参数，用于控制间距的大小，并额外支持一个 c_space-c 的参数，当两个带有边框的组件叠加在一起时，交界处会有重复的分割线，带有 c_space-c 参数的 c_space 组件可以消除重复的分割线。

![块间间距](../static/img/doc/gap2.png)

```html
<div jwcid="@Table" class="c_table c_table-border">
	...
</div>
<div class="c_space c_space-c"></div>
<div jwcid="@Navbar" class="c_page c_page-border">
	...
</div>
```

## 元素间距

使用<a href="/#/doc/element/space">块间间距组件（e_space）</a>可以控制两个小的元素之间的距离。

![元素间距](../static/img/doc/gap3.png)

```html
<button type="button">编辑</button>
<span class="e_space"></span>
<button type="button">删除</button>
```

它支持 e_space-1 ～ e_space-4 四个参数，用于控制间距的大小。

## 内边距

使用<a href="/#/doc/layout/padding">内边距布局（l_padding）</a>可以控制容器与内容之间的距离。

![内边距](../static/img/doc/gap4.png)

```html
<div class="c_box">
	<div class="l_padding">
		<table jwcid="@Table">
			...
		</table>
	</div>
</div>
```

内边距除了 l_padding-1 ～ l_padding-4 四个参数用于控制间距的大小之外，还有 l_padding-side、l_padding-u 和 l_padding-n 三个属性。

1. **l_padding-side**，使得内边距只在两侧产生。

	![l_padding-side](../static/img/doc/gap5.png)

2. **l_padding-u**，使得除了顶部，其他方向均有边距，构成一个 u 字母的形状。

	![l_padding-u](../static/img/doc/gap6.png)

3. **l_padding-n**，使得除了底部，其他方向均有边距，构成一个 n 字母的形状。

	![l_padding-n](../static/img/doc/gap7.png)

## 全局间距

Touch UI 在 2.2 版本中提供了全局间距设置的方法，在下载包中，提供了 base-gap-s.css 与 base-gap-xs.css，引用它们即可缩小间距。

base.css 样式效果：

![base.css](../static/img/doc/gap8.png)

base-gap-xs.css 样式效果：

![base-gap-xs.css](../static/img/doc/gap9.png)

Touch UI 的全局间距可以随意控制，未来将在 “定制” 中提供，如若现在需要其他全局间距，可以通过网站首页底部的联系方式联系我们。
