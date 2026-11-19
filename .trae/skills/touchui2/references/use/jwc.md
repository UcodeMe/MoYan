# 使用 Touch JWC 构建页面

## 什么是前端 JWC 框架组件

前端 JWC 框架组件是 Touch UI 2.0 中提供的纯前端 JS web 组件，它具有以下特性：

1. **纯前端组件，在客户端浏览器中运行**

	前端JWC框架组件是 JS 开发的纯前端 Touch UI 组件，它只需要浏览器即可运行，通过与后台数据交互实现动态效果。

2. **提供和 JAVA WEB 组件一致的开发方式**

	前端 JWC 框架组件仍然基于 HTML 的标签和属性来实现模板组件和组件属性的解析，并保持使用和 Java web 组件一致的 ``jwcid=@组件类型``来定义模板中的组件实例，通过 jwcid 属性所属元素的属性来定义组件实例的参数，并使用相同的的 ognl 属性来绑定动态变量。

3. **实现跨平台的模板组件解析**

	通过模板引擎一致的解析方式，基于前端 JWC 框架组件定义的 HTML 模板能够轻易的迁移到后端应用服务器上运行，由于纯前端方式不依赖于操作系统平台，这样的模板也能够在不同终端的浏览器上以本地方式进行解析，结合响应式 Touch UI，实现一次模板开发，跨平台和终端多处运行。

## 基本使用方式

准备工作：你需要一个趁手的 HTML 编辑工具和一个本地静态 web 服务器（Nginx, Apache, Nodejs 均可）。

使用本地静态 web 服务器建立一个虚拟目录到开发的工程或 demo 目录，确保可以通过浏览器访问到该目录下的静态界面，然后进行下一步。

使用前端 JWC 框架组件首先需要引入 Touch UI 框架的前端资源，包括样式和 JS 文件，这些 Touch UI 的前端资源可以在 Touch UI 的网站上获取，网站地址见“参考资料”

目录结构（<a href="static/download/TouchJWC 模版工程.rar" target="_blank">下载 TouchJWC 模版工程</a>）：

```
TouchUI
   |--css
   |--img
   |--js
assets
   |--css
   |--img
   |--js
      |--config.js // 资源文件路径配置
      |--data.js // 静态模拟数据
index.html
```

打开 index.html 引入框架前端资源示例：

```html
<script type="text/javascript" src="TouchUI/js/jcl-base.js"></script>
<script type="text/javascript" src="TouchUI/js/jcl.js"></script>
<script type="text/javascript" src="TouchUI/js/i18n/code.zh_CN.js"></script>
<script type="text/javascript" src="TouchUI/js/jcl-plugins.js"></script>
<script type="text/javascript" src="TouchUI/js/jcl-ui.js"></script>
<script type="text/javascript" src="TouchUI/js/jcl-jwc.js"></script>
```

如果 Touch UI 的前端静态资源存放在不同的相对路径下，则需要自定义相对路径，例如：

```html
<script type="text/javascript" src="assets/js/config.js"></script>
```

config.js 的内容如下：

```js
/*!
 * Touch UI adapter
 * http://www.wadecn.com/
 */
(function($, window, doc){

    var CSS_PATH = "../../TouchUI/css/";   // 定义框架 CSS 资源相对路径
    var SCRIPT_PATH = "../../TouchUI/js/"; // 定义框架 JS 资源相对路径

	//组件相关路径定义
	$.extend({
		UI_POPUP_JS: SCRIPT_PATH + "ui/component/base/popup.js", // 不需要修改
		UI_DIALOG_JS: SCRIPT_PATH + "ui/component/base/dialog.js", // 不需要修改
		UI_FRAME_JS: SCRIPT_PATH + "ui/component/base/frame.js", // 不需要修改
		UI_TABSET_JS: SCRIPT_PATH + "ui/component/tabset/tabset.js", // 不需要修改
		
		FILE_UPLOAD_URL: "attach?action=upload", // 定义附件文件上传服务路径
		FILE_DELETE_URL: "attach?action=delete", // 定义附件文件删除服务路径
		FILE_DOWNLOAD_URL: "attach?action=download", // 定义附件文件下载服务路径
		FILE_QUERY_URL: "attach?action=query", // 定义附件文件查询服务路径
		
		IMPEXP_URL: "impexp" // 定义导入导出服务路径
    });
    
	//JWC 资源路径定义
    $.extend($.JWC.resource, {
        RENDER_CSS: false,
        CSS_PATH: CSS_PATH,
        SCRIPT_PATH: SCRIPT_PATH
    });

})(window.Wade, window, document);
```

引入资源路径后，基本的 HTML 空内容模板如下：

```html
<!DOCTYPE HTML>
<html size="s">
<head>
	<meta http-equiv="X-UA-Compatible" content="IE=edge,chrome=1" />
	<meta name="viewport" content="width=device-width, initial-scale=1, minimum-scale=1, maximum-scale=1" />
	<meta http-equiv="Content-Type" content="text/html; charset=UTF-8" />
	<link href="TouchUI/css/base.css" rel="stylesheet" type="text/css" />
	<script type="text/javascript" src="TouchUI/js/jq/jquery-1.9.1.js"></script>
	<script type="text/javascript" src="TouchUI/js/jq/adapter-1.9.1.js"></script>
	<script type="text/javascript" src="TouchUI/js/jcl.js"></script>
	<script type="text/javascript" src="TouchUI/js/i18n/code.zh_CN.js"></script>
	<script type="text/javascript" src="TouchUI/js/jcl-plugins.js"></script>
	<script type="text/javascript" src="TouchUI/js/jcl-ui.js"></script>
	<script type="text/javascript" src="TouchUI/js/jcl-jwc.js"></script>
	<script type="text/javascript" src="assets/js/config.js"></script>
	<script type="text/javascript" src="assets/js/data.js"></script>
	<title>test</title>
</head>
<body jwcid="@Body">
    <!-- TODO -->
    <!-- 页面的内容 -->
    <!-- TODO -->

    <script type="text/javascript">
        //<![CDATA[
        Wade.setRatio();  // 执行 UI 响应式布局
        Wade.JWC.init(true);  // 执行前端 JWC 组件初始化，传入 true，表示初始化时即执行解析
        //]]>
    </script>
</body>
</html>
```

完成基本的基本的 HTML 空内容模板构建后，即可以往``<body jwcid="@Body"></body>``中添加页面内容和其它前端 JWC 组件，写法与原有的 JAVA WEB 组件保持基本一致，例如：

分页标签组件：

```html
<div name="myTab" jwcid="@Tabset" slidable="false">
	<div title="tab1" class="content">
		1
	</div>
	<div title="tab2" class="content">
		2
	</div>
	<div title="tab3" class="content">
		3
	</div>
</div>
```
日历组件：

```html
<div name="calendar1" jwcid="@Calendar" format="yyyy-MM-dd HH:mm:ss" time="true" value="" lunar="true"></div>
```


## 动态数据绑定

前端 JWC 组件的动态数据绑定基本使用与 JAVA  WEB 组件一致，都是通过在组件属性值中使用 ``ognl`` 前缀来标识动态变量，区别在于，Java web 组件使用 ognl 规则表达式解析属性值里的动态表达式，而前端 JWC 组件使用 JS 语法和 JSON 对象引用来解析属性值里的动态表达式，动态数据绑定示例如下：

### 下拉框组件

1. 定义数据集

	可以引入 data.js 或在执行 ``Wade.JWC.init(true);`` 之前写入对 srouce 的定义。

	引入 js：

	```html
	<script type="text/javascript" src="assets/js/data.js"></script>
	```
	js 内容：
    ```js
    var datasource =  [
        {text:"text1", value:"value1"},
        {text:"text2", value:"value2"},
        {text:"text3", value:"value3"},
        {text:"text4", value:"value4"},
        {text:"text5", value:"value5"},
        {text:"text6", value:"value6"},
        {text:"text7", value:"value7"},
        {text:"text8", value:"value8"},
        {text:"text9", value:"value9"},
        {text:"text10", value:"value10"},
        {text:"text11", value:"value11"},
        {text:"text12", value:"value12"}
    ];
    ```
2. 在组件属性中使用 ``ognl:`` 绑定数据集

    ```html
    <span jwcid="@Select"
          id="SEL2"
          name="SEL"
          source="ognl:datasource"
          addDefault="true"
          textField="text"
          valueField="value"
          optionAlign="right"
          inputable="true"
          class="e_select e_select-l e_select-r"
          nullable="yes"
          desc="选择项目">
    </span>
    ```

### 分段组件

1. 定义数据集

    ```js
    var levels = [
        {TEXT:"选项1", VALUE:"0"},
        {TEXT:"选项2", VALUE:"2"},
        {TEXT:"选项3", VALUE:"3"}
    ];
    ```

2. 在组件属性中绑定数据集

    ```html
    <span jwcid="@Segment"
          name="LEVEL"
          source="ognl:levels"
          textField="TEXT"
          valueField="VALUE"
          onchange="alert(this.value)"
          nullable="no"
          desc="级别">
    </span>
    ```

**注意**：

1. 使用 ognl 绑定的数据定义必须是在组件解析之前，通常来说就是在 ``Wade.JWC.process()`` 执行前定义好。

2. 前端 JWC 组件数据绑定中，使用 JSON 语义定义多条数据和单条数据，通常 JSON 中使用数组对象定义多条数据，使用 Object 定义单条数据。对应 JAVA 语言中的 List 和 Map。

## AJAX数据交互和局部刷新
AJAX 数据交互统一使用 JSON 数据格式，使用 POST 请求提交数据，API 如下：

```js
$.ajax.post(url, params, partids, callback, error, settings);
```

其中： 
* url 为请求 URL。
* params 为 POST 参数，对于中文或全角字符需要做 URLEncode 编码。
* partids 为要重新进行数据绑定渲染的 Part 组件 id。
* callback 为成功回调函数，形参为返回的 JSON 数据。
* error  为失败回调函数，形参为错误编码和错误信息。
* settings 为自定义配置 JSON 对象，可以覆盖默认的超时或编码设置。

示例：

```js
$.ajax.post(
	"queryCond", "data/user", "userid=1", "queryPart",
	function(data){
		//todo 数据关联处理
		$.endPageLoading();
	},
	function(code, info){
		$.endPageLoading();
		MessageBox.error("", code + "<br />" + info);
	}
);
$.ajax.submit(areaids, url, params, partids, callback, error, settings);
```

其中： 
* areaids 为要提交数据的页面区域 id，多个值用 "," 分隔。
* url 为请求 URL。
* params 为额外的 POST 参数，对于中文或全角字符需要做 URLEncode 编码。
* partids 为要重新进行数据绑定渲染的 Part 组件 id。
* callback 为成功回调函数，形参为返回的 JSON 数据。
* error  为失败回调函数，形参为错误编码和错误信息。
* settings 为自定义配置 JSON 对象，可以覆盖默认的超时或编码设置。

示例：
```js
$.ajax.submit(
	"queryCond", "data/user/list", null, "queryPart",
	function(data){
		// todo 数据关联处理
		$.endPageLoading();
	}, function(code, info){
		$.endPageLoading();
		MessageBox.error("", code + "<br />" + info);
	}
);
```

上面的 AJAX 方法结合 Part 组件即可以完成页面的局部刷新和数据绑定，示例如下：

页面模板：

```html
<div id="queryPart" jwcid="@Part">
    <div class="l_queryResult">
        <table name="mytable" class="c_table" jwcid="@Table">
            <thead>
                <tr>
                    <th><input type="checkbox" name="all" class="e_checkbox-white" /></th>
                    <th col="CUST_ID"><span>客户标识</span></th>
                    <th col="CUST_NAME"><span>客户姓名</span></th>
                    <th class="e_hide-print"><span>编辑</span></th>
                    <th class="e_hide-print"><span>删除</span></th>
                </tr>
            </thead>
            <tbody>
                <tr jwcid="@Foreach" source="ognl:infos" value="ognl:info" element="tr">
                    <td><input type="checkbox" name="row_id"  /></td>
                    <td><span jwcid="@Insert" value="ognl:info.text"/></td>
                    <td class="fn e_hide-print"><span class="e_ico-edit"></span></td>
                    <td class="fn e_hide-print"><span class="e_ico-delete"></span></td>
                </tr>
            </tbody>
        </table>
    </div>
    <div class="l_queryPage">
        <div name="navbar" jwcid="@NavBar" cond="queryCond" dataurl="data/navbar.json" count="ognl:infoCount" part="queryPart" currPage="2" pageSize="5" ></div>
    </div>
</div>
```

查询方法：
```js
function queryList(){
	if(!$.validate.verifyAll("queryCond"))
		return;
		
	// 加载页面时进行查询
	$.beginPageLoading("正在查询数据..");
	$.ajax.submit("queryCond", "data/navbar.json", null, "queryPart", function(data){
		window.infos = data.infos; // 更新数据
		$.endPageLoading();
	}, function(code, info){
		$.endPageLoading();
		MessageBox.error("", code + "<br />" + info);
	});
}
```

## 业务组件封装
业务组件封装通过继承组件基类，实现组件基本方法实现
封装示例如下：

```js
/**
 * 业务组件封装示例
 * 登录组件
 * 
 */
(function($, window, doc){  // 使用闭包处理
	"use strict";

	/**
	* 定义组件构造函数
	* 函数名为组件类名，方法体按照下面结构保持不变
	*/
	var userLogin = function(settings){
		var that = this;
		that.constructor.call(that, settings);
	};

	/**
	* 组件参数定义，格式为 参数名: {type:"参数类型", required: "是否必须(yes|no)", defaultValue:"默认值"} 
	* 其中，type为必传属性，required 和 defaultValue为非必传属性，required默认为no，
	* 如果参数名与JS中的关键字重复，则需要使用引号将其包括
	* 由于高版本浏览器DOM属性不区分大小写，因此定义组件参数和render中取值全部使用小写
	*/
	userLogin.parameters = {
		loginmode: {type: "string"},
		loginstaff: {type: "string"},
		enableverifyimg: {"type": "boolean", defaultValue: true}	
	};

	/**
	* 定义组件成员方法(原型方法),类继承的方式，使用prototype=$.extend(基类实例， 成员方法) 实现
	* 其中，可以继承的基类包括：
	* $.JWC.component.Basic JWC的组件基类，包含了组件的基本方法
	* $.JWC.component.NamedComponent 必须有name属性的组件基类，继承自Basic类，在beforeRender方法中做了name属性校验
	* $.JWC.component.FormComponent  表单组件基类，继承自NamedComponent类，增加了id属性处理
	* $.JWC.component.BodiedComponent 组件HTML结构内还包含其它HTML结构的组件基类，继承自NamedComponent类，增加了组件内HTML结构处理的类方法
	* $.JWC.component.TempComponent 模板组件类，继承自$.JWC.component.BodiedComponent，增加了模板输出类方法 
	*/
	userLogin.prototype = $.extend(new $.JWC.component.TempComponent(), {
		// 重写获取模板内容方法
		getTemplate: function(){
			var that = this;
			var html = [];

			// 获取组件参数值
			var loginMode = that.get("loginmode");
			var loginStaff = that.get("loginstaff");
			var enableVerifyImg = that.get("enableverifyimg");

			// ....push 模板内容到 HTML 数组中，模板内容可以包含其它的组件
			html.push('<div class="c_box" id="userLogin">');
			html.push('      <input jwcid="@TextField" name="userLoginServiceNumInput"');
			html.push('            nullable="no" ');
			html.push('            onenterAction="login();"');
			html.push('            datatype="mbphone" ');
			html.push('            desc="手机号码" ');
			html.push('      />');
			html.push('</div>');

			return html.join('');
		},
		//重写组件渲染方法
		render: function(writer){
			var that = this;

			// 加载组件的 JS 资源, includeScript(path, cache) cache 默认为 true
			window.includeScript("../asset/js/userLogin.js");

			// 调用父类组件渲染，默认会替换掉 jwcid 元素
			that.renderComponent();
		}
	});

	/**
	 * 注册组件，注册 API 为
	 * $.JWC.component.register(libraryid, componentid, constructor)
	 * libraryid: 组件库 id 
	 * componentid: 组件 id
	 * constructor：组件的构造函数
	 * 注册后，组件在 HTML 页面上的引用方式为：jwcid="@libraryid:componentid"
	 */
	$.JWC.component.register("foreground", "userLogin", userLogin);

})(window.Wade, window, document);
```

页面上引用组件：

```html
<div jwcid="@foreground:userLogin" ></div>
```