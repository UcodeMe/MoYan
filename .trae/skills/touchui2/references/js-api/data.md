# Data

DataMap 与 DatasetList 数据操作方法集合。

## 基本数据类型校验 

| 方法                   | 描述                                                         |
| ---------------------- | ------------------------------------------------------------ |
| $.isNumber(value)      | 判断 value 是否为数值类型                                    |
| $.isNumeric(value)     | 判断是否数值类型,与 isNumber 方法的区别是：字符串数字（比如"43.3"），isNumeric 判断是 true，但 isNumber 判断是 false |
| $.isString(value)      | 判断 value 是否为字符串                                      |
| $.isBoolean(value)     | 判断 value 是否为布尔值                                      |
| $.isElement(value)     | 判断 value 是否是一个 Dom 元素                               |
| $.isFunction(value)    | 判断 value 是否为函数                                        |
| $.isArray(value)       | 判断 value 是否为数组                                        |
| $.isObject(value)      | 判断 value 是否为 object                                     |
| $.isEmptyObject(value) | 判断 value 是否为没有任何属性的 object                       |
| $.isPlainObject(value) | 判断 value 是否为 json 对象的 object（无任何 prototype 方法） |

## $.DataMap 和 $.DatasetList 

和原 Wade.DataMap 和 Wade.DatasetList 用法基本一致 					

新增支持不带 new 的构造方式

```js
//例如
var data=$.DataMap(obj|str);
var ds=$.DatasetList(array|str);
```

支持将数据绑定到 html 模板

在后面的“DOM 模板”节介绍 					

$.DatasetList 常用 API

| 属性/方法                        | 描述                                                         |
| -------------------------------- | ------------------------------------------------------------ |
| length（属性）                   | 得到所包含的数据行数长度                                     |
| get(index[int]，key[string])     | 获取一行数据,如果设置了key，并且获取的数据是 Wade.DataMap 格式的数据，则返回其中名称为 key 的项 |
| each(fn[function],scope[object]) | 遍历数据                                                     |
| add(o[object])                   | 增加一行数据                                                 |
| remove(o[object])                | 删除数据列中的一列                                           |
| removeAt(index[int])             | 删除数据列中的一列                                           |
| indexOf(o[object])               | 获取某列数据的行标                                           |
| getCount()                       | 获取所包含的数据行记录数                                     |
| toString(false\|true)            | 生成字符串，当参数为 true 时生成旧的格式字符串，当参数为 false 或不填时生成 json 语义的字符串 |
| bind(selector,displayStyle)      | 将数据绑定到指定的DOM结构上                                  |

$.DataMap 常用 API

| 属性/方法                              | 描述                                                         |
| -------------------------------------- | ------------------------------------------------------------ |
| length（属性）                         | 得到所包含的数据行数长度                                     |
| get(name[string],defaultValue[string]) | 获取项数据，如果值为空并且设置了默认值则返回默认值           |
| each(fn[function],scope[object])       | 遍历所有数据项                                               |
| eachKey(fn[function],scope[object])    | 遍历所有数据项名称                                           |
| put(name[string],value[object])        | 增加一项数据                                                 |
| remove(o[object])                      | 删除数据中的一项                                             |
| removeKey(name[string])                | 根据项名称来删除数据中的一项                                 |
| getCount()                             | 获取所包含的数据项总数                                       |
| contains(o[object])                    | 判断是否包含指定的数据项                                     |
| containsKey(name[string])              | 判断是否包含指定名称的数据项                                 |
| clear()                                | 清空所有数据项                                               |
| first()                                | 获取第一项数据                                               |
| last()                                 | 获取最后一项数据                                             |
| fillFormElements(form[domelement])     | 将对象包含的值填充到和项名称匹配的表单元素中                 |
| toString(false\|true)                  | 生成字符串，当参数为 true 时生成旧的格式字符串，当参数为 false 或不填时生成 json 语义的字符串 |
| bind(selector,displayStyle)            | 将数据绑定到指定的 DOM 结构上                                |

$.JavaScript 数据结构的使用示例

```js
//根据字符串来创建 DataMap对象
var d4=new Wade.DataMap('{KEY33=["value33"], KEY32=["value32"], KEY31=["value31"]}');

//获取对象中的数据项
var v=d4.get('KEY33','defaultValue');

//增加对象的数据项
d4.put('KEY34','value34');

//根据项名删除对象中的数据项
d4.removeKey('KEY34');

//遍历对象中的数据项
d4.each(function(item,index,totalcount){
	alert('共' +totalcount +'项,这是第' + (index+1) + '项,值为:' + item);
});
//将对象输出成字符串格式
d4.toString();

//根据字符串来创建 DatasetList对象
var ds4=new Wade.DatasetList('[{KEY11=["value11"], KEY12=["value12"], KEY13=["value13"]}, {KEY22=[""], KEY23=["value23"], KEY21=["value21"]}]');

//在这个例子中，DatasetList对象里嵌套了DataMap对象
//根据行标删除多列数据集中的一列
ds4.removeAt(0);

//增加多列数据集的列
var newd=new Wade.DataMap();
newd.put('KEY33','value33');
newd.put('KEY43','value43');
ds4.add(newd);

//获取多列数据集中的一列
ds4.get(0);

//获取列中的项值 ds4.get(0,'KEY33')
//直接删除多列数据集中的列数据
d4.remove(newd);

//遍历多列数据集中的数据列
ds4.each(function(item,index,totalcount){
	alert('共' +totalcount +'列,这是第' + (index+1) + '列,输出为字符串是:' + item);
});

//输出多列数据集成字符串格式
ds4.toString();
```

