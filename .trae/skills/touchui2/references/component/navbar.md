# 分页（@Navbar）
分批加载大量数据时，显示批次相关统计数据、放置批次操作功能的区域。 

## JWC 方式
<a href="#/example/component/navbar" target="_blank">查看演示</a>

说明：分页组件通常和表格组件（@Table）一起使用
需要将分页组件和表格组件放在同一个 Part 组件里进行刷新
同时分页组件的 part 属性必须设置为刷新分页组件的 partId  						

```html
<div id="myQueryPart" jwcid="@Part">
	<table jwcid="@Table" name="myTable">...</table>
	<div jwcid="@NavBar"
		name="myNavbar"
		cond="queryCond"  
		count="ognl:infoCount" 
		listener="queryList" 
		part="myQueryPart"
		pageSize="50" 
		onbeforeAction="myBeforeAction()"
	></div>
</div>
```

## JS 方式

html
```html
<div class="c_page">
	<div class="more"><span class="e_ico-fold"></span></div>
	<div class="turn">
		<div class="first"><span class="e_ico-first"></span></div>
		<div class="prev">上一页</div>
		<div class="page">10/23</div>
		<div class="next">下一页</div>
		<div class="last"><span class="e_ico-last"></span></div>
	</div>
	<div class="option">
		<div class="goto">
			<div class="label">跳转至</div>
			<input type="text" />
			<button type="button">确定</button>
		</div>
		<div class="per">
			<div class="label">每页</div>
			<div class="value" id="page_per"></div>
		</div>
	</div>
</div>
```

## 属性

| 参数名     | 参数类型         | 是否必须 | 默认值       | 描述                                    |
| ---------- | ---------------- | -------- | ------------ | --------------------------------------- |
| name       | java.lang.String | 是       | 无           | 组件实例名，同一个页面里不允许重复      |
| class      | java.lang.String | 否       | 无           | 自定义样式名                            |
| style      | java.lang.String | 否       | 无           | 自定义样式                              |
| cond       | java.lang.String | 是       | 无           | 进行翻页加载数据时需要提交的条件区域 id |
| part       | java.lang.String | 是       | 无           | 包含分页组件自身的 PartId               |
| listener   | java.lang.String | 否       | 无           | 翻页时执行的页面类监听方法名            |
| component  | java.lang.String | 否       | 无           | 翻页时要执行的组件ID名                  |
| loadingTxt | java.lang.String | 否       | 'Loading...' | 翻页时的 loading 文本                   |
| count      | long             | 是       | 无           | 总数据条数，总数据条数 <= 0 时，不会显示总页数|
| pageSize   | int              | 否       | 20           | 每页数据条数                            |
| needcount   | boolean              | 否       | true           | 无统计按钮设置                            |


## JS API

绑定翻页前 beforeAction 事件

注意：beforeAction 事件返回 false 时不会执行翻页 					

```js
$("#myNavbar").beforeAction(function(){
	// 校验必填
	if(!$.validate.verifyAll("queryCond"))
		return false;

	// 判断 UL 列表中是否有数据
	return $("#ulid li").length > 0;

});
```

绑定翻页后 afterAction 事件

```js
$("#myNavbar").afterAction(function(){
	console.log("Navbar afterAction");
});
```

刷新案例

```js

function myBeforeAction(){
	// 校验必填
	if(!$.validate.verifyAll("queryCond"))
		return false;

	// 判断 UL 列表中是否有数据
	return $("#ulid li").length > 0;	
}

$(function(){
	// 加载页面时进行查询
	$.beginPageLoading("正在查询数据..");
	// 进行数据查询时，需要将包含分页组件的 PartId 传入 ajax 请求的 partId 参数中做刷新
	$.ajax.submit("queryCond", "queryList", null, "queryPart", function(data){
		$.endPageLoading();
		$("#myNavbar").beforeAction(myBeforeAction);
	}, function(code, info){
		$.endPageLoading();
		MessageBox.error("", code + ", " + info);
	});
});
```

## 样式

| 参数名        | 值            | 默认值 | 描述     |
| ------------- | ------------- | ------ | -------- |
| c_page-lite   | c_page-lite   | 无     | 简易模式 |
| c_page-border | c_page-border | 无     | 边框设置 |

