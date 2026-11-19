# 遍历（@Foreach）
遍历组件用于结合数据集生成数据列表或HTML中的数据集结构


# HTML
<a href="#/example/logical/foreach" target="_blank">查看演示</a>

```html
<tr jwcid="@Foreach" source="ognl:custs" value="ognl:listItem" index="ognl:rowIndex" id="ognl:'tr' + rowIndex" element="tr">
	...
</tr>
```
如果是使用的客户端 jwc 方式，foreach 的子项要定义，比如 listItem
```js
var listItem
```

## 属性

| 参数名   | 参数类型                                               | 是否必须 | 默认值 | 描述                                   |默认值 |
| -------- | ------------------------------------------------------ | -------- | ------ | -------------------------------------- |-- |
| source   | java.lang.Object，必须是 java.util.Iterator 的实现类。 | 是       | 无     | 要遍历的数据集                         ||
| element  | java.lang.String                                       | 否       | 无     | 循环输出的 HTML 元素类型，如 li、tr 等 |<a href="#/example/component/foreach-element" target="_blank">查看演示</a>|
| value    | java.lang.Object                                       | 否       | 无     | 遍历数据集时的每项数据对象引用         ||
| rowIndex | int                                                    | 否       | 无     | 遍历数据集时的下标引用                 ||

## 注意事项
在使用前台 JWC 方式实现时，“item” 是关键词，不能使用。