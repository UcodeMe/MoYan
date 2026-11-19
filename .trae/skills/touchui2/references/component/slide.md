# 幻灯（c_slide/@Slide）
可通过切换的方式来查看的一组图片。 

## JWC 方式

<a href="#/example/component/slide" target="_blank">查看演示</a>

```html
<div name="mySlide" jwcid="@Slide">
	<li><img src="../../img/ad1.jpg" /></li>
	<li><img src="../../img/ad2.jpg" /></li>
	<li><img src="../../img/ad3.jpg" /></li>
	<li><img src="../../img/ad4.jpg" /></li>
</div>
```
## JS 方式
html
```html
<div id="mySlide" name="mySlide" class="c_slide">
	<div class="pic">
		<ul>
			<li><img src="../../img/ad1.jpg" /></li>
			<li><img src="../../img/ad2.jpg" /></li>
			<li><img src="../../img/ad3.jpg" /></li>
			<li><img src="../../img/ad4.jpg" /></li>
		</ul>
	</div>
	<div class="fn">
		<span class="prev"></span>
		<span class="next"></span>
	</div>
	<div class="page"></div>
</div>
```
js
```js
window["mySlide"] = new Wade.Slide("mySlide");
```

## 插入缩略图

jwc 方式
```html
<div name="mySlide" jwcid="@Slide" thumb="true">
	<li><img src="../../img/phone1.jpg" /></li>
	<li><img src="../../img/phone1-2.jpg" /></li>
	<li><img src="../../img/phone1-3.jpg" /></li>
	<li><img src="../../img/phone1-4.jpg" /></li>
</div>
```

js 方式
html
```html
<div id="mySlide" name="mySlide" class="c_slide">
	<div class="pic">
		<ul>
			<li><img src="../../img/phone1.jpg" /></li>
			<li><img src="../../img/phone1-2.jpg" /></li>
			<li><img src="../../img/phone1-3.jpg" /></li>
			<li><img src="../../img/phone1-4.jpg" /></li>
		</ul>
	</div>
	<div class="thumb">
		<div class="prev"></div>
		<div class="list">
			<ul>
			</ul>
		</div>
		<div class="next"></div>
	</div>
	<div class="fn">
		<span class="prev"></span>
		<span class="next"></span>
	</div>
	<div class="page"></div>
</div>
```
js
```js
window["mySlide"] = new Wade.Slide("mySlide",{
	useThumb:true
});
```

## 属性

| 参数名 | 参数类型         | 是否必须 | 默认值 | 描述                            |默认值 | 
| ------ | ---------------- | -------- | ------ | ------------------------------- |-------- |
| name   | java.lang.String | 是       | 无     | 组件实例名,同一个页面中不能重复 ||
| class  | java.lang.String | 否       | 无     | 自定义样式名                    ||
| style  | java.lang.String | 否       | 无     | 自定义样式                      ||
| thumb  | boolean          | 否       | 无     | 是否显示缩略图                  |<a href="#/example/component/slide-thumb" target="_blank">查看</a>|
| c_slide-2  | java.lang.String          | 否       | c_slide-2~c_slide-10   | 图片设置数目                  ||


