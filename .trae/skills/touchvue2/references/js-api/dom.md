# DOM
TouchUI 对原生的 DOM 对象进行了封装，使用方法和 jquery 基本一致。

## DOM 选择器 

使用 DOM 选择器来查找 DOM 对象

```js
$(selector, context);
```

| 参数     | 描述                  |
| -------- | --------------------- |
| selector | 选择路径              |
| context  | 在某个 DOM 元素下查找 |

| 查找方式          | HTML                         | JS                                       |
| ----------------- | ---------------------------- | ---------------------------------------- |
| 按 id             | <div id="domid">...</div>    | $("#domid")                              |
| 按 tagName 或属性 | <div tag="value">...</div>   | $("div[tag=value]") , $("div",parentdiv) |
| 按样式类名        | <div class="slide">...</div> | $("div .slide"), $("div[class=slide]")   |

按元素属性选择

| 方法                    | 说明                                         |
| ----------------------- | -------------------------------------------- |
| $('button[class]')      | 有 class 属性的 button 元素                  |
| $('button[class^=not]') | 有 class 属性且值开头匹配 not 的 button 元素 |
| $('button[class$=not]') | 有 class 属性且值结尾匹配 not 的 button 元素 |
| $('button[class]')      | 有 class 属性且值中匹配 not 的 button 元素   |

按 id 或 tagName 选择

| 方法                        | 说明                                                         |
| --------------------------- | ------------------------------------------------------------ |
| $("#dddd div")              | 查找 id 为 dddd 的元素下所有 tagName 为 div 的元素           |
| $("#dddd .div .span")       | 查找 id 为 dddd 的元素下所有 tagName 为 div 或span的元素     |
| $("#dddd div:first")        | 查找 id 为 dddd 的元素下第一个 tagName 为 div 的元素         |
| $("#dddd div:last")         | 查找 id 为 dddd 的元素下最后一个 tagName 为 div 的元素       |
| $("#dddd div:nth-child(1)") | 查找 id 为 dddd 的元素下第 N 个（N 从 1 开始）tagName 为 div 的元素 |

按 css 样式路径选择

| 方法                      | 说明                                                         |
| ------------------------- | ------------------------------------------------------------ |
| $("#dddd .slide .center") | 查找 id 为 dddd 的元素下 class 为 slide 元素下 class 为 center 的元素节点 |
| $("input:enabled")        | 查找 css 选择中正常启用的元素                                |
| $("input:disabled")       | 查找 css 选择中的非启用元素                                  |
| $("input:checked")        | 查找 css 选择中选中元素                                      |
| $("input:only-child")     | 查找 css 选择中它为父节点唯一子节点的元素                    |

取得匹配元素的所有子元素组成的集合：children()。该方法只考虑第一层子元素而不考虑任何后代元素。
取得匹配元素后面紧邻的兄弟元素的集合（但集合中只有一个元素）：next()。
取得匹配元素前面紧邻的兄弟元素的集合（但集合中只有一个元素）：prev()。
取得匹配元素前后所有的兄弟元素: siblings()。 		

## DOM 对象扩展 

| 方法                              | 说明                                                         |
| --------------------------------- | ------------------------------------------------------------ |
| $("#id").attr(attrname,attrvalue) | 获取或设置 DOM 对象的某个属性值，当 attrvalue 参数不传的时候为取值，否则为设值 |
| $("#id").val(value)               | 获取或设置 DOM 对象的 value 值，当 value 参数不传时为取值，否则为设值 |
| $("#id").css(stylename,value)     | 获取或设置 DOM 对象的样式属性，当 value 参数不传时为取样式属性，否则为设置样式属性 |
| $("#id").addClass(className)      | 添加 DOM 对象上样式名                                        |
| $("#id").removeClass(className)   | 移除 DOM 对象上的样式名                                      |
| $("#id").html(value)              | 获取或设置 DOM 对象的 innerHTML，当 value 参数不传时为获取 innerHTML，否则为设置 innerHTML |
| $("#id").outerHtml()              | 获取 DOM 元素的 outerHTML                                    |
| $("#id").data(key,value)          | 设置 DOM 元素的关联数据                                      |
| $("#id").removeData(key)          | 删除 DOM 元素的关联数据                                      |
| $("#id").empty()                  | 移除 DOM 元素的所有子节点                                    |
| $("#id").remove()                 | 移除 DOM 元素                                                |
| $("#id").focus()                  | 取得焦点                                                     |
| $("#id").select()                 | 选中                                                         |
| $("#id").offset()                 | 获取元素在页面上的绝对位置，返回对象 o.left,o.top            |
| $("#id").position()               | 获取元素在页面上的位置，返回对象 o.left,o.top                |
| $("#id").width()                  | 获取元素的宽度                                               |
| $("#id").height()                 | 获取元素的高度                                               |
| $("#id").chidren()                | 获取当前元素的第一层子节点元素                               |
| $("#id").parent()                 | 获取当前元素的父节点元素                                     |
| $("#id").parents("[attr=val]")    | 查找当前元素所有父节点元素中 attr 属性为 val 的节点元素      |
|$("#id").toggleClass("className")  | 用于添加/删除某个 class      |

获取当前页面宽度：$(document.body).width();
获取当前页面高度：$(document.body).height(); 		

## DOM 元素创建和插入 

在 el 元素最后（结束 tag 之前）插入 html 片段

```js
$(el).append(html)
// 例如：
$("#ul").append('<li>title</li>');
```

在 el 元素开始标签之后插入 html 片段，做为第一个子节点插入 

```js
$(el).prepend(html);
// 例如：
$("#ul").prepend('<li>title</li>');
```

在 el 元素之前（开始 tag 之前）插入 html 片段

```js
$(el).before(html)
// 例如：
$("#ul").before('<div>title</div>');
```

在 el 元素之后（结束 tag 之后）插入 html 片段

```js
$(el).after(html)
// 例如：
$("#ul").after('<div>title</div>');
```

插入或替换元素下的 html

```js
$(el).html(html)
// 例如：
$("#ul").html('<li>title</li>');
```

## JS 模板解析 

构造方法

```js
var tpl=$.Template(template);	
// 以下是使用示例
var tpl=$.Template('<div id="list_{PRODUCT_ID}" product_name="{PRODUCT_NAME}"></div>');
```

template 为字符串类型的 html 模板，模板中使用花括弧“{}”来指明需要使用数据绑定的名称 					

解析和生成 DOM

使用 var tpl=$.Template(tempStr) 构造模板对象后可以通过它的原型方法来解析和生成 DOM
使用 tpl.applyTemplate(values(jsonobject|$.DataMap)); 生成解析后的 html 串
也可以直接将解析生成的 html 插入到某个 DOM 元素下，调用方式如下： 					

| 方法                                                         | 描述                                                         |
| ------------------------------------------------------------ | ------------------------------------------------------------ |
| tpl.append(el, values(jsonobject\|$.DataMap), returnElement); | 将模板解析生成的DOM元素插入到 el 元素的结束标签之前，returnElement 为 true 时，返回新插入 DOM 的封装对象，否则返回 DOM 原生对象 |
| tpl.insertFirst(el,values(jsonobject\|$.DataMap),returnElement); | 将模板解析生成的 DOM 元素做为 el 元素的第一个子节点插入      |
| tpl.insertBefore(el,values(jsonobject\|$.DataMap),returnElement); | 将模板解析生成的 DOM 元素插入到 el 元素的开始标签之前        |
| tpl.insertAfter(el,values(jsonobject\|$.DataMap),returnElement); | 将模板解析生成的 DOM 素插入到 el 元素的结束标签之后          |

模板中数据绑定时的数据处理

html 模板中的数据绑定可以调用 $.format 中的字符串处理函数来对绑定的数据进行处理，调用方式如下： 					

| 方法                | 描述                          | 示例                                                         |
| ------------------- | ----------------------------- | ------------------------------------------------------------ |
| $.format.trim       | 调用：{value:trim}            | <span class="title">{TITLE:trim}</span>                      |
| $.format.elppsis    | 调用 {value:elppsis(10,true)} | <span class="title">{TITLE:elppsis(10,true)}</span>          |
| $.format.undef      | 调用 {value:undef}            |                                                              |
| $.format.substr     | 调用 {value::substr(0,3)}     |                                                              |
| $.format.lowercase  | 调用 {value:lowercase}        |                                                              |
| $.format.uppercase  | 调用 {value:uppercase}        |                                                              |
| $.format.fileSize   | 调用 {value:fileSize}         |                                                              |
| $.format.htmlEncode | 调用 {value:htmlEncode}       |                                                              |
| $.format.htmlDecode | 调用 {value:htmlDecode}       |                                                              |
| $.format.math       | 调用 {value:math("* 10")}     | var tpl=$.Template('<span>计算结果：{value} * 10 = {value:math("* 10")}</span>') |
| $.format.number     | 调用 {value:number('0.00')}   |                                                              |
| $.format.nl2br      | 调用 {value:nl2br}            |                                                              |
| $.format.toUtf8     | 调用 {value:toUtf8}           |                                                              |
| $.format.toGbk      | 调用 {value:toGbk}            |                                                              |

$.DataMap 和 $.DatasetList 对象中集成的数据绑定方法

$.DataMap 数据绑定 					

```js
data.bind(selector,displayStyle);	
```

| 参数         | 描述                                                       |
| ------------ | ---------------------------------------------------------- |
| selector     | 为要绑定数据的容器元素选择串，如果有多个，则绑定第一个     |
| displayStyle | 为模板的 display 样式，如果为空，则使用 display:block 样式 |

示例 					

HTML 					

```js
<div id="user_info">
	<div x_tag="x-databind-template" style="display:none">
	<span>用户姓名：{USER_NAME}</span>
	<span>用户地址：{USER_ADDR}</span>
	</div>
</div>
```

data： 					

```js
var d=$.DataMap({"USER_NAME":"王国强","USER_ADDR":"北京二环"});
```

绑定： 					

```js
d.bind("#user_info");
```

$.DatasetList 数据绑定

```js
datalist.bind(selector,displayStyle);	
//示例：参数用途和 $.DataMap.bind 方法参数一致
```

HTML 					

```js
<ul id="menu_list">
	<li x_tag="x-databind-template" style="display:none"><a href="{MENU_URL}">{MENU_TITLE}</a></li>
</ul>
```

data： 					

```js
var ds=$.DatasetList([{"MENU_ITLE":"产品变更","MENU_URL":"app?service=page/ProductChange"},
{"MENU_ITLE":"开户","MENU_URL":"app?service=page/Open"}]);
```

绑定： 					

```js
ds.bind("#menu_list","inline-block");
```

【注】模板的 html 元素上必须加 x_tag="x_tag=x-databind-template" 属性 					

## 为 DOM 绑定事件 

为 DOM 绑定事件

```js
$(selector).bind(eventType,eventHandler);
$(selector).bind(eventType,data,eventHandler);	
```

如果 $(selector) 查找到多个 DOM 元素，则每个 DOM 元素都会绑定该事件
参数 eventType 可以是一个或多个事件类型，如果是多个则用空格分隔开
参数 data 为非必须参数，如果传入了 data，则可以通过 eventHandler 的形参 e.data 来取的 					

```js
//例如：
$("#input").bind("click",function(){

});
$("#dd div").bind("click keydown",[1,2],function(e){
	alert(e.data);	//打印“1,2”，传入的数组内容
	alert(e.which);	//鼠标左键为 1，中键为 2，右键为 3，如果是 keydown 事件，则 e.which 为对应的 keycode
	alert(e.pageX);	//打印鼠标光标在页面上点击的 x 坐标（left 值）
	alert(e.pageY);	//打印鼠标光标在页面上点击的 y 坐标（top 值）
	return true;	//如果 return false; 则停止 DOM 元素原本的事件触发
	//例如，在 submit 按钮绑定的事件方法里 return false 则不会触发提交表单
});
```

解除 DOM 的事件绑定

```js
$(selector).unbind(eventType,eventHandler);	
```

参数 eventType 可以是一个或多个事件类型，如果是多个则用空格分隔开
参数 eventHandler 为要解除的绑定事件
如果 eventHandler 参数为空，则解除所有 DOM 元素上类型为 eventType 的事件绑定 			

```js
$("#input").unbind("click",inputClick);
$("#input").unbind("click"); 	
```

如果 eventType 和 eventHandler 参数都为空，则解除 DOM 元素上所有绑定的事件 			

通过 js 代码来触发 DOM 元素的事件

```js
$("#input").trigger("click");	
```

触发 input 元素的 click 事件
包括通过 bind 方法绑定的事件和 html 里写的 onclick 事件
如果 DOM 元素既通过 bind 绑定了事件，又在 html 写了同样的事件
那么 trigger 方法先调用通过 bind 绑定的事件，再调用 html 的事件 			

DOM Ready 事件

页面载入完成时调用 			

```js
$(document).ready(function(){
	//todo
});
```

