# 客户端校验
设定一定的规则，批量判断表单控件合法性。

## HTML

```html
<input jwcid="@TextField"
	type="text" name="xxxx" value="ognl:info.SCHE_SUBJECT"
	desc="日程主题"
	nullable="no"
	maxsize="100"
	minsize="10"
	equsize="10"
	datatype="text"
/>
```

## 属性

| 参数     | 说明                                                         |
| -------- | ------------------------------------------------------------ |
| nullable | 是否必填，为 no 表示必填，默认为非必填                       |
| maxsize  | 最大输入数                                                   |
| minsize  | 最小输入数                                                   |
| max      | 最大值（仅当 datatype 为数值类型或 date 时有效）                 |
| min      | 最小值（仅当 datatype 为数值类型或 date 时有效）                 |
| maxName  | 最大值元素id（仅当 datatype 为数值类型或 date 时有效）           |
| minName  | 最小值元素id（仅当 datatype 为数值类型或 date 时有效）           |
| equsize  | 定值                                                         |
| datatype | 校验类型，即判断 value 值是否为“文本、数字”等，对照表如下：<table><tr><th>值</th><th>描述</th></tr><tr><td>text</td><td>文本</td></tr><tr><td>numeric</td><td>数字</td></tr><tr><td>integer</td><td>整数</td></tr><tr><td>pinteger</td><td>正整数（不包括0）</td></tr><tr><td>ninteger</td><td>负整数</td></tr><tr><td>nature</td><td>自然数（包括 0）</td></tr><tr><td>date</td><td>日期</td></tr><tr><td>email</td><td>EMAIL地址</td></tr><tr><td>file</td><td>文件</td></tr><tr><td>mbphone</td><td>手机号码</td></tr><tr><td>pspt</td><td>身份证号码</td></tr><tr><td>ip</td><td>IP 地址</td></tr><tr><td>numchar</td><td>只能为数字或字母</td></tr><tr><td>numuchar</td><td>只能为数字或小写字母</td></tr><tr><td>numulchar</td><td>只能为数字或大写字母</td></tr></table>|

校验方法 

| 方法                                        | 描述                                                         |
| ------------------------------------------- | ------------------------------------------------------------ |
| $.validate.verifyAll(el)                | 校验某 HTML 节点下的所有的表单元素，el 为原生 DOM 对象或元素 id，返回 true\|false，如果校验的表单元素是隐藏的，则可以通过设置元素的 x-visible-element 属性来指定用于校验信息 tip 定位的元素 id |
| $.validate.verifyBox(el, boxName, info) | 校验某 HTML 节点下的所有 name 为 boxName 的单选或复选框是否有选中项，el 为原生 DOM 对象或元素 id，返回 true\|false，如果校验的表单元素是隐藏的，则可以通过设置元素的 x-visible-element 属性来指定用于校验信息 tip 定位的元素 id |
| $.validate.verifyField(el)              | 校验某单个表单元素，el 可以为元素 id 或者元素的 DOM 对象，返回 true\|false |
| $.validate.alerter.one(el, msg, color);     | 显示校验提示信息，其中：el 为原生 DOM 对象或元素 id，msg 为提示信息，color 为可选参数，指定提示信息的颜色样式，默认为red |

注意：
配置校验属性，必须带有 ``desc="..."`` 参数，不配置将不会对当前的表单控件校验; ``disabled`` 的对象也不会对当前的表单控件校验；
``verifyAll`` 只做校验，没有载入效果，所以 ``verifyAll`` 是使用场景为自定义校验时可以组合使用;
``verifyBox`` 方法用来校验复选框如果校验的表单元素是隐藏的，则可以使用 ``$.validate.verifyField`` 和 ``$.validate.alerter.one`` 方法结合来在别的可见元素上弹出 tip 提示信息。				