# 拿到数据后如何重新渲染

## 使用 @Part 组件

使用 ajax 请求的数据传入此 Part 即可

### HTML

```html
<div id="QueryPart" jwcid="@Part" element="div">
    ...
</div>
```

### JS

```javascript
$.ajax.submit("", "../_asset/data/user.json", null, "userInfoPart", function(data){
	user = data;
	disabledArea("userInfoPart", false);
	endLoading();
}, function(code, info){
	endLoading();
	MessageBox.error("", code + "<br />" + info);
});
```

## 使用重渲染方法 

### 页面代码

```html
<span jwcid="@Insert" name="test" value="ognl:userName"></span>
<script>
    var userName = "Johnson"
    Wade.setRatio();
</script>
```

### 更新数据 

客户端 jwc 才支持(java方式请使用ajax方法刷新页面)

```javascript
user Name = "Bill"
Wade.JWC.process("test");
```

