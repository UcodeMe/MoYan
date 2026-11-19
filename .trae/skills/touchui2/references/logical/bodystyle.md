# 全局插入 CSS（@BodyStyle）
全局插入是指在 ``<body></body>`` 中最前或最后插入一段 CSS。
注：仅 Java 模式支持。

## HTML

```html
<link jwcid="@BodyStyle" href="mycss.css" />
```

## 属性

| 数名    | 参数类型         | 是否必须 | 默认值 | 描述                                                   |
| ------- | ---------------- | -------- | ------ | ------------------------------------------------------ |
| href      | java.lang.String | 否       | 无     | CSS 文件路径 |
| where      | java.lang.String | 否       | beforeEnd     | 代码插入的位置，支持 beforeEnd | afterBegin |