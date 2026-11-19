# IP 地址输入框（@IpAddressField）
IP 地址输入框是扩展的表单元素之一，主要用于按指定格式输入 IP 地址。 

## HTML
<a href="#/example/element/IpAddressField" target="_blank">查看演示</a>
```html
<input name="IPAddr" jwcid="@IpAddressField" />
```

## 属性

| 参数名   | 参数类型         | 是否必须 | 默认值 | 描述                                                         |
| -------- | ---------------- | -------- | ------ | ------------------------------------------------------------ |
| id       | java.lang.String | 否       | 无     | 组件 id，同一个页面里不能重复，如果不设置则默认取，name 属性值 |
| name     | java.lang.String | 是       | 无     | 组件实例名                                                   |
| value    | java.lang.String | 否       | 无     | 当前值                                                       |
| disabled | boolean          | 否       | false  | 是否禁用                                                     |
| readonly | boolean          | 否       | false  | 是否只读                                                     |
| class    | java.lang.String | 否       | false  | 样式表                                                       |
| style    | java.lang.String | 否       | false  | 样式                                                         |

## JS API

设置默认值

```JS
$("#myIpadressField").val("192.168.1.1")
```

## 样式

日期输入框继承了混合框的能力，支持大小与圆角的设置，设置方法请参考<a href="#/doc/element/mix" target="_blank">组合框</a>一章。 