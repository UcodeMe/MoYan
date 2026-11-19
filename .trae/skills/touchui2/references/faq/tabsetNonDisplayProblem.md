# @Tabset 在 @Dialog 中不显示问题

## 原因
由于 @Tabset 的高宽是依靠，外层组件的高宽来计算的，若外层组件初始状态是隐藏状态 ``display:none`` ，@Tabset 就会计算没有高度，造成 @Tabset 的不显示。如 @popup 组件、@dialog 组件。
此时需要在弹框触发时加上 ``myTab.adjust()`` ；就可以解决这个问题。


## 案例

```html
<button type="button" ontap="showPopup('myDialog'); myTab.adjust();"><span>弹出带有 @Tabset 的 @Dialog</span></button>
<div jwcid="@Dialog" name="myDialog">
	<div name="myTab" jwcid="@Tabset">
		<div title="Tab Label A" class="content">
			<!-- Tab Content A -->
		</div>
		<div title="Tab Label B" class="content">
			<!-- Tab Content B -->
		</div>
	</div>
</div>
```
	