# 前台-界面框架

前台界面框架基于模板化、组件化技术实现界面生成和界面组装 

## 框架结构
![IE8IconPosition](../static/img/doc/2-0.png) 

- 框架在服务端支持页面和组件模型、通过模板引擎支持页面模板和组件模板的解析和输出。
- 框架提供一套响应式前端 UI，支持多终端、多平台、多浏览器兼容
- 框架提供 JS 脚本库，封装了 DOM 操作、事件绑定、与服务端交互等功能
- 框架基于组件模型和前端UI封装一系列核心的动态组件提供给开发使用
- 前端和服务端使用统一的数据总线 (IData 和 IDataset) 进行数据交互


## 运行流程
![IE8IconPosition](../static/img/doc/2-1.png) 

- 启动应用服务器，加载部署的 WEB 应用
- WEB 应用寻找部署描述符（web.xml），做为程序运行的入口,并加载 servlet、filter、monitor、session 的一些配置
- 客户端提交访问路径发起请求到服务器端，服务器端在 web.xml 中寻找到客户端访问的 Servlet
- 通过 app Servlet 中配置的名称找到同名的 .application 配置文件,如 quickstart.application
- 加载 .application 中配置，包括 Engine、Visit、Page 的一些配置，并初始化 Engine,Page 等实例
- 根据客户端提交访问的页面别名，在 .application 文件中找到注册的页面别名对应的 page 文件
- 通过 page 文件找到对应的 java，html 文件，进行页面、组件的执行和界面输出

```html

<?xml version="1.O" encoding="UTF-8"?>
<!DOCTYPE application PUBLIC
	"-//Apache Software Foundation//Tapestry Specification 3.0//EN"
	"http://jakarta.apache.org/tapestry/dtd/Tapestry_3_0.dtd"[
	<!ENTITY public SYSTEM "public.application">
	<!ENTITY biz SYSTEM "biz.application">
	<!ENTITY frame SYSTEM "frame.application">
]>

<application name="touchframe" engine-class="com.wade.web.v5.tapestry.engine.BaseEngine">
	<description>touchframe</description>
	<property name= "org.apache.tapestry.disable-caching" value="true"/>
	<property name= "org.apache.tapestry.enable-reset-service" value="true"/>
	<property name="org.apache.tapestry.template-encoding" value="UTF-8"/>
	<property name= "org.apache.tapestry.output-encoding" value="UTF-8"/>
	<property name= "org.apache.tapestry.page-login" value="Home"/>
	<property name="org.apache.tapestry.page-exception" value="Exception"/>
	<property name="org.apache.tapestry.visit-class">com.ailk.biz.BizVi3it</property>

	&pub1ic;
	&frame;
 </application>
```

## 页面开发

- 一个页面由一个页面模板 (.html)，一个页面规范 (.page),一个页面类 (.java) 组成
- 每一个页面有一个唯一的名字，页面名字用来定位页面规范和页面模板，页面名字定义在 application 配置文件中
- 页面规范会定位页面模板对应的页面类，以及定义页面模板中用到的属性方法，
- 页面类中定义需要给 HTML 模板使用的属性和监听方法

### 页面开发基本步骤：

1. 准备页面需要的资源，包括页面模板 (.html)、页面规范 (.page)、页面类 (.java)，以及具体的业务逻辑
2. 注册页面，在 .application 文件中将写好的页面规范 (.page) 注册到 .application 中，并定义唯一的名字。
3. 配置测试菜单，在数据库或菜单页面中新加菜单项，点击菜单打开页面进行调试

```html
<page name="Home" specificacion-path="/frame/Home.page" version="1212" />
<page name="SS04ALogin" specification-path="/frame/5504ALogin.page" version="1212" />
<page name="SSOCHLogin" specification-path="/frame/SSOCHLogin.page" version="1212" />
<page name="Resource" specification-path="/frame/Resource.page" version="1212" />
<page name-"touchframe.frame.pc.Login" specification-path="/frame/pc/Login.page" version="1212" />
<page name="touchframe.frame.pc.Welcome" specification-path="/frame/pc/Welcome.page" version="1212" />
<page name="touchframe.frame.pc.Main" specification-path="/frame/pc/Main.page" version="1212" />
<page name="touchframe.Erame.pc.SearchAll" specification-path="/frame/pc/SearchAll.page" version="1212" />
<page name="touchframe.frame.pc.SearchErame" specification-path="/frame/pc/SearchFrame.page" version="1212" />
<page name="touchframe.frame.pc.SearchMenu" specification-path="/frame/pc/SearchMenu.page" version="1212" />
<page name="touchtrame.frame.pc.SearchPhone" specification-path="/frame/pc/searchPhone.page" version="1212" />
<page name="touchframe.frame.pc.SearchProduct" specification-path="/frame/pc/searchProduct.page" version="1212" />
<library id="frame" specification-path="/com/ai.touch/touchframe/view/frame/component/component.library" />
```
## 1. 页面规范编写
页面规范是扩展名为 .page 的文本文件，它作为衔接页面模板和页面类的枢纽，需要定义页面类、页面模板的路径，同时需要定义页面模板所用到的所有对象，如：

```html
<?xml version="1.0" encoding="UTF-8"?>
<! DOCTYPE page-speci fication PUBLIC
	"-//Apache Software Foundation//Tapestry Specification 3.0//EN"
	"http://jakarta.apache.org/tapestry/dtd/Tapestry_3_0.dtd">
<!-- 定义页面类的路径 -->
<page-specification class="com.ai.saas.customer.vc.cust.QueryCust">
	<!-- 定义页面模板需要用到的对象名以及对象的结构 -->
	<property-specification name="cond" type="com.ailk.common.data.IData"/>
	<property-specification name="custInfos" type="com.ailk.common.data.IDataset"/>
	<property-specification name="custInfo" type="com.ailk.common.data.IData"/>
	<property-specification name="rowIndex" type="int"/>
	<property-specification name="custsCount" type="long"/>
	<property-specification name="editInfo" type="com.ailk.common.data.IData"/>
	<property-specification name="custStatus" type="com.ailk.common.data.IDataset"/>
</page-specification>

```
补充说明：

a. 页面规范中能够定义页面类的路径，页面规范对应的页面模板的映射规则就是页面规范

b. 文件同目录下同文件名(不含扩展文件名)的 .html 文件

c. 一个页面规范必须对应一个页面模板和一个页面类

d. 一个页面模板必须对应一个页面规范，而一个页面类可以对应多个页面规范，也就是说多个页面模板可以共用一个页面类

## 2. 页面类编写
页面类作为 MVC 中的控制层，主要是用来衔接业务逻辑与页面之间的枢纽，将页面的请求提交给业务逻辑，由业务逻辑处理后反馈结果给页面。
页面类主要编写响应页面的事件以及反馈给页面的结果对象，如：

```js
public abstract class QueryCust extends Page {
	public abstract void setCustInfo(IData custInfo);
	public abstract void setCustInfos(IDataset dataset);
	public void queryCusts(IRequestCycle cycle) throws Exception{  
		IData pageData = this.getData();
		IData cond = getData ("cond");
		IData result = ServiceFactory.call("CC.customer.queryCustomer", cond, getPagination());
		setCustInfos(result.getDataset("DAIAS"));
	}
}
```
补充说明：

a. 页面类必须继承 wade-web 中的 com.ailk.web.BasePage 类或者其子类，否则无法和页面模板、页面规范组成一个功能

b. 定义的提供给页面模板使用的属性方法需要使用抽象方法

c. 定义的响应事件，必须带有参数 IRequestCycle cycle，否则页面中无法识别该方法是基于页面的响应事件

## 3.页面模板编写
页面模板主要是将业务逻辑中获取的数据展现在页面中，以及对页面操作的一些控制，页面模板基本结构如下：
```html
<!DOCTYPE HTML>
<html>
<head jwcid="@Head">
	<title>客户信息查询</title>
</head>
<body jwcid="@Body">

</body>
</html>
```
补充说明：

a. 页面中模板中用 jwcid 标识的表示动态内容，head 表示页面模板的相关资源及说明，其中 Head 组件主要是导入公用的一些资源，如 JavaScript 库，Body 组件对于大部分组件来说是组件容器的支撑

b. ognl 标记表示用 Java 表达式来动态表达内容

## 组件开发
前台 WEB 组件是对前台公用界面和功能可重用部分的封装实现。
### 组件开发分为两种：

- 单一型组件，一般基于最小粒度的组件使用单一型组件，该类型的组件的HTML代码在组件类中编写，所以组件只包含组件类，组件规范，单一型组件又分为表单型组件和普通单一组件。
- 组合型(模板)组件，一般基于一块页面区域使用组合型组件，该类型的组件的HTML代码在组件模板中实现，所以组件包含组件模板，组件类，组件规范。

### 页面开发基本步骤：

01. 准备组件需要的资源，包括组件模板(.html)、组件规范(.jwc)、组件类(.java)。
02. 编写组件，对于组合型组件需要编写组件模板，对于单一性组件则不需要。
03. 注册编写好的组件注册到组件库中，在. library文件中将写好的组件规范注册到. library中，并定义唯一的组件名字。
04. 注册组件库，在.application文件中将写好的组件库文件注册到.application中，并定义唯一的组件库名字。
05. 在页面中使用jwcid=”@组件库注册名:组件注册名”的方式引入组件，并进行调试。

### 1. 组件规范编写

组件规范作为衔接组件模板和组件类的枢纽，需要定义组件类、组件模板的路径，同时需要定义组件模板所用到的所有对象，如：

```html
<?xml version="1.O" encoding="UTF-8"?>
<! DOCTYPE component-specification PUBLIC
	"-//Apache Software Foundation//Tapestry Specification 3.0//EN" 
	"http://jakarta.apache.org/tapestry/dtd/Tapestry_3_0.dtd">
<component-specification class="com.ai.saas.component.jwcs.demo.TplComponent">
      <parameter name="staffId" type="java.lang.String" direction="in" required="no"/>
</component-specificatipn>
```

补充说明：

- 组件规范中能够定义组件类的路径，组件规范对应的组件模板的映射规则就是通过组件规范文件同目录下同文件名(不含扩展文件名)的.html文件。
- 组件中定义的所有属性，都可以开发给使用者，这样引用组件时可以随意定义组件的属性达到动态使用组件的效果。

	组件规范配置属性说明： 						

	omponent-specification 组件定义节点，包括以下常用属性： 							

| 属性名                    | 描述                             |
| ------------------------- | -------------------------------- |
| class                     | 组件类的路径                     |
| allow-body                | yes\|no 是否允许组件中有 body     |
| allow-informal-parameters | yes\|no 是否允许组件有非正式参数 |

parameter 组件参数配置节点，包括以下常用属性:

| 属性名        | 描述                                                         |
| ------------- | ------------------------------------------------------------ |
| required      | yes\|no 该参数是否必须                                       |
| direction     | form\|in 参数的输入输出形式，form 表示为输入/输出型，in 表示输入型 |
| default-value | 参数为空时的默认值                                           |

### 2.组件类编写（单一型）

单一型组件通常用于表单元素的组件封装，并在类中直接输出要构造的 html 结构

```js
public abstract class FormComponent extends BaseComponent {
	public abstract String getStaffId();
	@Override
	public void renderComponent(StringBuilder builder, IMarkupWriter writer, IRequestCycle cycle) throws Exception{
		writer.begin ("input");
		writer.attribute("name", "STAFF_ ID");
		writer.attribute("type", "hidden");
		writer.attribute("value", getStaffId());
		writer.end();
	}
}
```
补充说明：

a. 组件类必须继承 com.ailk.web.BaseComponent 或者其子类。

b. 组件类必须实现 renderComponent(StringBuilder builder, IMarkupWriter writer, IRequestCycle cycle) throws Exception 方法

### 3. 组件类编写(组合型)

组合型组件的组件类是用来衔接业务逻辑与组件之间的枢纽，将处理后的结果反馈给组件，并将数据绑定到组件模板上，如：
```js
public abstract class FormComponent extends BaseComponent {
	public abstract String getStaffId();
	@Override
	public void renderComponent(StringBuilder builder, IMarkupWriter writer, IRequestCycle cycle) throws Exception{
		writer.begin ("input");
		writer.attribute ("name", "STAFF_ID");
		writer.attribute("type", "hidden");
		writer.attribute ("value", getStaffId());
		writer.end();
	}
}
```

补充说明：

- 组件类必须继承 com.ailk.web.BaseTempComponent 或者其子类，否则无法和组件模板、组件规范组成一个功能。
- 组件类必须实现 renderComponent(StringBuilder builder, IMarkupWriter writer, IRequestCycle cycle) throws Exception 方法，该方法用来展现组件模板中的效果

### 4.组件模板编写(组合型)

- 只有组合型组件才有组件模板
- 组件模板是与组件规范文件在同一目录(类路径下)、同名且扩展名为 html 的文件
- 组件模板通常是一个 HTML 片段，主要是将组件类调用业务逻辑中获取的数据展现在页面中，组件模板结构如下：
```html
<div class="content">
	<ul id="msgList">
		<li jwcid="@Foreach" source="ognl:infos" value="ognl:info" element="li">
			<span jwcid="@Insert" value="ognl:info.TITLE"</span>
		</li>
	</ul>
</div>
```

补充说明：

- 组件模板中可以嵌套使用其他组件，嵌套的除核心组件库外，其他的组件需要在组件库中定义。

### 5.组件注册

注册组件到组件库中(.library)，如：

```html
<?xml version="1.O" encoding="UTF-8"?>
	<!DOCTYPE library-specification PUBLIC
	"-//Apache Software Foundation//Tapestry Specification 3.0//EN"
	"http://jakarta.apache.org/tapestry/dtd/Tapestry_3_0.dtd">
<library-specification>
	<component-type type="TplComponent" specification-path="demo/TplComponent.jwc"/>
	<component-type type="FormComponent" specification-path="demo/FormComponent.jwc"/>
</library-specification>

```
注册组件库到全局配置中(.application)，如：

```html
<library id="demo" specification-path="/com/ai/saas/component/jwcs/demo.library" />

```

## 数据交互

### 1.数据格式

- 前台的页面类，组件类中统一使用 com.ailk.common.data.impl.DataMap和com.ailk.common.data.impl.DatasetList 两种数据类向前端返回数据，通常在页面和组件规范定义中使用对应的接口类定义数据类型，对应的接口类: com.ailk.common.data.IData 和 com.ailk.common.data.IDataset 										
- 前端页面中对应的 javascript 数据结构，分别是 Wade.DataMap 和 Wade.DatasetList

### 2.前端和前台数据交互

前端界面与前台服务端进行数据交互主要有以下两种方式：

1. ognl 数据绑定方式

- 页面或组件可以在 java 类中设置绑定的数据，在页面或组建模板中通过 ognl 绑定页面或组建规范中定义的数据，在页面的监听方法执行或组件的 render 执行就可以将动态数据加载或展现在前端。
- 前端通过 ajax 方式对页面进行组件刷新或局部 Part 刷新时，也可以进行同样的 ognl 数据绑定，并将数据绑定后生成的 html 片段返回给前端。

2. JSON 数据交互方式

- 前端使用 ajax 方式与前台服务端交互时，页面类或 HttpHandler 类可以通过 ``setAjax(IData)`` 或 ``setAjax(IDataset)`` 的方式，向前端返回 JSON 格式数据。
- 框架默认把返回数据解析成 JS 的 ``$.DataMap`` 或 ``$.DatassetList`` 数据类型，也支持自定义的方式进行解析，具体参见 API 文档。

## 常见问题

### 1.中文和全角字符编码问题

要求所有的 HTML、配置文件、java 代码均等文本文件均使用 UTF-8 字符集编码，在开发时，需要将 Eclipse 工程的默认文本编码设置为 UTF-8。

服务端配置：

本地开发时，需要在 Tomcat 的服务配置 server.xml 里配置 URI 参数字符集：
```html
<Connector port="8080" protocol="HTTP/1.1"
	connectionTimeout="20000" 
	useBodyEncodingForURI="false"
	redirectPort="8443" 
	URIEncoding="UIF-8" />
```

配置： ``useBodyEncodingForURI="false" URIEncoding="UTF-8"``

测试、生产环境，需要设置在 BES 中间件集群服务的WEB容器自定义属性中配置 URI 字符集参数：
![IE8IconPosition](../static/img/doc/6-2.png) 


配置自定义属性：useBodyEncodingForURI 为 false， uriEncoding 为 UTF-8

测试、生产环境需要配置 JVM 文件编码和中文参数配置：-Dfile.encoding=UTF-8 -Duser.language=zh -Duser.region=CN

中文和全角字符的传值取值：

对于框架中以下常用 JS 的 API 自定义参数部分，如果有中文或全角字符的，均需要做编码处理：

```js
openNav(title, page, listener, params subsys)
redirectNav(title, page, listener, params, subsys)
$.redirect.buildUrl(subsys, page,  listener,params)
popupPage(title, page, listener, params, subsys, className, afterAction, hideAction)
forwardFopup(node, title, page, listener, params  subsys)
$.ajax.submit(areaids, listener, params, partids, sucfn, errfn, settings)
$.ajax.get(page, listener, params, partids, sucfn, errfn, settings)
$.ajax.post(page, listener, params,  partids, sucfn, errfn, settings)
$.httphandler.submit(areaids, clazz, method, params,  sucin, errfn, settings)
$.httphandler.get(clazz, method, params. sucfn, errfn, settings)
$.httphandler.post(clazz, method, params, sucfn,  errfn, settings)
```

对于其它情况 http get 请求中有中文或全角参数的，也应该要做编码处理

编码方式(JS)：``"&CUST_ID=1234456&CUST_NAME=" + encodeURIComponent("张三")``

编码方式(ognl): ``templateFile="ognl:'examples/template/' + urlEncode('任务模板') + '.xls'"``

编码方式(java): ``java.net.URLEncoder.encode(str, "UTF-8")``

服务端取值时，不需要再做任何解码处理。
### 2.页面和组件实例缓存问题

Tapestry 模板引擎默认会对页面和组件的模板、页面和组件的实例进行缓存以提升性能、通常在开发时会设置 JVM 参数-Dorg.apache.tapestry.disable-caching=true 设置来禁用这类缓存以方便开发调试，但在测试、生产环境则不应禁用它。

对于组件来说，缓存了实例则意味着同样会缓存实例的属性值。因此对于涉及业务数据属性的组件，需要在重写组件的 cleanupAfterRender 方法，将业务数据属性清空。

例如：组件 BasePopupField 有一系列属性

```js
public abstract class BasePopupField extends BizComponent {
	public abstract String getText();
	public abstract void setIext (String text);
	public abstract String getValue();
	public abstract void setValue(String value);
	public abstract String.getTextName();
	public abstract void setTextName(String textName);
	public abstract String getName();
	public abstract void setName(String name);
}
```
在组件重写的 cleanupAfterRender 方法里进行清空：
```js
@override
protected void cleanupAfterRender(IRequestCycle cycle) {
	super.cleanupAfterRender(cycle);
	setText(null);
	setValue(null) ;
	setTextName(null);
	setAction(null) ;
}
```


若不做清除处理，则存在组件加载未给属性重新设值的情况下仍然使用上一次属性值的情况。
### 3.BES 中间件配置文件缓存问题

前台界面组件包含组件属性定义 .jwc 文件，组件库定义 .library 文件，在编译发布时，这些文件都会统一打包到 jar 包中去。jar 包中也可能包含其它一些功能的xml或properties配置文件。

BES 中间件在应用发布后，会把 jar 包中的这些配置文件提取出来并缓存到以下目录：

domain/nodes/server-node/instances/server 实例 /tmp/jsp/apps/ 部署应用 /loader

重新发布应用时，该目录下的文件可能不会被更新，会导致界面组件属性定义、组件库定义文件修改后发布不生效的情况。

遇到此类情况，可以删除该目录下缓存的配置文件后重新发布应用解决。

