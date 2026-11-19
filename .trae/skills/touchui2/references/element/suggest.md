# 搜索建议（@Suggest）
当用户在输入搜索词时，搜索文本框下方会自动显示与搜索词相关的词汇供用户选择，以加快搜索速度与准确率。

## HTML
<a href="#/example/element/suggest" target="_blank">查看演示</a>
```html
<input name="mySuggest" textField="KEY" valueField="VALUE" listener="doSearch" jwcid="@Suggest" />
```
## 属性

| 参数名     | 参数类型         | 是否必须 | 默认值 | 描述                                                         |
| ---------- | ---------------- | -------- | ------ | ------------------------------------------------------------ |
| name       | java.lang.String | 是       | 无     | 组件实例名，元素 id 和 name 属性的值，也是提交表单数据时的字段名字 |
| class      | java.lang.String | 否       | 无     | 自定义样式名                                                 |
| style      | java.lang.String | 否       | 无     | 自定义样式                                                   |
| textField  | java.lang.String | 是       | 无     | 列表的文本字段                                               |
| valueField | java.lang.String | 否       | 无     | 列表的值字段                                                 |
| value      | java.lang.String | 否       | 无     | 默认值                                                       |
| disabled   | boolean          | 否       | false  | 是否禁用该组件                                               |
| page       | java.lang.String | 否       | 无     | 加载数据的页面 pageName                                      |
| listener   | java.lang.Object | 否       | 无     | 加载数据的页面监听方法                                       |
| component  | java.lang.String | 否       | 无     | 加载数据的组件 id                                            |
| clazz      | java.lang.String | 否       | 无     | 加载数据的 HttpHandler 类路径                                |
| method     | java.lang.String | 否       | 无     | 加载数据的 HttpHandler 方法名                                |
| params     | java.lang.Object | 否       | 无     | 执行搜索时需要额外传递的业务参数，可以是类似"&a=b&c=d"的参数串String或者Map类型 |

注意：page, listener, clazz, method 属性不能全为空，必须设置一种数据来源 

## Java

```java
public void doSearch(IRequestCycle cycle) throws Exception{
	
	// 从 URL 参数中获取输入的查询条件，固定使用"q"来获取
	String q = getParameter("q");
	
	// 调服务查询数据，返回数据集 IDataset
	IDataset ret = ServiceFactory.call("xxx");

	// 设置数据集到 ajax 返回数据中
	setAjax(ret);
}
```

## JS API

设置执行搜索加载数据请求的额外业务参数

```js
mySuggest.setParam('EPARCHY_CODE', '0871');
```

绑定 onchange 事件

```js
// 第一个代表 event，第二，三个形参分别为选择项的 text 和 val，如果没有设置 valueField，则 val 为 undefined
$("#mySuggest").change(function(e, text, val){
	console.log(text + "|" + val);
});
```

## 样式

Suggest输入框继承了基本输入框的能力，支持大小与圆角的设置，设置方法请参考 <a href="#/doc/element/group" target="_blank">组合框</a> 一章。 