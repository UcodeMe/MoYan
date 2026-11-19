# 全局插入内容（@BodyContent）
全局插入是指在 ``<body></body>`` 中最前或最后插入一段代码，适合封装组件时插入 @Popup 或 @Dialog 等需要在滚动条之外的控件。
注：仅 Java 模式支持。

## HTML

```html
<div jwcid="myContent@BodyContent">
	...
</div>
```

注：“myContent” 的名称是自定义的，必须存在，用于区分多个 ``@BodyContent`` 即可，否则可能只渲染其中一个。

## 属性

| 数名    | 参数类型         | 是否必须 | 默认值 | 描述                                                   |
| ------- | ---------------- | -------- | ------ | ------------------------------------------------------ |
| where      | java.lang.String | 否       | beforeEnd     | 代码插入的位置，支持 beforeEnd | afterBegin |