# 进度条（e_progress / @Progress）
以图片形式显示处理任务的速度，完成度，剩余未完成任务量的大小。

## JWC 方式
<a href="#/example/element/progress" target="_blank">查看演示</a>
```html
<span name="myProgress" percent="0.33" remainText="5/15" jwcid="@Progress"></span>
```

## JS 方式

html

```html
<span class="e_progress">
	<span class="e_progressBar">
		<span style="width:33.33%" class="e_progressProgress">33.33%</span>
	</span>
	<span class="e_progressValue">5/15</span>
	<input type="hidden" name="myProgress" id="myProgress" />
</span>
```

js
```js
window["myProgress"] = new Wade.Progress(
	// 对应元素，el 可以为元素 id，或者原生 dom 对象
	"myProgress",
	// 参数设置
	{
		disabled:false,
		percent:0
	}
);
```

## 属性


| 参数名       | 参数类型         | 是否必须 | 默认值  | 描述                                   |
| ------------ | ---------------- | -------- | ------- | ----------------------------------- |
| name         | java.lang.String | 是       | 无      | 组件实例名，同一个页面里不能重复       |
| percent      | float            | 否       | 0       | 进度条的百分比，必须是 >=0，<=1 的数值 |
| completeText | java.lang.String | 否       | 无      | 完成状态提示信息文本                  |
| remainText   | java.lang.String | 否       | 无      | 剩余状态提示信息文本                  |
| class        | java.lang.String | 否       | 无      | 自定义样式名                         |
| style        | java.lang.String | 否       | 无      | 自定义样式                           |
| disabled     | boolean          | 否       | false   | 是否禁用                             |
| color        | java.lang.String | 否       | "green" | 进度条颜色                           |

## JS API

更新进度条状态

```js
// 实例名.update(percent,remainText);
myProgress.update(0.33, '5/15');
```

注意：0 ≤ percent ≤ 1 	

## 样式
| 参数名     | 值           |  默认值 | 描述     |演示     |
| ---------- | ------------ | ------ | -------- |------ |
| e_progress-loading |e_progress-loading |  无     | 载入模式 |<a href="#/example/element/progress-loading" target="_blank">查看</a>|
| e_progress-value-s |e_progress-value-s<br />e_progress-value-m<br />e_progress-value-l<br />e_progress-value-none | e_progress-value-l | 提示文字宽度设置 |<a href="#/example/element/progress-value" target="_blank">查看</a>|
| e_progress-s |e_progress-m<br />e_progress-s<br />e_progress-xs  | e_progress-m | 大小设置 |<a href="#/example/element/progress-s" target="_blank">查看</a>|
| e_progress-blue |e_progress-blue<br />e_progress-red<br />e_progress-green<br />e_progress-orange |  e_progress-blue | 圆角设置 |<a href="#/example/element/progress-color" target="_blank">查看</a>|
| e_progress-r |e_progress-r |  无     | 圆角设置 |<a href="#/example/element/progress-r" target="_blank">查看</a>|
| e_progress-gd |e_progress-gd-greenBlue<br />e_progress-gd-redBlue<br />e_progress-gd-blueRed<br />e_progress-gd-orangeRed<br />e_progress-gd-redOrange |  无     | 渐变 |<a href="#/example/element/progress-gd" target="_blank">查看</a>|