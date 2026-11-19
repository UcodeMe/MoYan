# 卡片（c_card）


放置信息集合的载体，便于模块化操作。 

## HTML
<a href="#/example/component/card" target="_blank">查看演示</a>

### 一般模式

```html
<div class="c_card">
  <div class="pic">
   <img src="../../../temp/photo/1.jpg" alt="">
   </div>
  <div class="main">Tony Stark</div>
</div>
```

### 垂直模式

<a href="#/example/component/card-v" target="_blank">查看演示</a>

```html
<div class="c_card c_card-v c_box">
	<div class="pic">
		<img src="../../../temp/photo/4.jpg" alt="" />
			</div>
				<div class="main">
					<div class="title">Natasha Romanoff</div>
						 <div class="content">
								1928年出生于前苏联的斯大林格勒，自幼被前苏联特工人员训练成特工，身体经前苏联政府基因改造后大大延缓了其衰老速度，并增强其免疫系统以及抗击打能力，加上本身多年的各种体能及精神上的训练，自身已经犹如致命的毒蜘蛛“黑寡妇”，后背叛前苏联特工组织“红房”，加入复仇者联盟，其年龄仅比美国队长小8岁。
		</div>
	</div>
</div>
```

### 横向排列

<a href="#/example/component/card-Horizontal" target="_blank">查看演示</a>

```html
<div class="l_grid l_grid-space l_grid-col-3">
	<ul>
		<li>
			<div class="c_card c_box">
				<div class="main">Tony Stark2</div>
				<div class="side">钢铁侠</div>
			</div>
		</li>
		<li>
			<div class="c_card c_box">
				<div class="main">Tony Stark2</div>
				<div class="side">钢铁侠</div>
			</div>
		</li>
		<li>
			<div class="c_card c_box">
				<div class="main">Tony Stark2</div>
				<div class="side">钢铁侠</div>
			</div>
		</li>
	</ul>
</div>
```

### 竖向排列

<a href="#/example/component/card-Vertical" target="_blank">查看演示</a>

```html
<div class="l_grid l_grid-space l_grid-col-2">
	<ul>
		<li>
			<div class="c_box l_padding-2">
				<div class="c_card c_box">
					<div class="main">Tony Stark2</div>
					<div class="side">钢铁侠</div>
				</div>
				<div class="c_card c_box">
					<div class="main">Tony Stark2</div>
					<div class="side">钢铁侠</div>
				</div>
				<div class="c_card c_box">
					<div class="main">Tony Stark2</div>
					<div class="side">钢铁侠</div>
				</div>
			</div>
		</li>
	</ul>
</div>
```

### 插入边列

<a href="#/example/component/card-side" target="_blank">查看演示</a>

```html
<div class="c_card c_box">
	<div class="main">Tony Stark2</div>
	<div class="side">钢铁侠</div>
</div>
```

### 内部分组设置

<a href="#/example/component/card-group" target="_blank">查看演示</a>

```html
<div class="c_card c_box">
		<div class="group">
			<div class="pic">
				<span class="e_ico-asir e_ico-pic-white"></span>
					</div>
						<div class="main">
							<div class="title">Steve Rogers</div>
								<div class="content">
                                    1928年出生于前苏联的斯大林格勒，自幼被前苏联特工人员训练成特工，身体经前苏联政府基因改造后大大延缓了其衰老速度，并增强其免疫系统以及抗击打能力，加上本身多年的各种体能及精神上的训练，自身已经犹如致命的毒蜘蛛“黑寡妇”，后背叛前苏联特工组织“红房”，加入复仇者联盟，其年龄仅比美国队长小8岁。
                     </div>
				</div>
			<div class="side">美国队长</div>
		<div class="more"></div>
	</div>
</div>
```

### 插入功能区

<a href="#/example/component/card" target="_blank">查看演示</a>

```html
<div class="c_card c_box">
	<div class="main">Natasha Romanoff</div>
		<label class="fn">
		<span class="e_ico-close"></span>
	</label>
</div>
```

### 插入按钮

<a href="#/example/component/card-Button" target="_blank">查看演示</a>

```html
<div class="c_card c_box">
	<div class="main">Steve Rogers</div>
		<div class="button">
			<button type="button">
			   <span class="e_ico-config"></span>
				 <span>配置</span>
				 </button>
				</div>
		       </div><div class="c_card c_box">
	          <div class="main">Natasha Romanoff</div>
	      <label class="fn">
		<span class="e_ico-close"></span>
	</label>
</div>
```

### 插入图标

<a href="#/example/component/card-ico" target="_blank">查看演示</a>

```html
<div class="c_card">
	<div class="pic pic-row-0 pic-close">
		<span class="e_ico-user"></span>
	</div>
    <div class="main">Bruce Banner</div>
</div>
```



## 样式

| 参数名   | 值       | 默认值 | 描述         | 演示                                                         |
| -------- | -------- | ------ | ------------ | ------------------------------------------------------------ |
| c_card-v | c_card-v | 无     | 垂直排列模式 | <a href="#/example/component/card-v" target="_blank">查看</a> |

### &lt;div class="content"&gt;

| 参数名        | 值                                                           | 默认值        | 描述             | 演示                                                         |
| ------------- | ------------------------------------------------------------ | ------------- | ---------------- | ------------------------------------------------------------ |
| content-row-3 | content-row-1 ～ content-row-4<br />content-auto（不限高度） | content-row-1 | 内容高度限制设置 | <a href="#/example/component/card-contentRow" target="_blank">查看</a> |

### &lt;div class="pic"&gt;

| 参数名         | 值                              | 默认值       | 描述             | 演示                                                         |
| -------------- | ------------------------------- | ------------ | ---------------- | ------------------------------------------------------------ |
| pic-close      | pic-close                       | pic-close    | 取消图片边距     | <a href="#/example/component/card-picClose" target="_blank">查看</a> |
| pic-row-3      | pic-row-0 ～ pic-row-5          | pic-row-1    | 图片高度限制设置 | <a href="#/example/component/card-picRow" target="_blank">查看</a> |
| pic e_size-xxl | pic e_size-x～ pic e_size-xxxxl | pic e_size-x | 图片大小层级设置 | <a href="#/example/component/card-picSize" target="_blank">查看</a> |

### &lt;div class="title"&gt;

| 参数名      | 值                         | 默认值      | 描述             | 演示                                                         |
| ----------- | -------------------------- | ----------- | ---------------- | ------------------------------------------------------------ |
| title-row-3 | title-row-1 ～ title-row-5 | title-row-1 | 标题高度限制设置 | <a href="#/example/component/card-titleRow" target="_blank">查看</a> |

### &lt;div class="side"&gt;

| 参数名    | 值        | 默认值    | 描述           | 演示                                                         |
| --------- | --------- | --------- | -------------- | ------------------------------------------------------------ |
| side-line | side-line | side-line | 右边区域分界线 | <a href="#/example/component/card-sideLine" target="_blank">查看</a> |

### &lt;div class="l_grid"&gt;

| 参数名       | 值                         | 默认值       | 描述           | 演示                                                         |
| ------------ | -------------------------- | ------------ | -------------- | ------------------------------------------------------------ |
| l_grid-space | l_grid-space               | l_grid-space | 卡片间距离设置 | <a href="#/example/component/card-gridSpace" target="_blank">查看</a> |
| l_grid-col-3 | l_grid-col-1 ～ pic-row-10 | l_grid-col-1 | 卡片列数设置   | <a href="#/example/component/card-gridCol" target="_blank">查看</a> |

