# 分列（l_col）
将界面从水平方向上分作若干区域。

## HTML

<a href="#/example/layout/col" target="_blank">查看演示</a>

```html
<div class="l_col">
	<div class="l_colItem"> <!-- 第一列 -->
		<!-- 插入组件 -->
	</div>
	<div class="l_colItem"> <!-- 第二列 -->
		<!-- 插入组件 -->
	</div>
</div>
```
注意：需要多少列，就复制多少“l_colItem”，默认是平均分配宽度，宽度也可以自定义。宽度的单位可以是 em 和 % 。 

宽度设置

<a href="#/example/layout/col-width" target="_blank">查看演示</a>

```html
<div class="l_col">
	<div class="l_colItem" style="width:20%;">
		<div class="c_box c_box-blue"></div>
	</div>
	<div class="l_colItem">
		<div class="c_box c_box-green"></div>
	</div>
</div>
```

## 样式

| 参数名        | 值                                                         | 默认值        | 描述       | 演示       |
| ------------- | ---------------------------------------------------------- | ------------- | ---------- |----- |
| l_col-space-1 | l_col-space-1<br />l_col-space-2<br />l_col-space-3<br />l_col-space-4 | l_col-space-1 | 间距设置   |<a href="#/example/layout/col-space" target="_blank">查看</a>|
| l_col-line    | l_col-line                                                 | 无            | 分割线设置 |<a href="#/example/layout/col-line" target="_blank">查看</a>|
| l_col-full    | l_col-full                                                 | 无            | 全屏设置   |<a href="#/example/layout/col-full" target="_blank">查看</a>|
| l_col-middle | l_col-middle                                     | 无            | 垂直居中显示元素设置|<a href="#/example/layout/col-middle" target="_blank">查看</a>|

## 响应规则

| 参数名        |值       | 描述 |演示 |
| ----------|-------- | ---- |--- |
| l_col-phone-merge |l_col-phone-merge |手机端分列合并设置 |<a href="#/example/layout/col-phone-merge" target="_blank">查看</a>|
| l_col-phone-split-2|l_col-phone-split-2<br />l_col-phone-split-3|手机端页面分割设置|<a href="#/example/layout/col-phone-split" target="_blank">查看</a>|
| l_colItem-phone-float |l_colItem-phone-float|手机端页面叠加设置 |<a href="#/example/layout/col-phone-float" target="_blank">查看</a>|
| l_col-phone-sidebar |l_col-phone-sidebar| 手机端页面侧滑设置 |<a href="#/example/layout/col-phone-sidebar" target="_blank">查看</a>|
| l_col-phone-popup | l_col-phone-popup|手机端页面弹出设置 |<a href="#/example/layout/col-phone-popup" target="_blank">查看</a>|

### 响应式分割设置
```html
<div class="l_col l_col-split-2">
	<div class="l_colContent">
		<div class="l_colItem"> <!-- 第一列 -->
			<!-- 插入组件 -->
		</div>
			<div class="l_colItem"> <!-- 第二列 -->
		<!-- 插入组件 -->
		</div>
	</div>
</div>
```
若需兼容IE8，需加上"<div class="l_colItemContent>" 结构
```html
<div class="l_col l_col-line l_col-full l_col-phone-split-2">
      <div class="l_colContent">
            <div class="l_colItem" style="width:20em;">
                  <div class="l_colItemContent">
                        xxx
                  </div>
            </div>
            <div class="l_colItem">
                  <div class="l_colItemContent">
                        xxx
                  </div>
            </div>
      </div>
</div>
```



注意：l_colItem-phone-float 在手机上默认是隐藏的，需要设置 o.style.display = "block"，才能显示。 