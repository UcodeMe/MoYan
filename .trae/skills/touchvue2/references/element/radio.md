# 单选框（e_radio / @radio）
单选框是基本的表单元素之一，用于从一组平铺陈列的数据中选取一条，给提交的数据提供来源。

## HTML
<a href="#/example/element/radio" target="_blank">查看演示</a>
```html
<input
	jwcid="@Radio"
	name="MyRadio"
	disabled="true"
	value="ognl:info.value"
	checked="ognl:info.checked == 'true'"
	/>
```

## 属性

| 参数名   | 参数类型         | 是否必须 | 默认值 | 描述                                                         |
| -------- | ---------------- | -------- | ------ | ------------------------------------------------------------ |
| name     | java.lang.String | 是       | 无     | radio 元素 id 和 name 属性的值                                                   |
| value    | java.lang.String | 否       | 无     | radio 元素 value 属性的值                                                       |
| checked | boolean          | 否       | false  | radio 元素 checked 属性的值                                                     |
| disabled | boolean          | 否       | false  | radio 元素 disabled 属性的值                                                     |

# 样式

| 参数名     | 值           |  默认值 | 描述     |演示     |
| ---------- | ------------ | ------ | -------- |------- |
| e_radio-white |e_radio-white |  无     | 反白 |<a href="#/example/element/radio-white" target="_blank">查看</a>|