# UI 设计-常用效果教程

## 一、@Popup 的控制
打开<a href="#/example/component/popup" target="_blank"> popup 实例</a>
看到实例中如下代码，其中 “myPopup” 就是你 @Popup 控件的 name 值, "myPopup_item" 是指你需要弹出的层 name 值；
```html
<button type="button" ontap="showPopup('myPopup','myPopup_item')" class="e_button-red">打开弹窗</button>
```
所以你只要把这2个 name 值，改成你便于代码阅读的命名就行。


```html
<div jwcid="@Popup" name="myPopup">
	<div class="c_popupGroup">
		<div class="c_popupItem" id="myPopup_item">
			<div class="c_header">
				<div class="back" ontap="backPopup(this)">标题君</div>
			</div>
			<div class="c_scroll c_scroll-float c_scroll-header"><!--  -->
				<!-- 弹出层的内容从这里开始 -->
			</div>
		</div>
	</div>
</div>
```
上面的代码：
+ ``c_header`` 这部分不是 ``@Popup`` 控件中的，是为放常规的退出事件，可以按需调整；
+ ``c_scroll`` 这部分不是 ``@Popup`` 控件中的，是为配合 ``c_header`` 部分，给下面内容超出部分加上滚动条，而添加。

已经提供了以下 @Popup 的实例<br/>
![IE8IconPosition](../static/img/doc/popup.png)
### 注意
+ 其中“多层” @Popup ，如果是需要弹的界面，可以是同一宽度，就可以选择用“多层”的方式；
+ 如果所需弹出的界面，需要多种宽度界面，才能满足要求，就看实例中的“多个”；

## 二、显示/隐藏
+ 显示 ``show()``;
+ 隐藏 ``hide()``;
+ 隐藏/显示 ``toggle()``;
<br/>
<br/>
已经提供了以下显示隐藏的实例
<a href="#/example/effect/showHide" target="_blank">显示/隐藏实例</a>

![IE8IconPosition](../static/img/doc/showHide.png)
### 注意
+ 双引号里面不能再使用双引号；


## 三、控制 className
+ ``addClass()`` 添加 className ; 
+ ``removeClass()`` 移除 className ; 
+ ``toggleClass()`` 切换 className ;
+ ``hasClass()`` 是否包含 className ; 
<br/>
<br/>
已经提供了以下实例
<a href="#/example/effect/className" target="_blank">className 实例</a>

![IE8IconPosition](../static/img/doc/className.png)
### 注意
+ ``addClass()`` 对于元素来说、class 属性可以有多个值、该方法不会移除已经存在的值、而是在原有的基础上追加一个或多个 class 属性；
+ ``removeClass()`` 如需移除多个值、用空格来分隔类名；
+ ``toggleClass()`` 设置只进行删除或者只进行添加操作、 switch 值为 true 时、只添加、为 false 时、只删除 ``$(".XX").toggleClass("xxx",true)`` ;


## 四、层的位置的控制
已经提供了以下实例
<a href="#/example/effect/layer" target="_blank">层的位置的控制实例</a>

![IE8IconPosition](../static/img/doc/layer.png)




## 五、设置对象的 CSS
<a href="#/example/effect/css" target="_blank">设置对象的 CSS 实例</a>

+ 一个 ``$(element).css(property,value)``
+ 多个 ``$(element).css({property:value, property:value, ...})``


## 六、定时器 SetTimeout 的使用
<a href="#/example/effect/setTimeout" target="_blank">SetTimeout 实例</a>

+ ``setTimeout()`` 方法用于在指定的毫秒数后调用函数或计算表达式。








