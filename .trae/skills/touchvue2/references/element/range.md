# 范围（e_range / @Range）
范围是扩展的表单元素之一，主要用于通过拖曳的方式选取一定的取值范围，给提交的数据提供来源。

## JWC 方式
<a href="#/example/element/range" target="_blank">查看演示</a>
```html
<span name="myRange" jwcid="@Range" min="10" max="100"></span>
```

## JS 方式

html
```html
<span class="e_range">
	<span class="e_rangeMain">
		<span class="e_rangeBar">
			<span class="e_rangeProgress">
				<span class="e_rangeEnd"></span>
				<input type="hidden" name="myRange" id="myRange" />
			</span>
			<span class="e_rangeRuler">
				<span class="e_rangeScale">
					<span class="scaleValue">0</span>
				</span>
				<span class="e_rangeScale">
					<span class="scaleValue">20</span>
				</span>
				<span class="e_rangeScale">
					<span class="scaleValue">40</span>
				</span>
				<span class="e_rangeScale">
					<span class="scaleValue">60</span>
				</span>
				<span class="e_rangeScale">
					<span class="scaleValue">80</span>
				</span>
				<span class="e_rangeScale e_rangeScale-last">
					<span class="scaleValue">100</span>
				</span>
			</span>
		</span>
		<span class="e_rangeValue"></span>
	</span>
</span>
```
js

```js
window["myRange"] = new Wade.Range("myRange",{
	min  : 0,
	max  : 100,
	step : 20
});
```

## 属性

| 参数名 | 参数类型         | 是否必须 | 默认值 | 描述                                 |演示     |
| ------ | ---------------- | -------- | ------ | ------------------------------------ |------ | 
| name   | java.lang.String | 是       | 无     | 组件实例名，同一个页面里不能重复     ||
| min    | int              | 否       | 0      | 范围最小值（起始值），默认为 0       |<a href="#/example/element/range-min" target="_blank">查看</a>|
| max    | int              | 否       | 100    | 范围最大值（结束值），默认为 100     |<a href="#/example/element/range-min" target="_blank">查看</a>|
| step   | int              | 否       | 10     | 拖动时的步长, 从开始到结束移动多少步 |<a href="#/example/element/range-step" target="_blank">查看</a>|
| value  | int              | 否       | 无     | 当前值                               |<a href="" target="_blank">查看</a>|
| class  | java.lang.String | 否       | 无     | 自定义样式表                         ||
| style  | java.lang.String | 否       | 无     | 自定义样式                           ||
| start  | boolean          | 否       | false  | 设置双节点             |<a href="#/example/element/range-start" target="_blank">查看</a>|

## 样式
| 参数名     | 值           |  默认值 | 描述     |演示     |
| ---------- | ------------ | ------ | -------- |-------- |
| e_range-value-s |e_range-value-s<br />e_range-value-m<br />e_range-value-l | e_range-value-l | 提示文字宽度设置 |<a href="#/example/element/range-value" target="_blank">查看</a>|
| e_range-gd |e_range-gd | 无 |渐变|<a href="#/example/element/range-gd" target="_blank">查看</a>|