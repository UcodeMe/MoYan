# 滚动条（c_scroll/@Scroll）
滚动条控件是为了便于通过滚动方式控制显示限制区域内大于此区域的内容。

特别注意：滚动条需插入到拥有高度的对象中，否则请自行设置固定高度，比如 body、div.c_popupItem、div.l_col-full > div.l_colItem、div.l_float

## JWC 方式

<a href="#/example/component/scroll" target="_blank">查看演示</a>

```html
<div name="myScroller" jwcid="@Scroller" vScrollbar="true" bounce="true">
	<!-- 内容 -->
</div>
```

PC only 的项目，推荐使用以下方式，这种方式是纯粹的原生滚动条，无法使用以下各种属性与 API（样式与响应规则均可使用），请使用原生 JS API 控制：

```html
<div class="c_scroll">
	<!-- 内容 -->
</div>
```

## html 方式

html
```html
<div class="c_scroll" style="height:10em;">
	<!-- 内容 -->
</div>
```


## 属性

| 参数名          | 参数类型         | 是否必须 | 默认值 | 描述                                                   |
| --------------- | ---------------- | -------- | ------ | ------------------------------------------------------ |
| name            | java.lang.String | 是       | 无     | 元素 id 和 name 属性的值，也是提交表单数据时的字段名字 |
| class           | java.lang.String | 否       | 无     | 自定义样式名                                           |
| style           | java.lang.String | 否       | 无     | 自定义样式                                             |
| hScroll         | boolean          | 否       | true   | 是否允许横向滚动                                       |
| vScroll         | boolean          | 否       | true   | 是否允许竖向滚动                                       |
| hScrollbar      | boolean          | 否       | true   | 是否显示横向滚动条                                     |
| vScrollbar      | boolean          | 否       | true   | 是否显示竖向滚动条                                     |
| bounce          | boolean          | 否       | true   | 是否使用回弹效果                                       |
| momentum        | boolean          | 否       | true   | 是否使用惯性效果                                       |
| checkDOMChanges | boolean          | 否       | false  | 是否自动检测 DOM 变化                                  |
| snap            | boolean          | 否       | false  | 是否开启 snap 功能                                     |
| scrollContent   | boolean          | 否       | true   | 是否需要生成 content 结构                              |

## JS API 

<a href="#/example/component/scroll-jsApi" target="_blank">查看演示</a>

刷新（内容变化后，手动匹配）

```js
myScroll.refresh();
```

滚动到指定坐标

```js
// scrollTo(x, y, time);
myScroll.scrollTo(0, -100, 200);
```

x：水平方向移动距离，单位为 px

y：垂直方向移动距离，单位为 px

time：移动动画执行的时间，单位为 ms

滚动到指定元素位置

```js
myScroll.scrollToElement(myNode, 100);
```

滚动到指定页面

```js
myScroll.scrollToPage(0, 2, 100);
```

依赖：开启 snap 功能的时候才可以调用这个函数。 					

获取水平方向当前的滚动距离

```js
myScroll.x();
```

获取垂直方向当前的滚动距离

```js
myScroll.y();
```

## 固定高度设置

jwc方式

```html
<div name="myScroller" jwcid="@Scroller" style="height:20em;">
	<!-- 内容 -->
</div>
```

html 方式

```html
<div id="myScroller" class="c_scroll" style="height:20em;">
	<div class="c_scrollContent">
		<!-- 内容 -->
	</div>
</div>
```

## 自适应高度设置

滚动条组件默认高度为 100%，即已实现自适应，而此处描述的 “自适应高度” 则是 （100% - X）的高度设置方法。
如果仅需要设置自适应高度，不需要出现滚动条，请使用 浮动布局（l_float）

jwc方式

```html
<div name="myScroller" jwcid="@Scroller" class="c_scroll c_scroll-float c_scroll-header">
	<!-- 内容 -->
</div>
```

html 方式

```html
<div id="myScroller" class="c_scroll c_scroll-float c_scroll-header">
	<div class="c_scrollContent">
		<!-- 内容 -->
	</div>
</div>
```

## 样式

| 参数名     | 值           |  默认值 | 描述     |演示     |
| ---------- | ------------ | ------ | -------- |--- |
| c_scroll-white |c_scroll-white</br>c_scroll-gray |  无     | 色彩设置 |<a href="#/example/component/scroll-white" target="_blank">查看</a>|
| c_scroll-border |c_scroll-border |  无     | 边框设置 |<a href="#/example/component/scroll-border" target="_blank">查看</a>|
| c_scroll-hover |c_scroll-hover |  无     | 鼠标悬停显示 |<a href="#/example/component/scroll-hover" target="_blank">查看</a>|
| c_scroll-float |c_scroll-float |  无     | 浮动 ||
| c_scroll-wrapper |c_scroll-wrapper |  无     | 四周边距 ||
| c_scroll-header |c_scroll-header |  无     | 留出header区域 ||
| c_scroll-guide |c_scroll-guide |  无     | 留出guide区域 ||
| c_scroll-tab |c_scroll-tab |  无     | 留出tab区域 ||
| c_scroll-tip |c_scroll-float |  无     | 留出提示区域 ||
| c_scroll-search |c_scroll-search |  无     | 留出search区域 ||
| c_scroll-step |c_scroll-step |  无     | 留出step区域 ||
| c_scroll-user |c_scroll-user |  无     | 留出user区域 ||
| c_scroll-param |c_scroll-param |  无     | 留出param区域 ||
| c_scroll-page |c_scroll-page |  无     | 留出page区域 ||
| c_scroll-submit |c_scroll-submit |  无     | 留出submit区域 ||
| c_scroll-grey |c_scroll-grey |  无     | 灰色背景 ||
| c_scroll-noscroll |c_scroll-noscroll |  无     |无滚动条 ||




## 响应规则

| 参数名      | 值|描述     |演示     |
| ---------- | ---|--------- |------ |
| c_scroll-phone |c_scroll-phone| 手机端滚动条设置|<a href="#/example/component/scroll-phone" target="_blank">查看</a>|
| c_scroll-phone-gray |c_scroll-phone-gray（灰色背景）<br />c_scroll-phone-white（白色背景）|手机端的背景颜色 |<a href="#/example/component/scroll-phone-gray" target="_blank">查看</a>|
| c_scroll-phone-noborder |c_scroll-phone-noborder|手机端无边框设置 |<a href="#/example/component/scroll-phone-noborder" target="_blank">查看</a>|
| c_scroll-phone-wrapper-0 |c_scroll-phone-wrapper-0|手机端四周边距设置为 0 |<a href="#/example/component/scroll-phone-wrapper" target="_blank">查看</a>|
| c_scroll-phone-header |c_scroll-phone-header|手机端留出导航区域 |<a href="#/example/component/scroll-phone-header" target="_blank">查看</a>|
| c_scroll-phone-guide |c_scroll-phone-guide|留出 c_guide（向导）组件区域位置 |<a href="#/example/component/scroll-phone-guide" target="_blank">查看</a>|
| c_scroll-phone-tab | c_scroll-phone-tab|适应tab |<a href="#/example/component/scroll-phone-tab" target="_blank">查看</a>|
| c_scroll-phone-user |c_scroll-phone-user|手机端留出user区域 |<a href="#/example/component/scroll-phone-user" target="_blank">查看</a>|
| c_scroll-phone-step |c_scroll-phone-step|手机端留出step区域 |<a href="#/example/component/scroll-phone-step" target="_blank">查看</a>|
| c_scroll-phone-search |c_scroll-phone-search |手机端留出search区域|<a href="#/example/component/scroll-phone-search" target="_blank">查看</a>|
| c_scroll-phone-param |c_scroll-phone-param| 手机端留出留出param区域|<a href="#/example/component/scroll-phone-param" target="_blank">查看</a>|
| c_scroll-phone-page | c_scroll-phone-page|手机端留出留出page区域|<a href="#/example/component/scroll-phone-page" target="_blank">查看</a>|
| c_scroll-phone-submit | c_scroll-phone-submit|手机端留出留出submit区域|<a href="#/example/component/scroll-phone-submit" target="_blank">查看</a>|
| c_scroll-phone-notop |  c_scroll-phone-notop|手机端无顶部区域|<a href="#/example/component/scroll-phone-notop" target="_blank">查看</a>|
| c_scroll-phone-nobottom | c_scroll-phone-nobottom|手机端无按钮区域|<a href="#/example/component/scroll-phone-nobottom" target="_blank">查看</a>|
| c_scroll-phone-noscroll |c_scroll-phone-noscroll|手机端无滚动条|<a href="#/example/component/scroll-phone-noscroll" target="_blank">查看</a>|
| c_scroll-phone-float |c_scroll-phone-float|手机端浮动 |<a href="#/example/component/scroll-phone-float" target="_blank">查看</a>|











