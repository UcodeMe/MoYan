# 为何 $.attr 无法获取到对象属性


## 原因分析

$("#xxx").attr 是用于获取 DOM 对象的属性值，addDefault 不是 DOM 对象的属性，是组件本身的属性，


## 解决方法

直接在控制台输入组件名称，会显示它对应的是组件的哪个属性，通过如下方式可以获取：
![块间间距](../static/img/doc/getAttrFailed.png)
