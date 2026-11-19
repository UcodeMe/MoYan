# 悬浮提示（FloatBox）
悬浮于界面区域之外并在一定的时间后自动隐藏的区域，一般用于不需要手动关闭的操作提示，属于事中提醒。

## JS API

### 便捷模式

<a href="#/example/component/floatbox" target="_blank">查看演示</a>

```js
// API
showFloatBox(node, content, position, color, icon, keep);

// 示例
showFloatBox("Hello TouchUI");
showFloatBox(this, "Hello TouchUI", "left", "red", "user");
```

| 参数名 | 参数类型               | 是否必须 | 默认值  | 描述     |
| ------ | ---------------------- | -------- | ------- | -------- |
| node   | string 或原生 DOM 对象 | 否       | document.body      | 节点对象 |
| content    | string                 | 是       | "null"      | 提示信息 |
| position  | string                 | 否       | "top" | 色彩设置，支持 top / topLeft / topRight / bottom / bottomLeft / bottomRight / left / leftTop / leftBottom / right / rightTop / rightBottom |
| color  | string                 | 否       | "black" | 色彩设置，支持 black / blue / green / orange / red，content 如果是 HTML 此属性视为无效 |
| icon  | string                 | 否       | 无 | 图标设置，图标列表情参考<a href="#/doc/element/ico" target="_blank">图标文档</a>，content 如果是 HTML 此属性视为无效 |
| keep  | Number                 | 否       | 无 | 保留时长，单位为 ms |

### 完整模式

<a href="#/example/component/floatbox-settings" target="_blank">查看演示</a>

```js
// API
$.FloatBox.show(settings);

// 示例
$.FloatBox.show({
	node: "myButton",
	content: "<div>Hello TouchUI</div>", // 此处可以是 HTML 或 ID
	position: "bottom",
	color: "red",
	icon: "error",
	keep: 3000
})
```

注：如果 content 的值是 ID，ID 归属对象应是一个隐藏的对象，FloatBox 将提取其内容，且其内容应指定一定的宽度。<a href="#/example/component/floatbox-id" target="_blank">查看演示</a>