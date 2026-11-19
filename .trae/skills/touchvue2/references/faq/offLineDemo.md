# 脱机演示问题集

## 脱机演示目录说明
```javascript
|- dist // TouchUI 资源
|- demo // 演示
	|- myProject // 演示工程
		|- assets // 工程资源
		|- pages // 业务演示
		|- pc.html // PC 版外框
		|- phone.html // 手机版外框
		|- login.html // 登录
	|- myProject2
		|- assets
		|- pages
		|- pc.html
		|- phone.html
		|- login.html
```

## 如何运行脱机演示
1. 准备一个具备 httpServer 的服务环境，可利用 tomcat、apache、iis、nodejs 等工具或编辑器的相关插件，最好能支持 POST 请求方法，否则有些模拟 ajax 请求的演示会出现问题。
2. 在 http 协议下打开 pc.html 以演示 PC 端界面，如：``http://localhost:8080/myPorjectDemo/demo/myProject/pc.html``。
3. 使用 chrome 浏览器，打开开发者工具（F12），在 http 协议下打开 phone.html 以演示手机端界面。

## 演示中 ognl 绑定的数据是哪里来的
demo 均是静态演示，因此不会真的请求服务端的数据，为更加接近真实的数据效果，demo 均是由 demo 开发者自行在 js 里定义的常量，通常会放在 ``assets/js/data.js`` 中。

同理，ajax 请求的数据通常会放在 ``assets/json/xxx.json`` 中。

如果您使用的是前后端分离的开发方式，在开发时，将数据修改为异步请求的形式即可。

如果您使用的是前后端融合的开发方式，在开发时，需要在 java 中定义 ognl 绑定的数据，并在 page 文件中注册。
