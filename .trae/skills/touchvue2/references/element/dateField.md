# 日期输入框（@DateField）  

日期输入框是扩展的表单元素之一，主要用于输入/选择日期。

## JWC 方式
<a href="#/example/element/dateField" target="_blank">查看演示</a>
```html
<input jwcid="@DateField" 
	name="myDateField" 
	dropDown="true" 
	onafterAction="verifyAll('cond')" 
	value="1988-03-09" 
	format="yyyy-MM-dd"
/>
```

## JS 方式
```html
<span class="e_group">
	<span class="e_groupRight"><span class="e_ico-date"></span></span>
	<span class="e_groupMain">
		<input type="text" x-wade-uicomponent="datefield" id="myDateField" name="myDateField" placeholder="请选择" />
	</span>
</span>
```
```js
window["myDateField"] = new Wade.DateField(
	// 对应元素，在 el 元素下生成下拉框，el 可以为元素 id，或者原生 dom 对象
	"myDateField",
	// 参数设置
	{
		dropDown:true,
		now:new Date(1477967371975),
		value:"1988-03-09",
		format:"yyyy-MM-dd",
		useTime:false,
		useLunar:false
	}
);
```


## 属性

|参数名|参数类型|是否必填|默认值|描述|演示|
|-|-|-|-|-|-|
|id|java.lang.String|否|无|组件 id，同一个页面里不能重复，如果不设置则默认取，name 属性值||
|name|java.lang.String|是|无|组件实例名||
|value|java.lang.String|否|无|当前值||
|format|java.lang.String|否|"yyyy-MM-dd"|日期显示格式，时分秒格式 "HH:mm:ss"|<a href="#/example/element/dateField-format" target="_blank">查看</a>|
|time|boolean|否|false|是否显示时分秒功能，默认不显示，仅下拉模式有效|<a href="#/example/element/dateField-time" target="_blank">查看</a>|
|dropDown|boolean|否|false|是否使用下拉日期模式||
|readonly|boolean|否|false|是否只读||

## JS API

绑定 afterAction 事件

```JS
$("#myDateField").bind("afterAction", function(){
	alert(this.value);
});
```

设置默认值

```JS
$("#myDateField").val("TouchUI")
```

## 样式

日期输入框继承了组合框的能力，支持大小与圆角的设置，设置方法请参考<a href="#/doc/element/mix" target="_blank">组合框</a>。
