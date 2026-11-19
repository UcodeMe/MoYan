# AJAX 操作
异步数据请求操作方法。

## 页面类 AJAX 请求方法

```js
ajaxSubmit(areaids, listener, params, partids, sucfn, errfn, settings)
//该方法同 $.ajax.submit，默认为异步的 Ajax Post 请求，多用来做表单提交
```

| 参数     | 描述                                                         |
| -------- | ------------------------------------------------------------ |
| areaids  | 需要提交的区域（一般是指 div 的 id），如有多个则 "," 号分隔，若该 DOM 下有 uiid，则提交的数据模式如下uiid=[{新},{老}] |
| listener | 当前页面的监听事件                                         |
| params   | 额外的自定义POST提交参数集,格式为标准的application/x-www-form-urlencoded串，&a=b |
| partids  | 需要刷新的 Part 区域，必须是 jwcid="@Part" 对应的 id，多个则 "," 号分隔 |
| sucfn    | Ajax 调用成功的回调方法，方法第一个形参为返回的 JSON 格式数据，成功与失败的标识为 x_result_code 不为 0 |
| errfn    | Ajax 调用失败的回调方法，方法第一个形参为错误编码，第二个形参为错误描述 |
| settings | JSON 格式的串，仅对当前 Ajax 请求有效，可设置 Ajax 的字符集、同步或异步等，一般该参数可以不传，或为 null |

```js
ajaxGet(page, listener, params, partids, sucfn, errfn, settings)
//该方法同 $.ajax.post，自定义post数据集时，使用该方法提交
```

| 参数     | 描述                                                         |
| -------- | ------------------------------------------------------------ |
| page     | Ajax 请求的页面名称，为空时为当前页面                        |
| listener | 当前页面的监听事件                                           |
| params   | GET 请求中的 URL参数，&a=b， 如果有特殊字符或中文需要用encodeURIComponent进行编码 |
| partids  | 需要刷新的 Part 区域，必须是 jwcid="@Part" 对应的 id，多个则 "," 号分隔 |
| sucfn    | Ajax 调用成功的回调方法，方法第一个形参为返回的 JSON 格式数据，成功与失败的标识为 x_result_code 不为 0 |
| errfn    | Ajax 调用失败的回调方法，方法第一个形参为错误编码，第二个形参为错误描述 |
| settings | JSON 格式的串，仅对当前 Ajax 请求有效，可设置 Ajax 的字符集、同步或异步等，一般该参数可以不传，或为 null |

```js
ajaxPost(page, listener, params, partids, sucfn, errfn, settings)
//该方法同 $.ajax.post，自定义post数据集时，使用该方法提交
```

| 参数     | 描述                                                         |
| -------- | ------------------------------------------------------------ |
| page     | Ajax 请求的页面名称, 为空时为当前页面                        |
| listener | 当前页面的监听事件                                           |
| params   | POST提交的参数集,格式为标准的application/x-www-form-urlencoded串，&a=b |
| partids  | 需要刷新的 Part 区域，必须是 jwcid="@Part" 对应的 id，多个则 "," 号分隔 |
| sucfn    | Ajax 调用成功的回调方法，方法第一个形参为返回的 JSON 格式数据，成功与失败的标识为 x_result_code 不为 0 |
| errfn    | Ajax 调用失败的回调方法，方法第一个形参为错误编码，第二个形参为错误描述 |
| settings | JSON 格式的串，仅对当前 Ajax 请求有效，可设置 Ajax 的字符集、同步或异步等，一般该参数可以不传，或为 null |

示例代码：

```js
//ajaxSubmit 提交
ajaxSubmit('QueryCond', 'queryCusts', '&CUST_NAME=' + encodeURIComponent($('#CUST_NAME').val()), 'QueryPart',
	function(data){
		alert(data);
	},
	function(code, info){
		alert(code + ":" + info);
	});

//ajaxGet 请求
ajaxGet('examples.CustList', 'queryCusts', '&CUST_NAME=' + encodeURIComponent($('#CUST_NAME').val()), 'QueryPart', 
	function(data){
		alert(data);
	},
	function(code, info){
		alert(code + ":" + info);
	});

//ajaxPost 请求
ajaxPost(null, 'queryCusts', '&CUST_NAME=' + encodeURIComponent($('#CUST_NAME').val()), 'QueryPart', 
	function(data){
		alert(data);
	},
	function(code, info){
		alert(code + ":" + info);
	});


//ajaxPost 同步请求
ajaxPost(null, 'queryCusts', '&CUST_NAME=' + encodeURIComponent($('#CUST_NAME').val()), 'QueryPart', 
	function(data){
		alert(data);
	},
	function(code, info){
		alert(code + ":" + info);
	},
	{
		async: false  //ajaxSubmit, ajaxGet, ajaxPost 均支持同步请求，但同步ajax请求会导致浏览器挂起，通常不建议使用
	});
						
```

完整用法： 					

```js
$.ajaxRequest({
	url:url,  //Http 请求地址
	data:data, //传入的数据
	type:"GET"|"POST", //Http 请求方式
	dataType:"text"|"json"|"script"|"jsonp",
	async:true|false,  //是否异步的 XMLHttpRequest 请求，默认为 true,
	timeout:3000,   //设置超时时间

	success:function(data){
		//data 返回的数据
		//当 dataType 为 text 时、data 为字符串，为 json 时、data 为 JSON Object
		//为script或jsonp时 data参数为undefined
		//Todo..
	},
	error:function(status,errMsg){
		//status：XHR 状态，errMsg：错误信息
		//Todo
        //只有当 async 为 true，且调用时传入了 timeout 值时才会执行该回调
	}
});
						
```

