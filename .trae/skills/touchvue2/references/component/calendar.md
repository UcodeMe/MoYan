# 日历（@Calendar）
以星期为字段，以二维表格形式展示、选择日期数据。 

## JWC 方式
<a href="#/example/component/calendar" target="_blank">查看演示</a>

```html
<div name="myCalendar" jwcid="@Calendar"></div>
```

## JS 方式

html

```html
<div id="myCalendar" class="c_calendar">
	<div class="header">
		<div class="prevYear"></div>
		<div class="prev"></div>
		<div class="info">
			<span class="year"></span>
			<span class="month"></span>
		</div>
		<div class="next"></div>
		<div class="nextYear"></div>
	</div>
	<div class="select" style="display:none;">
		<div class="year">
			<span class="prev"></span>
			<ul>
				<li></li>
				<li></li>
				<li></li>
				<li></li>
				<li></li>
				<li></li>
				<li></li>
				<li></li>
				<li></li>
				<li></li>
			</ul>
			<span class="next"></span>
		</div>
		<div class="month">
			<ul>
				<li val="0">1月</li>
				<li val="1">2月</li>
				<li val="2">3月</li>
				<li val="3">4月</li>
				<li val="4">5月</li>
				<li val="5">6月</li>
				<li val="6">7月</li>
				<li val="7">8月</li>
				<li val="8">9月</li>
				<li val="9">10月</li>
				<li val="10">11月</li>
				<li val="11">12月</li>
			</ul>
		</div>
	</div>
	<div class="week">
		<ul>
			<li>日</li>
			<li>一</li>
			<li>二</li>
			<li>三</li>
			<li>四</li>
			<li>五</li>
			<li>六</li>
		</ul>
	</div>
	<div class="day">
		<ul>
			<li></li>
			<li></li>
			<li></li>
			<li></li>
			<li></li>
			<li></li>
			<li></li>
			<li></li>
			<li></li>
			<li></li>
			<li></li>
			<li></li>
			<li></li>
			<li></li>
			<li></li>
			<li></li>
			<li></li>
			<li></li>
			<li></li>
			<li></li>
			<li></li>
			<li></li>
			<li></li>
			<li></li>
			<li></li>
			<li></li>
			<li></li>
			<li></li>
			<li></li>
			<li></li>
			<li></li>
			<li></li>
			<li></li>
			<li></li>
			<li></li>
			<li></li>
			<li></li>
			<li></li>
			<li></li>
			<li></li>
			<li></li>
			<li></li>
		</ul>
	</div>
	<div class="time" style="display:none">
		<div class="hour">
			<div class="input">
				<input type="text" value="00" maxlength="2" />
			</div>
			<div class="text">时</div>
			<div class="option" style="display:none">
				<ul>
					<li>1</li>
					<li>2</li>
					<li>3</li>
					<li>4</li>
					<li>5</li>
					<li>6</li>
					<li>7</li>
					<li>8</li>
					<li>9</li>
					<li>10</li>
					<li>11</li>
					<li>12</li>
					<li>13</li>
					<li>14</li>
					<li>15</li>
					<li>16</li>
					<li>17</li>
					<li>18</li>
					<li>19</li>
					<li>20</li>
					<li>21</li>
					<li>22</li>
					<li>23</li>
					<li>24</li>
				</ul>
			</div>
		</div>
		<div class="min">
			<span class="input">
				<input type="text" value="00" maxlength="2" />
			</span>
			<span class="text">分</span>
			<div class="option" style="display:none">
				<ul>
					<li>0</li>
					<li>5</li>
					<li>10</li>
					<li>15</li>
					<li>20</li>
					<li>25</li>
					<li>30</li>
					<li>35</li>
					<li>40</li>
					<li>45</li>
					<li>50</li>
					<li>55</li>
				</ul>
			</div>
		</div>
		<div class="sec">
			<span class="input">
				<input type="text" value="00" maxlength="2" />
			</span>
			<span class="text">秒</span>
			<div class="option" style="display:none">
				<ul>
					<li>0</li>
					<li>5</li>
					<li>10</li>
					<li>15</li>
					<li>20</li>
					<li>25</li>
					<li>30</li>
					<li>35</li>
					<li>40</li>
					<li>45</li>
					<li>50</li>
					<li>55</li>
				</ul>
			</div>
		</div>
	</div>
	<div class="shortcut" style="display:none;">
		<ul>
			<li tag="HalfYearAgo">半年前</li>
			<li tag="ThreeMonthAgo">三月前</li>
			<li tag="PrevMonthToday">一月前</li>
			<li tag="Today">今天</li>
			<li tag="NextMonthFirst">次月1日</li>
			<li tag="NextMonthLast">次月末日</li>
			<li tag="NextYearToday">明年今日</li>
			<li tag="2050">2050</li>
		</ul>
	</div>
	<div class="fn">
		<button tag="clear" type="button" class="e_button-l e_button-navy">清除</button>
		<button tag="quicksel" type="button" class="e_button-l e_button-navy"><span>快捷选择</span><span tag="fold" class="e_ico-fold"></span></button>
		<button tag="ok" type="button" class="e_button-l e_button-blue">确定</button>
	</div>
</div>
```

js
```js
window["myCalendar"] = new Wade.Calendar("myCalendar", {
	now:new Date(1475235396350)
});
```

## 属性

| 参数名 | 参数类型         | 是否必须 | 默认值     | 描述                                                         |演示 |
| ------ | ---------------- | -------- | ---------- | ------ | ------------------------------------------------------------ |
| name   | java.lang.String | 是       | 无         | 组件实例名，同一个页面中不能重复                             ||
| class  | java.lang.String | 否       | 无         | 自定义样式名                                                 ||
| style  | java.lang.String | 否       | 无         | 自定义样式                                                   ||
| value  | java.lang.String | 否       | 无         | 当前值                                                       ||
| format | java.lang.String | 否       | yyyy-MM-dd | 日期显示格式                                                 ||
| lunar  | boolean          | 否       | ture       | 是否显示农历日期                                             |<a href="#/example/component/calendar-lunar" target="_blank">查看</a>|
| time   | boolean          | 否       | false      | 是否显示时分秒功能，默认不显示，设置为 true 时 要保证与 format 一致 |<a href="#/example/component/calendar-time" target="_blank">查看</a>|

## JS API

绑定选择事件

```js
$("#myCalendar").select(function(e){
	if(activeDateField && activeDateField.nodeType){
		$(activeDateField).val(this.val()); // 进行日期选择时候，把选择的日期值返回到 DateField 元素上
		// alert(this.lunarVal()); // 获取农历日期值
	}
});
```

绑定清除事件

```js
$("#myCalendar").clear(function(e){
	if(activeDateField && activeDateField.nodeType){
		$(activeDateField).val( "" );
	}
});
```

## 样式

| 参数名     | 值           |  默认值 | 描述     |演示     |
| ---------- | ------------ | ------ | -------- |---- | 
| c_calendar-gd |c_calendar-gd |  无     | 渐变 |<a href="#/example/component/calendar-gd" target="_blank">查看</a>|
| c_calendar-s |c_calendar-s |  无     | 大小设置 |<a href="#/example/component/calendar-s" target="_blank">查看</a>|

## 复合案例
<a href="#/example/component/calendar-mix" target="_blank">查看演示</a>

```html
<!-- input 框部分 -->
<div class="c_list">
	<ul>
		<li class="link" ontap="showPopup('myPopup','myPopup-calendar'); setCalendarValue('myDatefiled-start');">
			<div class="label">开始时间</div>
			<div class="value"><input name="myDatefiled-start" jwcid="@DateField" /></div>
			<div class="more"></div>
		</li>
		<li class="link" ontap="showPopup('myPopup','myPopup-calendar'); setCalendarValue('myDatefiled-end');">
			<div class="label">结束时间</div>
			<div class="value"><input name="myDatefiled-start" jwcid="@DateField" /></div>
			<div class="more"></div>
		</li>
	</ul>
</div>

<!-- 弹出框部分，必须是 body 的直接子对象 -->
<div class="c_popup" id="myPopup">
	<div class="c_popupBg" id="myPopup_bg"></div>
	<div class="c_popupBox">
		<div class="c_popupWrapper" id="myPopup_wrapper">
			<div class="c_popupGroup">
				<div class="c_popupItem" id="myPopup-calendar">
					<div class="c_header">
						<div class="back" ontap="hidePopup(this)">选择时间</div>
					</div>
					<div class="c_scroll c_scroll-float c_scroll-header">
						<div name="myCalendar" jwcid="@Calendar"></div>
					</div>
				</div>
			</div>
		</div>
	</div>
</div>
```

```js
// 建立一个变量存储当前 datefield
var activeDateField;

// 打开 popup 的同时，将 datefield 的值赋给 calendar
function setCalendarValue(dateField){
	var el = dateField && dateField.nodeType == 1 ? dateField : document.getElementById(dateField);
	if(!el) return;
	myCalendar.val(el.value);
	activeDateField = el;
}

// 选择日期后，将 calendar 的返回给 datefield，并关闭 popup
$("#myCalendar").select(function(e){
	if(activeDateField && activeDateField.nodeType){
		$(activeDateField).val( this.val() );
		backPopup(e.target);
	}
});
```