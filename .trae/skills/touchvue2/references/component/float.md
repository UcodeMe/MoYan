# 悬浮层（c_float）
用于放置次级信息的临时弹出层，在操作其他信息时会自动消失。 

## HTML
<a href="#/example/component/float" target="_blank">查看演示</a>

```html
<!-- 触发按钮，可以是任意元素 -->
<button type="button" x-wade-float="myFloatLayer" ontap="myShowFloatLayerFn(this);"><span class="e_ico-view"></span><span>点击测试</span></button>

<!-- 悬浮层部分，必须是 body 的直接子对象 -->
<div id="myFloatLayer" class="c_float" style="width:30em; top:10em; left:10em;">
	<div class="bg" ontap="hideFloatLayer('myFloatLayer')"></div>
	<div class="content">
		...
	</div>
</div>
```

## JS
```js
function myShowFloatLayerFn(o){

	// 获取 button 位置并设置给 floatLayer
	var left = $(o).offset().left;
	var top = $(o).offset().top;
	var width = $(o).width();
	var height = $(o).height();
	$("#myFloatLayer").css("top",top + height);
	$("#myFloatLayer").css("left",left);
	$("#myFloatLayer").css("width",width);

	// 打开 floatLayer
	toggleFloatLayer('myFloatLayer');
}
```

## JS API

隐藏层
```
hideFloatLayer("id");
```

切换层
```
toggleFloatLayer("id");
```

某种特殊情况下，悬浮层中需要打开其他的悬浮层（比如选择框、日期输入框等）时，点击二级悬浮层会导致 “点击到悬浮层之外”，继而触发 hideFloatLayer()。此时，需要设置一个使其无法关闭的条件，如：

```
// 当 id 为 _Wade_DropDownCalendar_float 的 DOM 对象含有样式 "c_float-show" 时（即处于打开状态），弹出层不会关闭
function stopHideLayerCondition() {
	return $("#_Wade_DropDownCalendar_float").hasClass("c_float-show");
}
```

## 响应规则

| 参数名             | 值| 描述   |演示   |
| ------------------- | ----|------------------- | --------- | 
| c_float-phone-auto  |c_float-phone-auto|  手机端不限高度设置                                                 |<a href="#/example/component/float-phone-auto" target="_blank">查看</a>|
| c_float-phone-notop |c_float-phone-notop |手机端无顶部模式 （ 说明：无顶部模式是指遮罩部分将会覆盖到 c_header 的部分。 ） |<a href="#/example/component/float-phone-notop" target="_blank">查看</a>|
| c_float-phone-menu  |c_float-phone-menu | 手机端菜单模式                                                     |<a href="#/example/component/float-phone-menu" target="_blank">查看</a>|
| c_float-phone-animate  |c_float-phone-animate| 手机端动画特效                                                     |<a href="#/example/component/float-phone-animate" target="_blank">查看</a>|

