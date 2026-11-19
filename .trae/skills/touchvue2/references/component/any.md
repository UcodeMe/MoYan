# 任意组件（@Any）

任意组件，被誉为“瑞士军刀”。所有元素上都可使用，一般多用于需要支持 ognl 的元素，如 ``<div jwcid="@Any" data="ognl:data" ...>``

## HTML

```HTML
<img
	jwcid="@Any"
	src="ognl:info.SRC"
	title="ognl:info.TITLE"
	width="ognl:info.WIDTH"
/>
```

## 属性

| 参数名  | 参数类型         | 是否必须 | 默认值 | 描述                 |
| ------- | ---------------- | -------- | ------ | -------------------- |
| element | java.lang.String | 否       | 空     | 元素名，一般可以不写 |

