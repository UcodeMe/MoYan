# @Popup 点击穿透的问题

手机客户端一直存在页面点击穿透的问题，尤其是 ``iOS`` 客户端。

解决的最佳办法是，在触发 ``@Popup`` 的时候，给 ``@Popup`` 下面的底部层加上 ``addClass("e_dis")`` ，隐藏层的时候 
加 ``removeClass("e_dis")`` 。

注意需加在底部层的最层结构上。

## 案例

```html
<div class="c_scroll" id="myScroller">
	<div class="padding">
		<button type="button" ontap="showPopup('myPopup','myPopup_item'); $('myScroller').addClass('e_dis')"><span>弹出 Popup</span></button>
	</div>
</div>
<div jwcid="@Popup" name="myPopup">
	<div class="c_popupGroup">
		<div id="myPopup_item" class="c_popupItem">
			<button type="button" ontap="hidePopup(this); $('myScroller').removeClass('e_dis')"><span>隐藏 Popup</span></button>
		</div>
	</div>
</div>
```