# IE8 中状态角标显示不正常

## 解决方法
需要在 ``<div class="c_list">...</div>`` 上加入 ``c_list-hasStatu`` 参数

如：

```
<div class="c_list c_list-hasStatu">
	<ul>
		<li>
			<div class="main">
				<div class="title">标题文字</div>
				<div class="content">描述文字</div>
			</div>
			<div class="statu statu-green">状态</div>
		</li>
	</ul>
</div>
```

