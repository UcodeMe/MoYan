# 为什么 @Tabset 的每个 page 都会等高

## 原因
@Tabset 默认的 display 是显示的，只是通过 ``overflow`` 等手段将它们放到了某个 div 之外。

![tabsetSyncHeight1](static/img/doc/tabsetSyncHeight1.png)

因为默认是支持在手机客户端通过滑动来切换的，它滑动切换意味着可能在手指滑动尚未松开之前的这段时间内，两个 page 是同时出现的，并且需要等高的效果，所以在设计之初实际上它们都是处于显示状态的，如果它们不等高，就会出现参差不齐的显示效果，而且手指的滑动范围也会因内容高度的不同而变化，滑动完成之后的高度突然变动也会比较突兀。

![tabsetSyncHeight2](static/img/doc/tabsetSyncHeight2.png)


## 解决

如果希望 @Tabset 的高度均是由其内容决定，只需要设置 slidable 属性为 false 即可，它将变成通过 display 来控制内容的显示/隐藏状态。

```html
<div name="myTab" jwcid="@Tabset" slidable="false">
	...
</div>
```