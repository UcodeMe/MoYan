# 多行输入框（@Textarea）
多行输入框是基本的表单元素之一，用于手动输入多行数据，给提交的数据提供来源。 

## JWC 方式
普通模式 

<a href="#/example/element/textarea" target="_blank">[查看演示]</a>

```html
<textarea name="REMARK" jwcid="@TextArea" onenterAction="alert('enterAction')" /></textarea>
```

高度自适应模式（div模拟）


<a href="#/example/element/textarea-div" target="_blank">[查看演示]</a>

```html
<div contenteditable="true"></div>
```

## HTML 方式
```html
<textarea class="e_textarea"></textarea>
```

## 属性

| 参数名      | 参数类型         | 是否必须 | 默认值 | 描述                    |
| ----------- | ---------------- | -------- | ------ | ----------------------- |
| type        | java.lang.String | 否       | text   | 输入框类型，默认为 text |
| value       | java.lang.String | 是       | 无     | 输入框值                |
| name        | java.lang.String | 是       | 无     | 输入框名称              |
| disabled    | boolean          | 否       | false  | 是否禁用该输入框        |
| readonly    | boolean          | 否       | false  | 该输入框是否只读        |

## JS API

设置默认值

```js
$("#REMARK").val("TouchUI")
```

## 样式

| 参数名     | 值           |  默认值 | 描述     |
| ---------- | ------------ | ------ | -------- |
| e_textarea-s |e_textarea-s<br />e_textarea-m<br />e_textarea-l | e_textarea-m | 大小设置 |
| e_textarea-r |e_textarea-r |  无     | 圆角设置 |
| e_textarea-row-2 |e_textarea-row-2 ~ e_textarea-row-10 | e_textarea-row-1 | 高度设置 |

