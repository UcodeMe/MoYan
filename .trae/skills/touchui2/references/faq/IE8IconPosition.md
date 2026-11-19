# IE8 下图标位置靠顶部

## 问题现象

![IE8IconPosition](../static/img/doc/IE8IconPosition.png)

## 解决方案

这一般是由于未使用正确的 doctype 导致的，将 doctype 改成如下代码即可：

```javascript
<!DOCTYPE HTML>
```

## 修复后

![IE8IconPosition2](../static/img/doc/IE8IconPosition2.png)