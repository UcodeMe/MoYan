# 公共方法
业务常用 JS 方法集合。

## tap 事件 

<a href="#/example/jsapi/api-tap" target="_blank">查看演示</a>

由于 click 方法在原生触屏设备上自带延时，tap 更适合于触屏设备，tap 方法同时加入了点击效果和点击取消效果，更适合触屏设备。
tap 方式代码： 					

```js
<div ontap="myFunction()">...</div>
```

事件绑定的方式： 					

```js
$("#id").tap(function(e){
	alert(this.id);
	alert(e.target);
});
```

tap 事件属于自定事件，阻止冒泡的方法也需要自定义，如：

```html
<div id="parent">
	<div id="child" ontap="alert('child')"></div>
</div>
<script type="text/javascript">
	$("#parent").tap(function(e){
		while (e.target.id != "parent") {
			if(e.target.id == "child") {
				return false;
			} else {
				e.target = e.target.parentNode;
			}
		}
		alert("parent");
	});
</script>
```

## 加载状态 Loading
参数支持 id，在指定区域显示。

可以不传参数，不传参数则全屏显示，显示文字由框架根据不同语言版本自动显示。

API
```js
$.beginLoading(title, areaId);
$.engLoading(areaId);
```
示例
```js
$.beginLoading("载入中", "tableArea");
$.endLoading("tableArea");
```

## 抛出异常

```js
$.error(msg)
```

## 文件下载
```js
$.FileTransfer.download(url)
```

对于 txt 类型的文件，可能会被浏览器直接打开而非下载下来，需要在服务器 web 应用上把 txt 文件类型的 mime-type 配置为 application/octet-stream，如果使用 SpringBoot 启动，则在应用启动类里加入如下代码：

```java
@Bean
public WebServerFactoryCustomizer webServerFactoryCustomizer() {
	return new WebServerFactoryCustomizer<ConfigurableServletWebServerFactory>() {
		@Override
		public void customize(ConfigurableServletWebServerFactory factory) {
			MimeMappings mappings = new MimeMappings(MimeMappings.DEFAULT);
			mappings.add("txt", "application/octet-stream; charset=utf-8");
			factory.setMimeMappings(mappings);
		}
	};
}
```


## 对象的属性复制或引用 

<a href="#/example/jsapi/api-extend" target="_blank">查看演示</a>

将 obj2,..对象的属性复制或引用到 obj1 上,如果参数 1 是 true，则进行深度复制，否则进行引用。 					

```js
$.extend(true,obj1,obj2,...); 或 $.extend(obj1,obj2,...);

//以下是参考例子
$.extend({},{"attr1":"v1"},{"attr2":"v2"});
```

## 遍历对象或数组

遍历 object 对象，object 可以是 typeof 为 object 的对象(包括函数)或数组 
callback 为遍历方法体，args 为 callback 方法调用时传入的参数集，对应到 callback 方法的形参上  					

遍历数组（无 args 参数）

```js
$.each([1,2,3],function(index,value){
	if(value==1)return false;
});
```

遍历数组（有 args 参数）

```js
$.each([1,2,3],function(_defaultvalue){
	if(this==_defaultvalue)return false;
},[2]);
```

遍历 JSON 对象（无 args 参数）

```js
$.each({"key1":"v1","key2":"v2"},function(name,value){
	if(name=="key2" && value=="v2" || this=="v2")return false;
});
```

遍历 JSON 对象（有 args 参数）

```js
$.each({"key1":"v1","key2":"v2"},function(_defaultvalue){
	if(this==_defaultvalue) return false;
},["v2"]);
```

如果： args 参数为空，则使用默认的形参，数组使用(index,value)的形参，object 使用(name,value)的形参，执行遍历时，当 callback 方法返回 false 时跳出遍历； 
无论有无 args 参数，遍历方法里的“this”，都引用到当前遍历到的值上。

## 判断元素是否为指定 tagName

返回 true|false，判断 dom 元素 elem 的 nodeName（也是 tagName）是否是 name 					

```js
$.nodeName(elem,name)
```

## 截取字符串首尾空格

```js
$.trim(str)
```

## 判断某元素是否为某数组的子集

返回item在数组array中的位置下标, 判断 item 项是否在 array 数组的值中存在（小于0时不存在） 					

```
$.inArray(item, array)
```

## 数组合并

将数组 array 合并到结果集数组 results 中，如果 results 参数为空则将 array 转换为标准的 js Array 对象 					

```js
$.makeArray(array, results)
```

## JSON 字符串转换为 JSON 对象

```js
$.parseJSON(data)
```

## JSON 对象转换为字符串

将 JSON 对象转换为字符串（注意只能转换对象中的值类型） 					

```js
$.stringifyJSON(jsonObject)
```

## 将 window 作为上下文来执行 js 代码串

将 window 作为上下文来执行 js 代码串 					

```
$.globalEval(scriptStr)
```

## 选中一组 checkbox

<a href="#/example/jsapi/api-checkedAll" target="_blank">查看演示</a>

```
checkedAll(boxName, checked)
```
## 反选一组 checkbox

<a href="#/example/jsapi/api-checkedOther" target="_blank">查看演示</a>

```
checkedOther(boxName)
```

## 获取一组 checkbox 的选中数目

<a href="#/example/jsapi/api-getCheckedBoxNum" target="_blank">查看演示</a>

```
getCheckedBoxNum(boxName)
```

## 获取一组 checkbox 选中的数据

<a href="#/example/jsapi/api-getCheckedValues" target="_blank">查看演示</a>

```
getCheckedValues(boxName)
```

## 表单数据填充

<a href="#/example/jsapi/api-fillArea" target="_blank">查看演示</a>

将 JSON 或 DataMap 对象按 key=dom 的 id,value=dom 的 value 的关系将值绑到 areaId 指定的区域 					

```
fillArea(areaId, data)
```

## 重置表单数据

<a href="#/example/jsapi/api-resetArea" target="_blank">查看演示</a>

重置 areaId 里的表单数据，clean:true|false，是否清空 $.data() 设置的值 					

```
resetArea(areaId, clean)
```

## 禁用或激活指定的区域

<a href="#/example/jsapi/api-disabledArea" target="_blank">查看演示</a>

禁用或激活 areaId 指定的区域 					

```
disabledArea(areaId, disabled)
```

## 打印指定的区域

<a href="#/example/jsapi/api-printArea" target="_blank">查看演示</a>

打印 areaId 指定的区域，title为弹出的打印窗口标题。 					

```
printArea(areaId, title);
```

## 获取 url 中的参数

用于获取 url 中的参数，对于通过区域传递的数据，get 方法的入参为区域 ID，然后从返回的 json 对象中获取对应的元素的值。 					

```
$.params.get("cond")
```

## 强行进入手机模式

特殊情况下，需要在 pc / pad 中展示手机的形态，可通过设置 phoneMode 来实现。 					

```
<html phoneMode="ture">
```