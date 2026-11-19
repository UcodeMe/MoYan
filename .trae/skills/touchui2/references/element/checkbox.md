# 复选框（e_checkbox / @checkbox）
复选框是基本的表单元素之一，用于从一组平铺陈列的数据中选取多条，给提交的数据提供来源。 

## HTML
<a href="#/example/element/checkbox" target="_blank">查看演示</a>
```html
<input
	jwcid="@Checkbox"
	name="MyCheckbox"
	disabled="true"
	value="ognl:info.value"
	checked="ognl:info.checked == 'true'"
	/>
```

## 属性

| 参数名   | 参数类型         | 是否必须 | 默认值 | 描述                                                         |
| -------- | ---------------- | -------- | ------ | ------------------------------------------------------------ |
| name     | java.lang.String | 是       | 无     | checkbox 元素 id 和 name 属性的值                                                   |
| value    | java.lang.String | 否       | 无     | checkbox 元素 value 属性的值                                                       |
| checked | boolean          | 否       | false  | checkbox 元素 checked 属性的值                                                     |
| disabled | boolean          | 否       | false  | checkbox 元素 disabled 属性的值                                                     |

## 样式

| 参数名     | 值           |  默认值 | 描述     |演示     |
| ---------- | ------------ | ------ | -------- |-------- |
| e_checkbox-white |e_checkbox-white |  无     | 反白 |<a href="#/example/element/checkbox-white" target="_blank">查看</a>|