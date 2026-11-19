# 如何强制横/纵屏打印

## 场景分析

在页面打印时，不管页面内容是何展示形式，需要让其强制打印为纵向/横向的方式。

## 解决方案

### 横向打印

css
```css
<style type="text/css" media="print">
	@page { size: landscape; }
</style>
```

### 纵向打印

css
```css
<style type="text/css" media="print">
	@page { size: portrait; }
</style>
```

## 兼容性
|Values|	IE|	Firefox|	Chrome|	Safari|	Opera	|iOS Safari|	Android Browser|	Android Chrome|
|-|-|-|-|-|-|-|-|-|
|Basic Support|	8.0+|	2.0+|	4.0+|	6.0|	15.0+	|6.0+|	2.1+|	18.0+|
