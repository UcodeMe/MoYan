# 星级（e_star/@Star）
以“点亮星星”的方式呈现或选择目标等级。 

## HTML
<a href="#/example/element/star" target="_blank">查看演示</a>
```html
<span class="e_star">
	<span></span>
	<span></span>
	<span></span>
	<span></span>
	<span></span>
</span>
```
可编辑星级

```html
<span name="mystar" class="e_star e_star-edit" jwcid="@Star" value="3" readonly="true" />
```
## 属性

| 参数名   | 参数类型         | 是否必须 | 默认值 | 描述                             |
| -------- | ---------------- | -------- | ------ | -------------------------------- |
| name     | java.lang.String | 是       | 无     | 组件实例名，同一个页面里不能重复 |
| class    | java.lang.String | 否       | 无     | 自定义样式名                     |
| style    | java.lang.String | 否       | 无     | 自定义样式                       |
| value    | java.lang.String | 否       | 无     | 当前值（只能为 0 ~ 5 的整数）    |
| readonly | boolean          | 否       | 无     | 是否只读，只读状态不能选择       |
| disabled | boolean          | 否       | 无     | 是否禁用                        |

## JS API

绑定 change 事件

```js
$("#myStar").change(function(){
	alert(this.value); // this.value 获取 Star 组件当前值
});
```

获取 Star 的当前值

```js
$("#myStar").val();
```

设置 Star 的值（只能为 0 ~ 5 的整数）

```js
$("#myStar").val(value);
```

设置只读

```js
$("#myStar").attr("readonly", true);
```


## 样式

| 参数名           | 值   | 默认值   | 描述  | 演示   |
|-------------| -------- | ------------ |---------- |-------- |
| e_star-3         | e_star-0<br />e_star-1<br />e_star-2<br />e_star-3<br />e_star-4<br />e_star-5<br />e_star-gold(金色星星)<br />e_star-diamond(钻石) | e_star-0 | 级别设置     |<a href="#/example/element/star-star" target="_blank">查看</a>|
| e_star-s         | e_star-s<br />e_star-m<br />e_star-l<br />e_star-xl                      | e_star-m | 大小设置     ||