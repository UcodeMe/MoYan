# 数值调节（@IncreaseReduce）
数值调节是扩展的表单元素之一，按照一定规律增/减数值输入框中的数值，给提交的数据提供来源。 

## JWC 方式
<a href="#/example/element/increaseReduce" target="_blank">查看演示</a>
```html
<span name="myIncreaseReduce" jwcid="@IncreaseReduce" value="5"></span>
```

## JS 方式

html

```html
<span class="e_group">
      <span class="e_groupLeft"><span class="e_ico-reduce"></span></span>
      <span class="e_groupRight"><span class="e_ico-add"></span></span>
      <span class="e_groupMain e_center">
            <input type="text" name="myIncreaseReduce">
      </span>
</span>
```

js

```js
window["myIncreaseReduce"] = new Wade.IncreaseReduce("myIncreaseReduce", {
	disabled:false,
	step:5
});
```

## 属性

| 参数名    | 参数类型         | 是否必须 | 默认值 | 描述                             |演示     |
| --------- | ---------------- | -------- | ------ | -------------------------------- |------ | 
| name      | java.lang.String | 是       | 无     | 组件实例名，同一个页面里不能重复 ||
| class     | java.lang.Object | 否       | 无     | 自定义样式名                     ||
| style     | java.lang.String | 否       | 无     | 自定义样式                       ||
| maxlength | int              | 否       | 10     | 最长位数                         |<a href="#/example/element/increaseReduce-maxlength" target="_blank">查看</a>|
| step      | int              | 否       | 1      | 增减的步长大小                   |<a href="#/example/element/increaseReduce-setp" target="_blank">查看</a>|
| value     | java.lang.String | 否       | 无     | 当前值                           ||
| disabled  | boolean          | 否       | false  | 是否禁用                         ||
|readonly|boolean|否|false|是否只读||

## JS API

设置默认值

```js
$("#myIncreaseReduce").val("5")
```