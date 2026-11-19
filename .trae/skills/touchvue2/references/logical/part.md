# 局部刷新（@Part）
局部刷新能组件用于通过 ajax 的 api 来刷新界面上的一块区域 

## HTML


```html
<div id="QueryPart" jwcid="@Part" element="div">
	...
</div>
```

## 属性

| 数名    | 参数类型         | 是否必须 | 默认值 | 描述                                                   |
| ------- | ---------------- | -------- | ------ | ------------------------------------------------------ |
| id      | java.lang.String | 是       | 无     | 局部刷新组件的标识，也是通过 ajax 刷新时指定 part 的值 |
| element | java.lang.String | 是       | 无     | 局部刷新组件输出的 html 标签，默认为 div               |