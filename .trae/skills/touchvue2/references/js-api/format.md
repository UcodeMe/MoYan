# 格式化工具
数据格式的判断与转换。

## 属性

| 方法                                           | 描述                                                         |
| ---------------------------------------------- | ------------------------------------------------------------ |
| $.format.undef(value)                          | IE浏览器                                                     |
| $.format.substr(value,start,length)            | 从字符串 value 中的 start 下标处的字符开始截取 length 长度   |
| $.format.lowercase(value)                      | 将字符串转换为小写                                           |
| $.format.uppercase(value)                      | 将字符串转换为大写                                           |
| $.format.toUtf8(value)                         | 转换为 utf-8 编码格式                                        |
| $.format.toGbk(value)                          | 转换为 gbk 编码格式                                          |
| $.format.nl2br(value)                          | 将字符串中的换行符转换为 html 换行                           |
| $.format.htmlEncode(value)                     | 将 html 字符串转换为可以在 html 文档里显示的 html 字符串(&转换为&，>转换为>); |
| $.format.htmlDecode(value)                     | 将可以在 html 文档里显示的 html 字符串转化为 html 字符串（与 htmlEncode 相反） |
| $.format.ellipsis(value,len,word(true\|false)) | 按 len 长度截取字符串，如果超过 len 长度，则显示省略号(...)，当 word 为 true 时，自动查找 len 附近的结束标点符号进行截取 |
| $.format.fileSize(size)                        | 将字符串数字转换为文件大小值，size 单位按 bytes 计算，例如 $.format.fileSize(24) 返回“24bytes”，$.format.fileSize(1024) 返回“1KB” |
| $.format.math(v,a)                             | 对字符串数字进行四则运算，例如 $.format.math("10","*10") 返回 100 |
| $.format.number(v,format)                      | 使用 format 格式来格式化字符串数字 										 										 										 											 												 													 														取整数部分 														0 - (123456) 													 													 														取整数部分 														0 - (123456) 													 													 														取 4 位小数 														0.0000 - (123456.7890) 													 													 														取金额格式 														0,000 - (123,456) 													 													 														取金额格式并 2 位小数 														0,000.00 - (123,456.78) 													 													 														取简短金额格式 														0,0.00 - (123,456.78) |
| $.format.px2rem(px)                            | px转rem，示例：$.format.px2rem(300)，返回保留2位小数的数值   |
| $.foramt.rem2px(rem)                           | rem转px ，示例：$.format.rem2px(3)，返回保留2位小数的数值    |
| 字符串转日期 String.toDate(format)             | 示例："2016-03-01".toDate("yyyy-MM-dd")  format为日期格式，必须和字符串的值相匹配，返回Date类型值 |

注意：c_popupBg 和 c_popupWrapper 的 id 均需要继承 c_popup 的前缀，c_popupItem 的 id 则可以任意命名

## 日期格式化

format为格式，返回格式化后的日期时间字符串字符串 					

```js
Date.format(format);
//例如： (new Date()).format("yyyy-MM-dd HH:mm:ss")
```