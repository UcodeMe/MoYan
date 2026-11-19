# UI 设计-零基础入门（上篇）

UI 设计教程是专为使用 Touch UI 提供界面模版的工程人员所特别撰写的系列教程，也适用于前后端分离模式开发入门，适合 UI 设计师、前端工程师阅读。

## 一、先来准备工具与资源

### 	1. 工具：Sublime Text、Visual Studio Code、Atom 等编辑器均可
如果你从未使用过一款编辑器，不用着急，先下载个编辑器并安装好，我们并不需要使用编辑器中太高级的功能，只需使用他们打开 HTML、在里面复制粘贴编辑代码，最后保存就行了。 （小编用的 Visual Studio Code）。

## 二、开始写界面
### 	1. 新建一个 HTML 文件
框架资源直接在 Touch UI 主站“下载”频道，下载最新的即可，且不要修改框架资源中的任何内容。<br/>
再新建一个 HTML 文件它可能空空如也，把下面的代码复制粘贴到你的空文件中，并认真确认资源路径是否正确。
```html
<!DOCTYPE HTML>
<html size="s">
<head>
	<meta http-equiv="X-UA-Compatible" content="IE=edge,chrome=1" />
	<meta name="viewport" content="width=device-width, initial-scale=1, minimum-scale=1, maximum-scale=1" />
	<meta http-equiv="Content-Type" content="text/html; charset=UTF-8" />
	<link href="dist/main/css/base.css" rel="stylesheet" type="text/css" />
	<script type="text/javascript" src="dist/main/js/jq/jquery-1.9.1.js"></script>
	<script type="text/javascript" src="dist/main/js/jq/adapter-1.9.1.js"></script>
	<script type="text/javascript" src="dist/main/js/jcl.js"></script>
	<script type="text/javascript" src="dist/main/js/i18n/code.zh_CN.js"></script>
	<script type="text/javascript" src="dist/main/js/jcl-plugins.js"></script>
	<script type="text/javascript" src="dist/main/js/jcl-ui.js"></script>
	<script type="text/javascript" src="dist/main/js/jcl-jwc.js"></script>
	<script type="text/javascript" src="../asset/js/config.js"></script>
	<title>test</title>
</head>
<body jwcid="@Body" id="body">
	<!-- 代码从这里开始 -->
	<script type="text/javascript">
		Wade.setRatio();
		Wade.JWC.init(true);
	</script>
</body>
</html>
```
### 	2. 填入界面内容
```html
<!-- 代码从这里开始 -->
```
替换成下面代码
```html
<button class="e_button e_button-blue">你好，我是一个按钮</button>
```
### 	3. 看效果
首先要在你的 Visual Studio Code 中装一个 “Live Server” 插件，装好后会在软件右下角，看到如下图的 “图标 + 字段”，点击它~<br/>
![IE8IconPosition](static/img/doc/goLive.png) <br/>
<br/>
点击后，就能在浏览器中，看到你的界面效果了<br/>
![IE8IconPosition](../static/img/doc/blueButton.png) <br/>

这是一个纯静态的界面，接下来“下篇” 将会接着讲如果编写 Touch JWC 界面