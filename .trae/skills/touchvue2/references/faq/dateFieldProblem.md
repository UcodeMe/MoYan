# @DateField 使用中出现样式结构问题

``@DateField`` 在与其他组件组合使用的过程中， ``@DateField`` 会生成 ``e_mix`` / ``e_group``，不要把 ``@DateField`` 插入到 ``e_mix`` 中，因为在 ``@DateField`` 组件会生成 ``e_mix`` ，如在 ``e_mix`` 中加入 ``@DateField`` 组件就会出现问题。

## 错误示范

```html
<div name="myScroller" jwcid="@Scroller" vScrollbar="true" bounce="true">
	<div class="padding">
		<div class="c_list c_list-line c_list-border">
			<ul>
				<li>
					<div class="label">提示文字</div>
					<div class="value">
						<span class="e_group">
							<input name="myDateField" 
								jwcid="@DateField" 
								dropDown="true" 
								onafterAction="verifyAll('cond')" 
								value="1988-03-09 11:13:14" 
								format="yyyy-MM-dd HH:mm:ss" />
						</span>
					</div>
				</li>
			</ul>
		</div>
	</div>
</div>
```

## 正确示范

```html
<div name="myScroller" jwcid="@Scroller" vScrollbar="true" bounce="true">
	<div class="padding">
		<div class="c_list c_list-line c_list-border">
			<ul>
				<li>
					<div class="label">提示文字</div>
					<div class="value">
						<input name="myDateField" 
							jwcid="@DateField" 
							dropDown="true" 
							onafterAction="verifyAll('cond')" 
							value="1988-03-09 11:13:14" 
							format="yyyy-MM-dd HH:mm:ss" />
					</div>
				</li>
			</ul>
		</div>
	</div>
</div>
```