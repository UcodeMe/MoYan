# 富文本编辑器（ckeditor/@Editor）
富文本编辑器 

## JWC 方式
<a href="#/example/component/editor" target="_blank">查看演示</a>

```html
<textarea name="INFO_CONTENT" jwcid="@Editor" bbcode="true" />
```

## JS 方式

html
```html
<textarea id="INFO_CONTENT" name="INFO_CONTENT" cols="80" rows="10"></textarea>
```

js

```js
CKEDITOR.replace("INFO_CONTENT",{"extraPlugins":"bbcode"});
```

## 属性


| 参数名  | 参数类型         | 是否必须 | 默认值   | 描述                                                    |
| ------- | ---------------- | -------- | -------- | ------------------------------------------------------- |
| name    | java.lang.String | 是       | 无       | 元素 id 和 name 属性的值，也是提交表单数据时的字段名字  |
| value   | java.lang.String | 否       | 无       | 设置编辑器默认显示的内容                                |
| width   | int              | 否       | 无       | 编辑器宽度，单位为 px                                    |
| height  | int              | 否       | 无       | 编辑器高度，单位为 px                                    |
| toolbar | java.lang.String | 否       | standard | 设置编辑器工具栏样式，可选 full \| standard \| text \| simple |
| bbcode  | boolean          | 否       | false    | 设置是否使用UBB编码模式，默认为 false                    |

