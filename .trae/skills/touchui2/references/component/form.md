# 表单（c_form）
用于布局各种表单元素，以达到采集数据的目的。

## HTML
<a href="#/example/component/form" target="_blank">查看演示</a>

```html
<div class="c_form">
	<ul>
		<li>
			<div class="label">提示文字：</div>
			<div class="value">任何元素</div>
		</li>
	</ul>
</div>
```

## 插件

### 插入复选框
<a href="#/example/component/form-checkbox" target="_blank">查看演示</a>

```html
<div class="c_form">
	<ul>
		<li>
			<div class="label">必备技能：</div>
			<div class="value">
				<div class="c_list c_list-s">
					<ul>
						<li>
							<label class="content link">
								<div class="fn"><input type="checkbox" /></div>
								<div class="main">
									<div class="title">选项内容</div>
								</div>
							</label>
						</li>
					</ul>
				</div>
			</div>
		</li>
	</ul>
</div>
```

### 插入按钮
<a href="#/example/component/form-submit" target="_blank">查看演示</a>

```html
<div class="c_form c_form-submit">
	<ul>
		<li>
			<div class="label">提示文字：</div>
			<div class="value"><input type="text" /></div>
		</li>
	</ul>

	<div class="submit">
		<button type="button" class="e_button-blue">查询</button>
	</div>

</div>
```

### 插入横向按钮
<a href="#/example/component/form-submit-h" target="_blank">查看演示</a>

```html
<div class="c_form c_form-submit-h">

	<div class="submit">
		<button type="button" class="e_button-blue">查询</button>
	</div>

	<ul>
		<li>
			<div class="label">提示文字：</div>
			<div class="value"><input type="text" /></div>
		</li>
	</ul>
</div>
```

## 样式

| 参数名| 值 | 默认值  | 描述  |  演示|
----------------------- |------------------------------- | -------------- | ---------------- |------- |
| c_form-label-2          | c_form-label-2 ~ c_form-label-20 | c_form-label-4 | 设置提示文字长度 |<a href="#/example/component/form-label" target="_blank">查看</a>|
| c_form-col-2            | c_form-col-2                     | 无             | 设置分列         |<a href="#/example/component/form-col" target="_blank">查看</a>|
| c_form-box              | c_form-box                       | 无             | 设置边框         |<a href="#/example/component/form-box" target="_blank">查看</a>|
| c_form-padding          | c_form-padding                   | 无             | 设置边距         |<a href="#/example/component/form-padding" target="_blank">查看</a>|
| c_form-v                | c_form-v                         | 无             | 设置垂直排列     |<a href="#/example/component/form-v" target="_blank">查看</a>|
| c_form-auto             | c_form-auto                      | 无             | 设置自动宽度     |<a href="#/example/component/form-auto" target="_blank">查看</a>|
| c_form-submit-nopadding | c_form-submit-nopadding          | 无             | 取消边距         ||

## 响应规则

| 参数名                |值| 描述 |演示 |
| ------------------- | ---|---- |--- |
| c_form-phone-col-1 |c_form-phone-col-1<br />c_form-phone-col-2<br />c_form-phone-col-3<br />  |手机端分列列数设置 |<a href="#/example/component/form-phone-col" target="_blank">查看</a>|
| c_form-phone-v     |c_form-phone-v| 手机端布局垂直排列 |<a href="#/example/component/form-phone-v" target="_blank">查看</a>|
| c_form-submit-phone-v   |c_form-submit-phone-v| 手机端提交区域垂直排列 |<a href="#/example/component/form-submit-phone-v" target="_blank">查看</a>|

