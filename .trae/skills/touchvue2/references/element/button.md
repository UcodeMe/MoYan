---
trigger: model_decision
description: 按钮元素，某种控制功效的可视可交互的触发载体
---

# 按钮（e_button）
某种控制功效的可视可交互的触发载体。

## HTML
<a href="#/example/element/button" target="_blank">查看演示</a>
```html
<button type="button"><span>按钮</span></button>
<a href="#nogo" class="e_button"><span>按钮</span></a>
```

## 样式

| 参数名     | 值           |  默认值 | 描述     |演示     |
| ---------- | ------------ | ------ | -------- |-------- |
| e_button-blue |e_button-blue<br />e_button-green<br />e_button-orange<br />e_button-red<br />e_button-navy |  无     | 色彩设置 |<a href="#/example/element/button-color" target="_blank">查看</a> |
| e_button-gd-greenBlue |e_button-gd-greenBlue<br />e_button-gd-blueRed<br />e_button-gd-redBlue<br />e_button-gd-redOrange<br />e_button-gd-orangeRed<br />e_button-gd-green<br />e_button-gd-red<br />e_button-gd-blue<br />e_button-gd-orange<br />e_button-gd-navy |  无     | 渐变设置 |<a href="#/example/element/button-gd" target="_blank">查看</a> |
| e_button-s |e_button-s<br />e_button-m<br />e_button-l | e_button-m | 大小设置 ||
| e_button-r |e_button-left<br />e_button-right<br />e_button-r |  无     | 圆角设置 |<a href="#/example/element/button-r" target="_blank">查看</a> |
| e_button-outline |e_button-outline |  无     | 外框设置 |<a href="#/example/element/button-outline" target="_blank">查看</a> |
| e_button-width-s |e_button-width-s<br />e_button-width-m<br />e_button-width-l |  无     | 固定宽度设置 |<a href="#/example/element/button-width" target="_blank">查看</a> |

## 插件

### 插入图标

```html
<button type="button"><span class="e_ico-edit"></span><span>编辑</span></button>
```



### 插入折叠/展开标识 (支持：fold、unfold、next )

```html
<button type="button"><span>发布</span><span class="e_ico-unfold"></span></button>
```



### 插入进度条 

```html
<button type="button">
	<span>加载中...</span>
	<span class="e_buttonProgress">
		<span class="e_buttonBar" style="width:39%;"></span>
	</span>
</button>
```

