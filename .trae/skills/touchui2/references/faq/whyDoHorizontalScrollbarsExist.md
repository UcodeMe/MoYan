# 为什么会有横向滚动条?

可能存在的场景 ``@Popup`` 、 ``@Dialog``

## 原因
``@Popup`` 组件、 ``@Dialog`` 组件写在了滚动条 ``@Scroller`` 之中，导致问题出现。正确的做法是需要放在 ``@Scroll`` 组件之外

## 错误示范

```html
<div name="myScroller" jwcid="@Scroller" vScrollbar="true" bounce="true">
	<div class="padding">
		<!-- 内容 -->
	</div>

	<div jwcid="@Dialog" 
		name="myDialog" 
		title="标题" 
		width="20"
		closeable="true">
		<!-- 内容开始 -->
		<div class="c_msg c_msg-border"> 
			<div class="wrapper">
				<div class="emote"></div>
				<div class="info">
					<div class="text">
						<div class="title">弹出框</div>
					</div>
				</div>
			</div>
		</div>
		<!-- 内容结束 -->
	</div>
</div>
```

## 正确示范

```html
<div name="myScroller" jwcid="@Scroller" vScrollbar="true" bounce="true">
	<div class="padding">
		<!-- 内容 -->
	</div>
</div>

<div jwcid="@Dialog" 
	name="myDialog" 
	title="标题" 
	width="20"
	closeable="true">
	<!-- 内容开始 -->
	<div class="c_msg c_msg-border">
		<div class="wrapper">
			<div class="emote"></div>
			<div class="info">
				<div class="text">
					<div class="title">弹出框</div>
				</div>
			</div>
		</div>
	</div>
	<!-- 内容结束 -->
</div>
```

但是有同学要问了，如果我要封装一个组件，组件本身需弹出一个 ``@Popup`` 或 ``@Dialog``，按照以上示范，组件的一部分要放到滚动条里面，而另一部分需要放到外面，这就很为难了。

此时需要用到<a href="#/doc/logical/bodycontent">@BodyContent</a>，它具备控制代码生成位置的能力，把它放到 ``@BodyContent`` 中，即可将 ``@Popup`` 或 ``@Dialog`` 自动生成在滚动条之外。

需要注意的是，这只支持服务端 JWC 组件的封装，而客户端，嗯…自己 append 到 body 下吧…