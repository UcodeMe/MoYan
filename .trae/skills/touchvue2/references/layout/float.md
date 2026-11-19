# 自定义悬浮（l_float）
将内容固定显示在自定义的区域，不受到滚动条的影响，主要用于通过 top、left、bottom、right 值来定义一个自适应的区域。 

## HTML

<a href="#/example/layout/float" target="_blank">查看演示</a>

```html
<div class="l_float"></div>
```

## 复合案例

### 留出标题栏

<a href="#/example/layout/float-header" target="_blank">查看演示</a>

```html
<div class="c_header">
	<div class="back">标题栏</div>
</div>
<div class="l_float l_float-header">
	<div class="c_msg c_msg-full">
		<div class="wrapper">
			<div class="emote"></div>
			<div class="info">
				<div class="text">
					<div class="title">留出标题栏</div>
					<div class="content">l_float-header</div>
				</div>
			</div>
		</div>
	</div>
</div>
```

### 留出标题栏 + 搜索框

<a href="#/example/layout/float-headerSearch" target="_blank">查看演示</a>

```html
<div class="c_header">
	<div class="back">标题栏</div>
</div>
<!-- 搜索框 -->
<div class="l_padding l_padding-u c_box c_box-blue">
	<div class="l_col l_col-space">
		<div class="l_colItem">
			<span class="e_mix e_mix-r" style="width:100%;">
				<input type="text">
				<span class="e_ico-search"></span>
			</span>
		</div>
		<div class="l_colItem" style="width:auto;">
			<button type="button" class="e_button-r"><span>高级选项</span><span class="e_ico-next"></span></button>
		</div>
	</div>
</div>
<!-- 搜索框 -->
<div class="l_float l_float-header l_float-search">
	<div class="c_msg c_msg-full">
		<div class="wrapper">
			<div class="emote"></div>
			<div class="info">
				<div class="text">
					<div class="title">留出标题栏 + 搜索框</div>
					<div class="content">l_float-header l_float-search</div>
				</div>
			</div>
		</div>
	</div>
</div>
```

### 留出标题栏 + 客户信息

<a href="#/example/layout/float-headerInfo" target="_blank">查看演示</a>

```html
<div class="c_header">
	<div class="back">标题栏</div>
</div>
<!-- 客户信息 -->
<div class="c_list c_list-blue">
	<ul>
		<li>
			<div class="pic"><span class="e_ico-pic e_ico-user"></span></div>
			<div class="main">
				<div class="title">辛可护</div>
				<div class="content">13510242048</div>
			</div>
			<div class="side"><span class="e_tag e_tag-white e_blue">长沙</span></div>
		</li>
	</ul>
</div>
<!-- 客户信息 -->
<div class="l_float l_float-header l_float-user">
	<div class="c_msg c_msg-full">
		<div class="wrapper">
			<div class="emote"></div>
			<div class="info">
				<div class="text">
					<div class="title">留出标题栏 + 客户信息</div>
					<div class="content">l_float-header l_float-user</div>
				</div>
			</div>
		</div>
	</div>
</div>
```

### 留出标题栏 + 参数列表

<a href="#/example/layout/float-headerParam" target="_blank">查看演示</a>

```html
<div class="c_header">
	<div class="back">标题栏</div>
</div>
<!-- 参数 -->
<div class="c_param c_param-blue c_param-v c_param-avg">
	<ul>
		<li>
			<span class="value">1024</span>
			<span class="label">关注</span>
		</li>
		<li>
			<span class="value">2048</span>
			<span class="label">粉丝</span>
		</li>
		<li>
			<span class="value">4096</span>
			<span class="label">积分</span>
		</li>
	</ul>
</div>
<!-- 参数 -->
<div class="l_float l_float-header l_float-param">
	<div class="c_msg c_msg-full">
		<div class="wrapper">
			<div class="emote"></div>
			<div class="info">
				<div class="text">
					<div class="title">留出标题栏 + 参数列表</div>
					<div class="content">l_float-header l_float-param</div>
				</div>
			</div>
		</div>
	</div>
</div>
```

### 留出标题栏 + 向导

<a href="#/example/layout/float-headerGuide" target="_blank">查看演示</a>

```html
<div class="c_header">
	<div class="back">标题栏</div>
</div>
<!-- 向导 -->
<div class="c_guide">
	<ul>
		<li><span>选择目录</span></li>
		<li><span>流量</span></li>
		<li class="on"><span>国内流量</span></li>
	</ul>
</div>
<!-- 向导 -->
<div class="l_float l_float-header l_float-guid">
	<div class="c_msg c_msg-full">
		<div class="wrapper">
			<div class="emote"></div>
			<div class="info">
				<div class="text">
					<div class="title">留出标题栏 + 向导</div>
					<div class="content">l_float-header l_float-guid</div>
				</div>
			</div>
		</div>
	</div>
</div>
```

### 留出标题栏 + 步骤

<a href="#/example/layout/float-headerStep" target="_blank">查看演示</a>

```html
<div class="c_header">
	<div class="back">标题栏</div>
</div>
<!-- 步骤 -->
<div class="c_step c_step-blue">
	<ul>
		<li class="on">
			<div class="step">1</div>
			<div class="title">第一步</div>
		</li>
		<li>
			<div class="step">2</div>
			<div class="title">第二步</div>
		</li>
		<li>
			<div class="step">3</div>
			<div class="title">第三步</div>
		</li>
	</ul>
</div>
<!-- 步骤 -->
<div class="l_float l_float-header l_float-step">
	<div class="c_msg c_msg-full">
		<div class="wrapper">
			<div class="emote"></div>
			<div class="info">
				<div class="text">
					<div class="title">留出标题栏 + 步骤</div>
					<div class="content">l_float-header l_float-step</div>
				</div>
			</div>
		</div>
	</div>
</div>
```

### 留出标题栏 + 两行表单

<a href="#/example/layout/float-form-towRow" target="_blank">查看演示</a>

```html
<div class="c_header">
	<div class="back">标题栏</div>
</div>
<!-- 两行表单 -->
<div class="c_form c_form-padding c_form-label-4 c_form-col-1">
	<ul>
		<li>
			<div class="label">手机号码：</div>
			<div class="value">
				<input type="text">
			</div>
		</li>
		<li>
			<div class="label">开始时间：</div>
			<div class="value">
				<span class="e_mix">
					<input type="text">
					<span class="e_space"></span>
					<button type="button" class="e_button-blue"><span class="e_ico-search"></span><span>查询</span></button>
				</span>
			</div>
		</li>
	</ul>
</div>
<!-- 两行表单 -->
<div class="c_line"></div>
<div class="l_float" style="top:12em;">
	<div class="c_msg c_msg-full">
		<div class="wrapper">
			<div class="emote"></div>
			<div class="info">
				<div class="text">
					<div class="title">留出标题栏 + 两行表单</div>
					<div class="content">style="top:12em;"</div>
				</div>
			</div>
		</div>
	</div>
</div>
```

### 留出标题栏 + 提交

<a href="#/example/layout/float-submit" target="_blank">查看演示</a>

```html
<div class="c_header">
	<div class="back">标题栏</div>
</div>
<div class="l_float l_float-header l_float-submit">
	<div class="c_msg c_msg-full">
		<div class="wrapper">
			<div class="emote"></div>
			<div class="info">
				<div class="text">
					<div class="title">留出标题栏 + 提交</div>
					<div class="content">l_float-header l_float-submit</div>
				</div>
			</div>
		</div>
	</div>
</div>
<!-- 提交-->
<div class="l_bottom">
	<div class="c_submit c_submit-full">
		<button type="button" class="e_button-l e_button-navy">重置</button>
		<button type="button" class="e_button-l e_button-green">提交</button>
	</div>
</div>
<!-- 提交-->
```

### 留出标题栏 + 分页

<a href="#/example/layout/float-page" target="_blank">查看演示</a>

```html
<div class="c_header">
	<div class="back">标题栏</div>
</div>
<!-- 两行表单 -->
<div class="c_form c_form-padding c_form-label-4 c_form-col-1">
	<ul>
		<li>
			<div class="label">手机号码：</div>
			<div class="value">
				<input type="text">
			</div>
		</li>
		<li>
			<div class="label">开始时间：</div>
			<div class="value">
				<span class="e_mix">
					<input type="text">
					<span class="e_space"></span>
					<button type="button" class="e_button-blue"><span class="e_ico-search"></span><span>查询</span></button>
				</span>
			</div>
		</li>
	</ul>
</div>
<!-- 两行表单 -->
<div class="l_float l_float-header l_float-page">
	<div class="c_msg c_msg-full">
		<div class="wrapper">
			<div class="emote"></div>
			<div class="info">
				<div class="text">
					<div class="title">留出标题栏 + 分页</div>
					<div class="content">l_float-header l_float-page</div>
				</div>
			</div>
		</div>
	</div>
</div>
<div class="l_bottom">
	<div class="c_line"></div>
	<!-- 分页 开始 -->
	<div class="c_page">
		<div class="more"><span class="e_ico-fold"></span></div>
		<div class="turn">
			<div class="first"><span class="e_ico-first"></span></div>
			<div class="prev">上一页</div>
			<div class="page">10/23</div>
			<div class="next">下一页</div>
			<div class="last"><span class="e_ico-last"></span></div>
		</div>
	</div>
	<!-- 分页 结束 -->
</div>
```

### 留出标题栏 + 分页 + 提交

<a href="#/example/layout/float-pageSubmit" target="_blank">查看演示</a>

```html
<div class="c_header">
	<div class="back">标题栏</div>
</div>
<!-- 两行表单 -->
<div class="c_form c_form-padding c_form-label-4 c_form-col-1">
	<ul>
		<li>
			<div class="label">手机号码：</div>
			<div class="value">
				<input type="text">
			</div>
		</li>
		<li>
			<div class="label">开始时间：</div>
			<div class="value">
				<span class="e_mix">
					<input type="text">
					<span class="e_space"></span>
					<button type="button" class="e_button-blue"><span class="e_ico-search"></span><span>查询</span></button>
				</span>
			</div>
		</li>
	</ul>
</div>
<!-- 两行表单 -->
<div class="l_float l_float-header l_float-submit l_float-page">
	<div class="c_msg c_msg-full">
		<div class="wrapper">
			<div class="emote"></div>
			<div class="info">
				<div class="text">
					<div class="title">留出标题栏 + 分页 + 提交</div>
					<div class="content">l_float-header l_float-submit l_float-page</div>
				</div>
			</div>
		</div>
	</div>
</div>
<div class="l_bottom">
	<div class="c_line"></div>
	<!-- 分页 开始 -->
	<div class="c_page">
		<div class="more"><span class="e_ico-fold"></span></div>
		<div class="turn">
			<div class="first"><span class="e_ico-first"></span></div>
			<div class="prev">上一页</div>
			<div class="page">10/23</div>
			<div class="next">下一页</div>
			<div class="last"><span class="e_ico-last"></span></div>
		</div>
	</div>
	<!-- 分页 结束 -->
	<!-- 提交 开始 -->
	<div class="c_submit c_submit-full">
		<button type="button" class="e_button-l e_button-navy">重置</button>
		<button type="button" class="e_button-l e_button-green">提交</button>
	</div>
	<!-- 提交 结束 -->
</div>
```

### 留出四周

<a href="#/example/layout/float-wrapper" target="_blank">查看演示</a>

```html
<div class="l_float l_float-wrapper" style="background:#8fc320;">
	<div>
		<br />
		<br />
		<br />
		<span class="e_white">
			假装很多内容。。。
		</span>
		<br />
		<br />
		<br />
	</div>
</div>
```

### 响应规则-留出四周

<a href="#/example/layout/float-phone-wrapper" target="_blank">查看演示</a>

```html
<div class="l_float l_float-wrapper l_float-phone-wrapper-0" style="background:#8fc320;">
	<div>
		<br />
		<br />
		<br />
		<span class="e_white">
			请切换至手机模式查看效果
		</span>
		<br />
		<br />
		<br />
	</div>
</div>
```

## 样式

| 参数名         | 值                                                           | 默认值 | 描述           |
| -------------- | ------------------------------------------------------------ | ------ | -------------- |
| l_float-header | l_float-header<br />l_float-guide<br />l_float-tab<br />l_float-param<br />l_float-page<br />l_float-submit<br />l_float-wrapper（四周预留空间 10px ）<br />l_float-result（查询结果 10px ） | 无     | 自适应高度设置 |
| l_float-white  | l_float-white                                                | 无     | 背景色设置     |
| l_float-border | l_float-border                                               | 无     | 边框设置       |

## 响应规则

| 参数名        | 值  | 描述 |演示|
| --------|------ | ---- |---|
| l_float-phone-wrapper |l_float-phone-wrapper| 手机端留出四周边距 |<a href="#/example/layout/float-phone-wrapper" target="_blank">查看</a>    |
| l_float-phone-header |l_float-phone-header|手机端留出header区域 |<a href="#/example/layout/float-phone-header" target="_blank">查看</a> |
| l_float-phone-guide |   l_float-phone-guide| 手机端留出guide区域   |<a href="#/example/layout/float-phone-guide" target="_blank">查看</a>|
| l_float-phone-tab | l_float-phone-tab| 手机端留出tab区域    |<a href="#/example/layout/float-phone-tab" target="_blank">查看</a>|
| l_float-phone-param |  l_float-phone-param | 手机端留出param区域   |<a href="#/example/layout/float-phone-param" target="_blank">查看</a>|
| l_float-phone-page | l_float-phone-page | 手机端留出page区域    |<a href="#/example/layout/float-phone-page" target="_blank">查看</a>|
| l_float-phone-submit |l_float-phone-submit | 手机端留出submit区域     |<a href="#/example/layout/float-phone-submit" target="_blank">查看</a>|
| l_float-phone-white |  l_float-phone-white|手机端白色背景     |<a href="#/example/layout/float-phone-white" target="_blank">查看</a>|
| l_float-phone-gray | l_float-phone-gray|   手机端灰色背景   |<a href="#/example/layout/float-phone-gray" target="_blank">查看</a>|
| l_float-phone-headertab | l_float-phone-headertab| 手机端留出header和tab区域    |<a href="#/example/layout/float-phone-headertab" target="_blank">查看</a>|
| l_float-phone-notop |l_float-phone-notop|手机端无上边距     |<a href="#/example/layout/float-phone-notop" target="_blank">查看</a>|
| l_float-phone-nobottom |l_float-phone-nobottom|  手机端无下边距    |<a href="#/example/layout/float-phone-nobottom" target="_blank">查看</a>|
| l_float-phone-noscroll | l_float-phone-noscroll|  手机端无滚动条    |<a href="#/example/layout/float-phone-noscroll" target="_blank">查看</a>|
| l_float-phone-noborder |l_float-phone-noborder |  手机端无边框    |<a href="#/example/layout/float-phone-noborder" target="_blank">查看</a>|


