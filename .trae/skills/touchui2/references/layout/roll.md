# 翻转（l_roll）
翻转布局是通过正反两面的形式展示两组相呼应的数据，通过翻转的形式切换。

## HTML

<a href="#/example/layout/roll" target="_blank">查看演示</a>

```html
<div class="l_roll" id="roll">
	<div class="l_rollFront">
		<!-- 正面内容 -->
	</div>
	<div class="l_rollBack">
		<!-- 反面内容 -->
	</div>
</div>
```

注：整体高度是根据正面内容的高度自动伸缩的，因此正面内容最好不要脱离文档流。


## 样式

| 参数名              | 值                  | 默认值 | 描述         |
| ------------------- | ------------------- | ------ | ------------ |
| l_roll-back      | l_roll-back      | 无     | 显示反面       |