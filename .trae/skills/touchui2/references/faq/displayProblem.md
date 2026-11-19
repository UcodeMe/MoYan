# 关于隐藏的问题

## 问题现象

使用 ``$("#id").css("display","block")`` 方法让隐藏的元素显示后，发现布局出现了问题
![IE8IconPosition](../static/img/doc/display3.png) <br/>


## 解决方案

使用 ``$("#id").show()`` 或 ``$("#id").css("display","")``的方法来让元素显示

## 正确使用后的效果

元素正常显示，没有出现问题
![IE8IconPosition](../static/img/doc/display1.png) <br/>

## 原因

这是因为我们很多元素的默认属性不是 ``display:block``，而是 ``inline-block`` 或 ``flex``、``inline-table`` 等。当时用 ``display:block`` 来显示元素时，就会使布局出现问题。
因此框架中不推荐使用 ``$("#id").css("display","block")`` 的方法

