# c_list-fixWrapper

## 参数说明

在 html 中连续几个内联标签或表单元素标签的换行在浏览器会被解释为一个空格。

``</li>`` 是一个块元素，由于在框架中 ``<li>`` 标签 加了 ``display：inline-block`` 属性变成了行内元素，代码换行则会出现一个空格，这个空格是有宽度的，框架中我们对此宽度做了处理。

但是经常有 JS 生成代码时，``</li>`` 和下一个 ``<li>`` 之间是没有插入换行的，如同例二，此时就不存在会出现空格了，所以需要此样式的支持，加上 ``c_list-fixWrapper`` 可以消除框架样式对其之间的宽度的处理。

**c_list-fixWrapSpace**

在框架中，是通过 margin 负值来去除了此间距，加上 ``c_list-fixWrapSpace``，JS 生成的如果没有加入 \r\n，则不会有换行符，没有换行符又有 margin 负值，导致的结果是两个 ``<li>`` 会重叠。

## 例一

html
```html
<div>
	<a href="">我右边有空格</a>
	<a href="">我两边都有空格</a>
	<a href="">我左边有空格</a>
</div>
```
输出效果：

![IE8IconPosition](../static/img/doc/space.png) <br/>

## 例二

html
```html
<div>
	<a href="">我右边没空格</a><a href="">我两边没空格</a><a href="">我左边没空格</a>
</div>
```
输出效果：

![IE8IconPosition](../static/img/doc/nospace.png) <br/>


