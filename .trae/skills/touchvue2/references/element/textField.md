# 输入框（&lt;input /&gt; / @TextField）

输入框是最基本的表单元素，用于手动输入数据，给提交的数据提供来源。

## HTML
<a href="#/example/element/textField" target="_blank">查看演示</a>

```html
<input name="CUST_NAME" jwcid="@TextField" onenterAction="alert('enterAction')" />
```

## 属性

| 参数名      | 参数类型         | 是否必须 | 默认值 | 描述                    |
| ----------- | ---------------- | -------- | ------ | ----------------------- |
| type        | java.lang.String | 否       | text   | 输入框类型，默认为 text |
| value       | java.lang.String | 是       | 无     | 输入框值                |
| name        | java.lang.String | 是       | 无     | 输入框名称              |
| enterAction | java.lang.String | 否       | 无     | 输入框回车事件执行脚本  |
| disabled    | boolean          | 否       | false  | 是否禁用该输入框        |
| readOnly    | boolean          | 否       | false  | 该输入框是否只读        |

## JS API

绑定 enterAction 事件

```js
$("#CUST_NAME").enterAction(function(){
	alert('enterAction');
});
```

设置默认值

```js
$("#CUST_NAME").val("TouchUI")
```

## 样式

| 参数名    | 值                                     | 默认值    | 描述     |演示     |
| --------- | -------------------------------------- | --------- | -------- |-------- |
| e_input-s | e_input-s<br />e_input-m<br />e_input-l        | e_input-m | 大小设置 ||
| e_input-r | e_input-left<br />e_input-right<br />e_input-r | 无        | 圆角设置 |<a href="#/example/element/textField-r" target="_blank">查看</a>|

