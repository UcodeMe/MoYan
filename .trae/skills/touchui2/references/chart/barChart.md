# 柱状图（@BarChart）
以长方形的长度为变量的表达图形的统计报告图。
图表组件均封装自echarts

## HTML

<a href="#/example/chart/barChart-mul" target="_blank">查看演示</a>

```html
<div jwcid="@BarChart"
	title="产量图示"
	subTitle="纯属虚构"
	name="barChart"
	horizontal="false"
	labels="ognl:chartLabels"
	datasets="ognl:chartDatas"
	style="height:35em;">
</div>
```

注意：style 中的高度必须设置，宽度不设置时为自适应 

### 多条

<a href="#/example/chart/barChart-mul" target="_blank">查看演示</a>

```html
<div name="chart" id="chart" style="height:280px; width:500px;"></div>
```
```js
var chart = echarts.init(document.getElementById("chart"));
var option = {
	title: {
		text: "产量图示",
		textStyle: {
			fontWeight: "normal",
			fontSize: 16
		},
		padding:0
	}, 
	legend: {
		data: [
			{
				name:"数据1",
				icon:"circle"
			},
			{
				name:"数据2",
				icon:"circle"
			}
		],
		right:0,
		padding:[2,0,2,0]
	}, 
	xAxis: [
		{
			type: "category", 
			data: [
				"1月",
				"2月",
				"3月",
				"4月",
				"5月",
				"6月",
				"7月"
			],
			axisLine:{
				show: false,
				lineStyle: {
					color: "#666"
				}
			},
			axisTick: {
				show:false
			}
		}
	], 
	yAxis: [
		{
			type: "value",
			axisLine:{
				show: false,
				lineStyle: {
					color: "#999999"
				}
			},
			axisTick: {
				show:false
			},
			splitLine: {
				lineStyle: {
					color:"#e6e6e6"
				}
			}
		}
	],
	tooltip: {
		trigger: "axis", 
		axisPointer: {
			type: "shadow"
		}
	},
	series: [
		{
			name: "数据1", 
			type: "bar", 
			barWidth: "25%",
			barGap:"5%",
			itemStyle: {
				color: "#5dabe6"//柱子颜色设置
			},
			data: [83.12, 37, 76, 65, 97, 91, 78]
		},
		{
			name: "数据2", 
			type: "bar", 
			barWidth: "25%",
			barGap:"5%",
			itemStyle: {
				color: "#92e5a9"//柱子颜色设置
			},
			data: [19.3,22.7,16.4,32.9,91.2,77.4,5.5]
		}
	], 
	animation: true,
	grid:{
		top: 50,
		left: 0,
		right: 0,
		bottom: 0,
		containLabel:true
	}
};
if( option && option.series ){
	chart.setOption(option);
}
chart = null;
```
### 基准线

<a href="#/example/chart/barChart-datum" target="_blank">查看演示</a>

```html
<div name="chart" id="chart" style="height:280px; width:500px;"></div>
```
```js
var chart = echarts.init(document.getElementById("chart"));
var option = {
	title: {
		text: "产量图示",
		textStyle: {
			fontWeight: "normal",
			fontSize: 16
		},
		padding:0
	}, 
	legend: {
		data: [
			{
				name:"数量",
				icon:"circle"
			},
			{
				name:"基准线",
				icon:"circle"
			}
		],
		right:0,
		padding:[2,0,2,0]
	}, 
	xAxis: [
		{
			type: "category", 
			data: [
				"1月",
				"2月",
				"3月",
				"4月",
				"5月",
				"6月",
				"7月"
			],
			axisLine:{
				show: false,
				lineStyle: {
					color: "#666"
				}
			},
			axisTick: {
				show:false
			}
		}
	], 
	yAxis: [
		{
			type: "value",
			axisLine:{
				show: false,
				lineStyle: {
					color: "#999999"
				}
			},
			axisTick: {
				show:false
			},
			splitLine: {
				lineStyle: {
					color:"#e6e6e6"
				}
			}
		}
	],
	tooltip: {
		trigger: "axis", 
		axisPointer: {
			type: "shadow"
		}
	},
	series: [
		{
			name: "数量", 
			type: "bar", 
			barWidth: "45%",
			barGap:"-100%",
			itemStyle: {
				color: "#92e5a9"//柱子颜色设置
			},
			data: [83.12, 37, 76, 65, 97, 91, 5.5]
		},
		{
			name: "基准线", 
			type: "bar", 
			barWidth: "45%",//柱子宽度
			barGap:"-100%",
			itemStyle: {
				color: "rgba(93,171,230,0.6)"//柱子颜色设置,这里设置了0.6的半透明。
			},
			data: [20,20,20,20,20,20,20]
		}
	], 
	animation: true,
	grid:{
		top: 50,
		left: 0,
		right: 0,
		bottom: 0,
		containLabel:true
	}
};
if( option && option.series ){
	chart.setOption(option);
}
chart = null;
```

### 纵向

<a href="#/example/chart/barChart-v" target="_blank">查看演示</a>

```html
<div name="chart" id="chart" style="height:280px; width:500px;"></div>
```
```js
var chart = echarts.init(document.getElementById("chart"));
var option = {
	title: {
		text: "产量图示",
		textStyle: {
			fontWeight: "normal",
			fontSize: 16
		},
		padding:0
	}, 
	legend: {
		data: [
			{
				name:"新增数量",
				icon:"circle"
			}
		],
		right:0,
		padding:[2,0,2,0]
	}, 
	xAxis: [
		{
			type: "value", 
			axisLine:{
				show: false,
				lineStyle: {
					color: "#999"
				}
			},
			axisTick: {
				show:false
			},
			axisLabel: {
				show:true
			},
			splitLine: {
				show:true,
				lineStyle: {
					color:"#e6e6e6"
				}
			}
		}
	], 
	yAxis: [
		{
			type: "category",
			data: [
				"1月",
				"2月",
				"3月",
				"4月"
			],
			axisLine:{
				show: false,
				lineStyle: {
					color: "#666"
				}
			},
			axisTick: {
				show:false
			},
			splitLine: {
				show:false
			}
		}
	],
	tooltip: {
		trigger: "axis", 
		axisPointer: {
			type: "shadow"
		}
	},
	series: [
		{
			name: "基准线", 
			type: "bar", 
			barWidth: "30%",
			barGap:"-100%",
			itemStyle: {
				color: "#ebebeb"//柱子颜色设置
			},
			data: [100,100,100,100]
		},
		{
			name: "新增数量", 
			type: "bar", 
			barWidth: "30%",
			barGap:"-100%",
			itemStyle: {
				color: "#69afe5"//柱子颜色设置
			},
			data: [83.12, 37, 76, 65]
		}
	], 
	animation: true,
	grid:{
		top: 46,
		left: 0,
		right: 10,
		bottom: 0,
		containLabel:true
	}
	};
	if( option && option.series ){
	chart.setOption(option);
}
chart = null;
```

### 迷你

<a href="#/example/chart/barChart-mini" target="_blank">查看演示</a>

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
				<div name="chart1" id="chartYellow" style="height:100px; width:100px; margin:0 auto;"></div>
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

## 属性

| 参数名           | 参数类型         | 是否必须 | 默认值 | 描述                                                         |
| ---------------- | ---------------- | -------- | ------ | ------------------------------------------------------------ |
| id               | java.lang.String | 否       | 无     | 组件实例标识，同一个页面中不能重复                           |
| name             | java.lang.String | 是       | 无     | 组件实例名称，如果参数id未设值，则该参数值在同一个页面中不能重复 |
| title            | java.lang.String | 否       | 无     | 图表主标题                                                   |
| subTitle         | java.lang.String | 否       | 无     | 图表副标题                                                   |
| tooltipFormatter | java.lang.String | 否       | 无     | 提示框浮层内容格式器，可以是字符串模板或者 js 回调函数       |
| animation        | boolean          | 否       | true   | 是否启用图表动画效果                                         |
| horizontal       | boolean          | 否       | false  | 是否为水平柱状图                                             |
| labels           | java.util.List   | 是       | 无     | 分类标签集                                                   |
| datasets         | java.util.List   | 是       | 无     | 数据集                                                       |
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
