# 图表如何自适应窗口变化 

考虑到性能原因，Touch UI 框架中的图表组件，在新建 chart 对象之后，都会主动销毁，所以无法实现图表自适应窗口。
但 Touch UI 的图表实际是基于 echarts 组件的封装，因此可以直接引入 echarts。
需要注意一点的是，e_charts 是不兼容 IE8 浏览器的，如果要兼容 IE8 浏览器，最好还是使用 TouchUI 封装的组件。

```html
<script src="/assets/chart/echarts.min.js"></script>
```

然后自己去 new chart 对象：
```js
var myChart = echarts.init(document.getElementById("myChart"));
var option = {
	...
}
myChart.setOption(option);
```

最后给 window.resize 事件绑定自适应的方法：
```js
window.addEventListener("resize", function(){
	chart.resize();
})
```