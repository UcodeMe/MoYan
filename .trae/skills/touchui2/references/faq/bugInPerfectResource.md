# 结构没问题，界面依然乱 



## 决解方法
这种情况一般是由于样式表引用次序不正确导致的，样式表必须按照以下次序引入：

1. 框架样式，如：all.css、base.css、touchui.css 等。
2. 兼容样式：如：base-ie8.css 等。
3. 项目样式，如：project.css、crm.css 等。

## 示例

### 非 IE8 下
```html
<link href="touchui/main/css/base.css" rel="stylesheet" type="text/css" />
<link href="assets/css/project.css" rel="stylesheet" type="text/css" />
```

### IE8 下
```html
<link href="touchui/main/css/base-ie8.css" rel="stylesheet" type="text/css" />
<link href="assets/css/project.css" rel="stylesheet" type="text/css" />
```

### 两者均需兼容
```html
<!--[if gt IE 9]>< -->
<link rel="stylesheet" href="touchui/main/css/base.css" rel="stylesheet" type="text/css" />
<!-- ><![endif]-->
<!--[if lte IE 9]>
<link rel="stylesheet" href="touchui/main/css/base-ie8.css" rel="stylesheet" type="text/css" />
<![endif]-->
<link href="assets/css/project.css" rel="stylesheet" type="text/css" />
```