# 置顶/底边悬浮（l_top/l_bottom/l_fixed）
将内容固定显示在一定的区域，不受到滚动条的影响。 

## HTML

<a href="#/example/layout/absolute" target="_blank">查看演示</a>


顶部悬浮

<a href="#/example/layout/absolute-top" target="_blank">查看演示</a>

```html
<div class="l_top"><!-- 插入组件 --></div>
```

底部悬浮 

<a href="#/example/layout/absolute-bottom" target="_blank">查看演示</a>

```html
<div class="l_bottom"><!-- 插入组件 --></div>
```

固定悬浮 

<a href="#/example/layout/absolute-fixed" target="_blank">查看演示</a>

```html
<div class="l_fixed"><!-- 插入组件 --></div>
```

## 响应规则

| 参数名        |值       | 描述 |演示 |
| ----------|-------- | ---- |--- |
| l_fixed-phone |l_fixed-phone |仅手机端固定悬浮 |<a href="#/example/layout/absolute-fixed-phone" target="_blank">查看</a>|
| l_fixed-phone-slide |l_fixed-phone-slide |手机端固定悬浮并可侧滑显示（侧滑显示时需加入 l_fixed-show 样式） |<a href="#/example/layout/absolute-fixed-phone-slide" target="_blank">查看</a>|

