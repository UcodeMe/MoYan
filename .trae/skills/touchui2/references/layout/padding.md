# 内边距（l_padding）

指定对象内部元素与其之间的距离。 

## HTML

<a href="#/example/layout/padding" target="_blank">查看演示</a>

```html
<div class="l_padding">
	<!-- 插入组件 -->
</div>
```

## 样式

| 参数名      | 值                                                           | 默认值      | 描述         |演示         |
| ----------- | ------------------------------------------------------------ | ----------- | ------------ |----- |
| l_padding-4 | l_padding-1<br />l_padding-2<br />l_padding-3<br />l_padding-4<br /> | l_padding-1 | 间距大小设置 |<a href="#/example/layout/padding-size" target="_blank">查看</a>|
| l_padding-side | l_padding-side<br />l_padding-u<br />l_padding-n | | 间距位置 |<a href="#/example/layout/padding-posi" target="_blank">查看</a>|

## 响应规则

| 参数名|值| 描述 |演示 |
| -----------|---------- | ---- |-- |
| l_padding-phone-0 |l_padding-phone-0 <br />l_padding-phone <br />l_padding-phone-1<br />l_padding-phone-2 <br />l_padding-phone-3 <br />l_padding-phone-4 <br />|   手机端大小设置   |<a href="#/example/layout/padding-phone-size" target="_blank">查看</a>|
| l_padding-phone-side | l_padding-phone-side| 手机端无上下边距   |<a href="#/example/layout/padding-phone-posi" target="_blank">查看</a>|
| l_padding-phone-u    | l_padding-phone-u|  手机端无上边距   |<a href="#/example/layout/padding-phone-posi" target="_blank">查看</a>|
| l_padding-phone-n    |  l_padding-phone-n|  手机端无下边距   |<a href="#/example/layout/padding-phone-posi" target="_blank">查看</a>|

注意：c_param-phone-h 是针对 c_param-v 而言的，如果 PC 端设置了 c_param-v，则可以通过 c_param-phone-h 重置回来。 