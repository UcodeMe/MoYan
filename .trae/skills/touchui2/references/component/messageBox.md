# 消息提示框（c_msg/MessageBox）
重点划分出一块区域以显示人机交互时机器反馈的信息。

## HTML
<a href="#/example/component/messageBox" target="_blank">查看</a>

```html
<div class="c_msg">
	<div class="wrapper">
		<div class="emote"></div>
		<div class="info">
			<div class="text">
				<div class="title">提示标题</div>
				<div class="content">提示内容信息</div>
			</div>
			<div class="fn">
				<button type="button">确定</button>
				<button type="button">取消</button>
			</div>
		</div>
	</div>
</div>
```

## JS 方式

### 一般提示

<a href="#/example/component/messageBox-alert" target="_blank">查看演示</a>

常用于提醒，默认只有 “确定”（button 值为 ok）按钮

```js
MessageBox.alert("提示信息", "提示信息", function(btn){
	alert(btn);
});
```

### 错误提示

<a href="#/example/component/messageBox-error" target="_blank">查看演示</a>

常用于异常或故障提示，默认只有 “取消”（button 值为 cancel）按钮

```js
MessageBox.error("错误信息", "错误信息", function(btn){
	alert(btn); //打印button值
});
```

错误提示显示详情
```js
MessageBox.error("成功信息", "成功信息", function (btn) {
    if ("ext1" == btn) {
        alert("打印");
    }
}, {
    "ext1": "确定",
},"错误详情")

```


### 成功提示

<a href="#/example/component/messageBox-success" target="_blank">查看演示</a>

常用成功提示，默认只有“确定”（button 值为 ok）按钮

```js
MessageBox.success("成功信息", "成功信息", function(btn){
	alert(btn);
});

```

### 全局设置


```js
// 弹框按钮逆序排列
MessageBox.buttonOrder = "desc"

// 默认显示详情
MessageBox.showDetail = false

// 禁用详情按钮
MessageBox.disableDetail = "asc"

// 以弹出模式显示
MessageBox.fullMode = false

// 提交异常信息
Messagebox.reportDetail = false
```

### 确认信息

<a href="#/example/component/messageBox-confirm" target="_blank">查看演示</a>

常用于确认选择交互,默认有 “确定”（button 值为 ok）和“取消”（button 值为 cancel）按钮

```js
MessageBox.confirm("提示信息", "提示信息", function(btn){
	alert(btn);
});
```

### 自定义扩展按钮

<a href="#/example/component/messageBox-userDefined" target="_blank">查看演示</a>

显示成功信息 + 扩展按钮

alert、error、confirm、success 方法都可以设置扩展按钮
共三个默认按钮可以供使用, button 值分别为 ext0、ext1、ext2

```js
MessageBox.success("成功信息", "成功信息", function(btn){
	if("ext1" == btn){
		alert("打印");
	}
}, {"ext1" : "打印", "ext2": "提交辅助信息"});
```

### 完全自定义用法

完全自定义显示 MessageBox 方式，可以设置 beforeHide 方法控制是否关闭消息框

```js
MessageBox.fullMode = false; //设置为非全屏模式
MessageBox.show({
	success : true,
	msg : "成功信息1",
	content : "成功信息1",
	buttons : {"ok" : "确认", "ext1" : "打印", "ext2" : "扩展按钮"},
	fn : function(btn){
		if("ext1" == btn){
			alert("打印");
		}
	},
	beforeHide : function(btn){
		if("ext1" == btn) {
			return window.confirm("是否设置校验成功？");
		}
	}
});
```

## 样式

| 参数名        | 值                                                           | 默认值 | 描述     |演示     |
| ------------- | --- | ------------------------------------------------------------ | ------ | -------- |
| c_msg-however | c_msg-smlie(微笑)<br />c_msg-warn(不悦/警示) <br />c_msg-success(开心/成功) <br />c_msg-error(流泪/错误) <br />c_msg-help(帮助) <br />c_msg-sweat(汗颜) <br />c_msg-angry(生气)<br />c_msg-however(无奈) <br />c_msg-embrsd(窘迫) <br />c_msg-loading(努力载入中) | 无     | 表情设置 |<a href="#/example/component/messageBox-emoticon" target="_blank">查看</a>|
| c_msg-h       | c_msg-h                                                      | 无     | 水平模式 |<a href="#/example/component/messageBox-h" target="_blank">查看</a>|
| c_msg-popup   | c_msg-popup                                                  | 无     | 弹出模式 |<a href="#/example/component/messageBox-popup" target="_blank">查看</a>|
| c_msg-full    | c_msg-full                                                   | 无     | 全屏模式 |<a href="#/example/component/messageBox-full" target="_blank">查看</a>|
| c_msg-border  | c_msg-border                                                 | 无     | 边框设置 |<a href="#/example/component/messageBox-border" target="_blank">查看</a>|
| c_msg-s       | c_msg-s                                                      | 无     | 大小设置 ||


## 插件 

### 详情设置

<a href="#/example/component/messageBox-detail" target="_blank">查看演示</a>

```html
<div class="c_msg c_msg-h c_msg-error">
	<div class="wrapper">
		<div class="emote"></div>
		<div class="info">
			<div class="text">
				<div class="title">提示标题</div>
				<div class="content">提示内容信息</div>
			</div>
			<div class="detail">
				<div class="text">详情</div>
			</div>
			<div class="fn">
				<button type="button">确定</button>
				<button type="button">取消</button>
			</div>
		</div>
	</div>
</div>
```

## 响应规则

| 参数名          | 值|描述                                   | 演示 |
| ------------- | ---|-------------------------------------- |--- |
| c_msg-phone-v |c_msg-phone-v| 手机端使用垂直模式                 |<a href="#/example/component/messageBox-phone-v" target="_blank">查看</a>|
| fn-phone-v    | fn-phone-v|手机端 fn 部分按钮排列使用垂直模式 |<a href="#/example/component/messageBox-fn-phone-v" target="_blank">查看</a>|

```html
<div class="c_msg">
	<div class="wrapper">
		<div class="emote"></div>
		<div class="info">
			<div class="text">
				<div class="title">提示标题</div>
				<div class="content">提示内容信息</div>
			</div>
			<div class="fn fn-phone-v">
				<button type="button">确定</button>
				<button type="button">取消</button>
			</div>
		</div>
	</div>
</div>
```
