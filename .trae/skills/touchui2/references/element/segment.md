# 分段器（e_segment / @Segment）
分段器是基本的表单元素之一，用于从一组数据（2-5条）中选取其中一条，给提交的数据提供来源。 

## JWC 方式
<a href="#/example/element/segment" target="_blank">查看演示</a>
```html
<span name="mySegment" jwcid="@Segment" source="ognl:levels" value="0"></span>
```

## JS 方式

html
```html
<span class="e_segment">
	<span idx="0" val="0">低</span>
	<span idx="1" val="1">中</span>
	<span idx="2" val="2">高</span>
	<input type="hidden" name="mySegment" id="mySegment" nullable="no" desc="级别" />
</span>
```
js
```js
window["mySegment"] = new Wade.Segment("mySegment",{
	disabled:false
});
```


## 属性

| 参数名       | 参数类型         | 是否必须 | 默认值 | 描述                                           |
| ------------ | ---------------- | -------- | ------ | ---------------------------------------------- |
| name         | java.lang.String | 是       | 无     | 组件实例名，同一个页面中不能重复               |
| source       | java.lang.Object | 是       | 无     | 绑定的数据源，必须实现 java.util.List 接口     |
| textField    | java.lang.String | 否       | 无     | 文本字段                                       |
| valueField   | java.lang.String | 否       | 无     | 值字段                                         |
| titleField   | java.lang.String | 否       | 无     | 标题字段                                       |
| value        | java.lang.String | 否       | 无     | 当前值（必须匹配分段器组件数据源中的值才有效） |
| class        | java.lang.String | 否       | 无     | 自定义样式名                                   |
| style        | java.lang.String | 否       | 无     | 自定义样式                                     |
| disabled     | boolean          | 否       | 无     | 是否禁用                                       |

## 样式
| 参数名     | 值           |  默认值 | 描述     |演示     |
| ---------- | ------------ | ------ | -------- |------ |
| e_segment-gd |e_segment-gd | 无 | 渐变|<a href="#/example/element/segment-gd" target="_blank">查看</a>|

## JS API

绑定分段器组件 change 事件

```js
$("#mySegment").change(function(){
	alert(this.value); // this.value 获取分段器组件当前值
});
```

获取分段器组件的当前值

```js
$("#mySegment").val();
```

设置分段器组件的值（必须匹配分段器组件数据源中的值才有效）

```js
$("#mySegment").val(value);
```

支持点击还原

<a href="#/example/element/segment-change" target="_blank">查看</a>

全局设置 ``$.ENABLE_SEGMENT_RESTORE = true``后，所有的 Segment 默认可以点击还原，对单个 Segment 设置 ``nullable="no"``，则不会点击还原。

```js
$.ENABLE_SEGMENT_RESTORE = true
```
