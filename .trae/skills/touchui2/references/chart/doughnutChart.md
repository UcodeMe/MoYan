# 环形图（@DoughnutChart）
由两个及两个以上大小不一的饼图叠在一起，挖去中间的部分所构成的图形。
图表组件均封装自echarts

## HTML

<a href="#/example/chart/doughnutChart" target="_blank">查看演示</a>

```html
<div name="chart" id="chart5" style="height:280px; width:500px;"></div>
```

### 比例

<a href="#/example/chart/doughnutChart-proportion" target="_blank">查看演示</a>

```html
<div name="chart" id="chart6" style="height:280px; width:500px;"></div>
```

```js
var chart = echarts.init(document.getElementById("chart6"));
var option = {
	title: {
		text:"1134",
		subtext:"总重量",
		left:"center",
		top:"center",
		textStyle: {
			color:"#000",
			fontSize: 50,
			fontWeight:"lighter"
		},
		subtextStyle: {
			color:"#666",
			fontSize: 14,
			verticalAlign: "top"
		},
		itemGap:-10
	},
	legend: {
		data: [
			"玉米", 
			"水份", 
			"菠菜", 
			"荞麦", 
			"冰块", 
			"紫薯", 
			"南瓜", 
			"红糖"
		],
		left:0,
		padding:0,
		orient: "vertical",
		icon:"circle"
	},
	tooltip: {
		trigger: "item", 
		formatter: "{b}：{c}g<br />（{d}%）"
	},
	series: [
		{
			avoidLabelOverlap: false, 
			color:["#5dabe6","#92e5a9","#ffdd02","#e0d5aa","#8dd4d4","#9485b0","#fec652","#fd7a7a"],
			label: {
				normal: {
					show: false
				}
			},
			radius: [
				"80%",
				"100%"
			],
			itemStyle:{
				borderColor:"#fff",
				borderWidth:2,
				borderType:"solid"
			},
			type: "pie",
			data: [
				{
					name: "玉米", 
					value: 200
				},
				{
					name: "水份", 
					value: 60
				},
				{
					name: "菠菜", 
					value: 90
				},
				{
					name: "荞麦", 
					value: 140
				},
				{
					name: "冰块", 
					value: 90
				},
				{
					name: "紫薯", 
					value: 40
				},
				{
					name: "南瓜", 
					value: 5
				},
				{
					name: "红糖", 
					value: 72
				}
			]
		}
	],
	animation: false
};
if( option && option.series ){
	chart.setOption(option);
}
chart = null;
```
### 迷你

<a href="#/example/chart/doughnutChart-mini" target="_blank">查看演示</a>

```html
<div class="l_grid l_grid-col-3 l_grid-space">
	<ul>
		<li>
			<div class="c_box l_padding-2">
				<div name="chart" id="chartBlue" style="height:100px; width:100px; margin:0 auto;"></div>
			</div>
		</li>
		<li>
			<div class="c_box l_padding-2">
				<div name="chart" id="chartYellow" style="height:100px; width:100px; margin:0 auto;"></div>
			</div>
		</li>
		<li>
			<div class="c_box l_padding-2">
				<div name="chart" id="chartGreen" style="height:100px; width:100px; margin:0 auto;"></div>
			</div>
		</li>
	</ul>
</div>
```

```js
//蓝色圈圈
var chart = echarts.init(document.getElementById("chartBlue"));
var option = {
	title: {
		text: "14.7G",
		left: "center",
		top: "center",
		textStyle: {
			color: "#000",
			fontSize: 20,
			fontWeight: "lighter"
		}
	},
	tooltip: {
		trigger: "item",
		formatter: "{b}：{c}G<br />（{d}%）"
	},
	series: [{
		avoidLabelOverlap: false,
		label: {
			normal: {
				show: false
			}
		},
		radius: [
			"90%",
			"100%"
		],
		type: "pie",
		data: [{
				name: "剩余",
				value: 14.7,
				itemStyle: {
					color: "#5dabe6"
				}
			},
			{
				name: "已用",
				value: 5.3,
				itemStyle: {
					color: "#e6e6e6"
				}
			}
		]
	}],
	animation: false
};
if (option && option.series) {
	chart.setOption(option);
}
//黄色圈圈
var chart = echarts.init(document.getElementById("chartYellow"));
var option = {
	title: {
		text: "9.2G",
		left: "center",
		top: "center",
		textStyle: {
			color: "#000",
			fontSize: 20,
			fontWeight: "lighter"
		}
	},
	tooltip: {
		trigger: "item",
		formatter: "{b}：{c}G<br />（{d}%）"
	},
	series: [{
		avoidLabelOverlap: false,
		label: {
			normal: {
				show: false
			}
		},
		radius: [
			"90%",
			"100%"
		],
		type: "pie",
		data: [{
				name: "剩余",
				value: 9.2,
				itemStyle: {
					color: "#ffdd02"
				}
			},
			{
				name: "已用",
				value: 10.8,
				itemStyle: {
					color: "#e6e6e6"
				}
			}
		]
	}],
	animation: false
};
if (option && option.series) {
	chart.setOption(option);
}
//蓝色圈圈
var chart = echarts.init(document.getElementById("chartGreen"));
var option = {
	title: {
		text: "19.4G",
		left: "center",
		top: "center",
		textStyle: {
			color: "#000",
			fontSize: 20,
			fontWeight: "lighter"
		}
	},
	tooltip: {
		trigger: "item",
		formatter: "{b}：{c}G<br />（{d}%）"
	},
	series: [{
		avoidLabelOverlap: false,
		label: {
			normal: {
				show: false
			}
		},
		radius: [
			"90%",
			"100%"
		],
		type: "pie",
		data: [{
				name: "剩余",
				value: 19.4,
				itemStyle: {
					color: "#92e5a9"
				}
			},
			{
				name: "已用",
				value: 0.6,
				itemStyle: {
					color: "#e6e6e6"
				}
			}
		]
	}],
	animation: false
};
if (option && option.series) {
	chart.setOption(option);
}
chart = null;
```

## 属性

| 参数名           | 参数类型         | 是否必须 | 默认值 | 描述                                                         |
| ---------------- | ---------------- | -------- | ------ | ------------------------------------------------------------ |
| id               | java.lang.String | 否       | 无     | 组件实例标识，同一个页面中不能重复                           |
| name             | java.lang.String | 是       | 无     | 组件实例名称，如果参数 id 未设值，则该参数值在同一个页面中不能重复 |
| title            | java.lang.String | 否       | 无     | 图表主标题                                                   |
| subTitle         | java.lang.String | 否       | 无     | 图表副标题                                                   |
| tooltipFormatter | java.lang.String | 否       | 无     | 提示框浮层内容格式器，可以是字符串模板或者 js 回调函数       |
| animation        | boolean          | 否       | true   | 是否启用图表动画效果                                         |
| datas            | java.util.List   | 是       | 无     | 数据集                                                       |
| compatible       | boolean          | 否       | false  | 是否兼容 IE8                                                 |

 tooltipFormatter：提示框浮层内容格式器，可以是字符串模板或者 js 回调函数：

字符串模板

模板变量有 {a}, {b}，{c}，{d}，{e}，分别表示系列名，数据名，数据值等。 在 trigger 为 'axis' 的时候，会有多个系列的数据，此时可以通过 {a0}, {a1}, {a2} 这种后面加索引的方式表示系列的索引。 不同图表类型下的 {a}，{b}，{c}，{d} 含义不一样。 其中变量{a}, {b}, {c}, {d} 在不同图表类型下代表数据含义为：

1. 折线（区域）图、柱状（条形）图、K线图 : {a}（系列名称），{b}（类目值），{c}（数值）, {d}（无）
2. 散点图（气泡）图 : {a}（系列名称），{b}（数据名称），{c}（数值数组）, {d}（无）
3. 地图 : {a}（系列名称），{b}（区域名称），{c}（合并数值）, {d}（无）
4. 饼图、仪表盘、漏斗图: {a}（系列名称），{b}（数据项名称），{c}（数值）, {d}（百分比）

示例：`formatter:'{b0}: {c0}<br />{b1}: {c1}' ` 

回调函数格式：`(params:Object|Array, ticket:string, callback:(ticket:string, html:string)) => string` 

回调函数格式：(params:Object|Array, ticket:string, callback:(ticket:string, html:string)) => string
图表组件数据对象类

com.wade.web.v5.tapestry.component.chart.data.ChartData


　　图表组件的单值数据项类，用于饼图，饼圈图，包括以下构造函数：

　　　　1. new ChartData(String name, Number value)

　　　　2. new ChartData(String name, String color, Number value)

　　　　3. new ChartData(String name, String color, String highlight, Number value)

　　参数说明：

　　　　1. name 数据项名称，通常是分类的显示值

　　　　2. color 数据项图形颜色

　　　　3. highlinght 数据项图形高亮颜色

　　　　4. value 数据值， 对应的数据项值

com.wade.web.v5.tapestry.component.chart.data.ChartMultipleData

　　图表组件的多值数据项类，用于柱状图，线状图，包括以下构造函数：

　　　　1. new ChartMultipleData(String name, Number[] data)

　　　　2. new ChartMultipleData(String name, String stack, Number[] data)

　　　　3. new ChartMultipleData(String name, String stack, boolean showLabel, Number[] data)

　　　　4. new ChartMultipleData(String name, String stack, String color, Number[] data)

　　　　5. new ChartMultipleData(String name, String stack, boolean showLabel, String labelPosition, Number[] data)

　　　　6. new ChartMultipleData(String name, String stack, boolean showLabel, String labelPosition, String color, Number[] data)

　　参数说明：

　　　　1. name 数据项名称，通常是分类的显示值

　　　　2. stack 数据堆叠归类名称，如果需要进行数据堆叠归类显示，这需要传入该值

　　　　3. showLabel 是否在数据项图形上显示值标签

　　　　4. labelPosition 数据项图形上显示值标签的位置

　　　　　　可选：'top', 'left', 'right', 'bottom', 'inside', 'insideLeft', 'insideRight', 'insideTop', 'insideBottom', 'insideLeftTop', 'insideLeftBottom', 'insideRightTop', 'insideRightBottom'

　　　　5. color 图形填充颜色

　　　　6. data 数据项的值集，多项必须长度一致

