# 手机界面自动缩放问题

## 问题描述
* 界面打开字体特别大或特别小
* 双击时界面比例发生变化
* 双指操作会改变界面比例

## 问题解决
在界面中加入如下代码在``<head>``标签之中即可。

```html
<meta name="viewport" content="width=device-width, initial-scale=1, minimum-scale=1, maximum-scale=1" />
```