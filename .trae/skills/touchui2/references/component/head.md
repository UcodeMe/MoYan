# @Head
将 HTML 文档中的 ``<head>...</head>`` 部分封装成组件，以便工程进行批量维护。


## HTML
```html
<head jwcid="@Head" includeJs="'./assets/js/config.js', './assets/js/data.js'">
	<link href="./assets/css/project.css" rel="stylesheet" type="text/css" />
</head>
```

## 属性

| 参数名              | 参数类型	|是否必须                  | 默认值 | 描述             |
| ------------------- | ------------------- | ------ | ---------------- |-------- |
| simpleJs      | boolean| 否      | false     | 为 true，则只引入框架基础JS，否则还加载 tapestry相关JS||
| noneJs      | boolean| 否      | false     | 为 true，则不加载任何框架 JS，否则还加载框架 JS||
| noneCss      | boolean| 否      | false     | 为 true，则不加载任何框架 CSS，否则还加载框架 CSS||
| charset      | java.lang.String| 否      | utf-8	     | 生成 HTML head 里的 ``<meta http-equiv="Content-Type" content="text/html; charset="utf-8" />`` ||
| includeJs      | java.lang.String| 否      | 无     | 需要引入的 JS 文件，多个则以“,”号分隔 ||
| touchuiSkin      | java.lang.String| 否      | 无     | 设置当前页面的touchuiSkin样式文件名 ||
| touchuiGap      | java.lang.String| 否      | 无     | 设置当前页面的touchuiGap间距设置子样式文件名 ||

