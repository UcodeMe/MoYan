# 基本数据类型校验 

判断数据类型的方法集合。

## 属性

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