# 页面嵌套（@Frame）
将指定页面在当前页面中展示。

## HMTL
<a href="" target="_blank">查看演示</a>
```html
<!-- 使用 page、listener 方式 -->
<iframe name="myFrame" jwcid="@Frame" page="examples.MessageBox" params="&aa=11" />

<!-- 使用 src 方式 -->
<iframe name="myFrame" jwcid="@Frame" src="?service=page/examples.component.Table&listener=init&aa=11" />
```

## 属性

| 参数名   | 参数类型         | 是否必须 | 默认值 | 描述                                                         |
| -------- | ---------------- | -------- | ------ | ------------------------------------------------------------ |
| name     | java.lang.String | 是       | 无     | 组件实例名，同一个页面不能重复                               |
| class    | java.lang.String | 否       | 无     | 自定义样式名                                                 |
| style    | java.lang.String | 否       | 无     | 自定义样式                                                   |
| page     | java.lang.String | 否       | 无     | 页面注册的 pageName                                          |
| listener | java.lang.String | 否       | 无     | 页面 java 类的监听方法名                                     |
| params   | java.lang.String | 否       | 无     | 打开页面时携带的 url 参数                                    |
| subsys   | java.lang.String | 否       | 无     | 子系统模块相对路径（打开其它模块页面时需要，类似“/customercentre/customercentre”这种相对路径地址） |
| src      | java.lang.String | 否       | 无     | Frame 嵌套页面地址，注意：src 方式设置地址和 page、listener 方式设置地址只能选择一种 |
| init     | boolean          | 否       | false  | 是否加载时打开页面，默认为 false（如果 Frame 组件放在弹窗或 Tabset 组件中，请使用默认值，否则传 true） |

## JS API

初始化

仅在 Frame 组件的 init 属性为 false 的情况下使用 					

```js
myFrame.init();
```

重新加载

```js
myFrame.reload();
```

跳转

```js
// API：
myFrame.redirect(page, listener, params, subsys);

// 示例：
tabFrame.redirect('examples.Popup', 'init', '&a=1&b=2', '/ordercentre/ordercentre');
```

开始 loading

```js
tabFrame.beginLoading();
```

结束 loading

```js
tabFrame.endLoading();
```

