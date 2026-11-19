# 条件判断（@Conditional）
条件判断组件用于根据不同的条件来决定是否要输出条件判断组件区域中的内容 

## HTML
<a href="#/example/component/conditional" target="_blank">查看演示</a>

```html
<div jwcid="@Conditional" condition="ognl:info.status == 1" element="div">
	...
</div>
```

## 属性

| 参数名    | 参数类型         | 是否必须 | 默认值 | 描述                                                         |演示 |
| --------- | ---------------- | -------- | ------ | ------------------------------------------------------------ |----- |
| condition | boolean          | 是       | 无     | 条件区域的判断条件                                           |<a href="#/example/component/conditional-expression" target="_blank">查看</a>|
| element   | java.lang.String | 否       | 无     | 判断输出的 HTML 元素类型，默认为不输出 Conditional 组件的元素行 |<a href="#/example/component/conditional-element" target="_blank">查看</a>|

