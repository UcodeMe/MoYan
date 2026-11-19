# 地图（@Chart）
按照一定比例依据实际位置的相对坐标，描绘其二维空间的分布状况。
图表组件均封装自echarts
## HTML

<a href="#/example/chart/mapChart" target="_blank">查看演示</a>

```html
<div name="chart" id="mapChart" style="height:80%; width:80%;"></div>
```	

<!-- page

```html
<property-specification name="option" type="com.wade.web.v5.tapestry.component.chart.model.Option" />
``` -->

## 属性

| 参数名 | 参数类型                                               | 是否必须 | 默认值 | 描述                                                         |
| ------ | ------------------------------------------------------ | -------- | ------ | ------------------------------------------------------------ |
| id     | java.lang.String                                       | 否       | 无     | 组件实例标识，同一个页面中不能重复                           |
| name   | java.lang.String                                       | 是       | 无     | 组件实例名称，如果参数id未设值，则该参数值在同一个页面中不能重复 |
| option | com.wade.web.v5.tapestry. component.chart.model.Option | 是       | 无     | 图表设置对象                                                 |

 tooltipFormatter：提示框浮层内容格式器，可以是字符串模板或者 js 回调函数：

字符串模板

模板变量有 {a}, {b}，{c}，{d}，{e}，分别表示系列名，数据名，数据值等。 在 trigger 为 'axis' 的时候，会有多个系列的数据，此时可以通过 {a0}, {a1}, {a2} 这种后面加索引的方式表示系列的索引。 不同图表类型下的 {a}，{b}，{c}，{d} 含义不一样。 其中变量{a}, {b}, {c}, {d} 在不同图表类型下代表数据含义为：

1. 折线（区域）图、柱状（条形）图、K线图 : {a}（系列名称），{b}（类目值），{c}（数值）, {d}（无）
2. 散点图（气泡）图 : {a}（系列名称），{b}（数据名称），{c}（数值数组）, {d}（无）
3. 地图 : {a}（系列名称），{b}（区域名称），{c}（合并数值）, {d}（无）
4. 饼图、仪表盘、漏斗图: {a}（系列名称），{b}（数据项名称），{c}（数值）, {d}（百分比）

示例：`formatter:'{b0}: {c0}<br />{b1}: {c1}' ` 

回调函数

回调函数格式：`(params:Object|Array, ticket:string, callback:(ticket:string, html:string)) => string` 

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