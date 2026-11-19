# 前台-客户端显示比例配置

Touch UI 的界面具备矢量伸缩的能力，整体界面是根据设备屏幕分辨率大小自动调整的，分辨率越大，字体越大，其默认值如下：

```js
ui.ratio.unit.pc = 1280 // PC 设备基础分辨率
ui.ratio.unit.pad = 1100 // Pad 设备基础分辨率
ui.ratio.unit.phone = 400 // Phone 设备基础分辨率
```

这个配置在 global.properties 文件中，将其值调整越小，单个控件（或文字）的显示越大，如：

```js
ui.ratio.unit.pc = 640
```

以上设置可以讲 PC 上的字体放大到原来的 2 倍。