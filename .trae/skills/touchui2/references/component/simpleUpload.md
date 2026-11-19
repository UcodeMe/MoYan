# 单文件上传（@SimpleUpload）
将一个文件传送到服务器并返回 URL 地址。
注意：已上传文件数据在参数库 ucr_param.WD_F_FTPFILE 表

## JWC 方式

<a href="#/example/component/SimpleUpload" target="_blank">查看</a>

```html
<span jwcid="@SimpleUpload"
	name="simpleupload"
	ftpCode="CustomerCentre"
	fileTypes=".doc;.docx;.jpg;.png"
></span>
```

## JS 方式

html

```html
<span class="e_mix">
	<input class="e_black-light" readonly="readonly" type="text"/>
	<input type="hidden" id="simpleupload" name="simpleupload"/>
	<form><input type="file" style="position:absolute;width:0px;height:0px;left:-99999px;top:-99999px;" /></form>
	<span class="e_ico-loading" style="display:none"></span>
	<span class="e_ico-close" style="display:none"></span>
	<span class="e_ico-download" style="display:none"></span>
	<span class="e_ico-browse"></span>
	<span class="e_mixProgress">
		<span class="e_mixBar" style="width:0%"></span>
	</span>
</span>
```

js

```js
window["simpleupload"] = new Wade.SimpleUpload("simpleupload", {
	ftpCode:"personserv",
	filePath:"import",
	fileTypes:".txt;.doc;.docx;.jpg;.png"
});
```

## 属性

| 参数名         | 参数类型         | 是否必须 | 默认值        | 描述                                                         |
| -------------- | ---------------- | -------- | ------------- | ------------------------------------------------------------ |
| name           | java.lang.String | 是       | 无            | 组件实例名，同一个页面里不能重复                             |
| class          | java.lang.String | 否       | 无            | 自定义样式名                                                 |
| style          | java.lang.String | 否       | 无            | 自定义样式                                                   |
| value          | java.lang.String | 否       | 无            | 上传组件默认值，只能设置为已上传文件的fileId,设置后会自动加载文件信息 |
| readonly       | boolean          | 否       | false         | 是否只读                                                     |
| disabled       | boolean          | 否       | false         | 是否禁用                                                     |
| ftpCode        | java.lang.String | 否       | 无            | ftp站点编码                                                  |
| filePath       | java.lang.String | 否       | 无            | 文件上传的远端相对路径                                       |
| copyToFtpCode  | java.lang.String | 否       | 无            | 上传文件后复制文件到另一个站点的fpt站点编码，多个用","号分隔 |
| copyToFilePath | java.lang.String | 否       | upload/attach | 上传文件后复制文件到另一个站点的远端相对路径，多个用","号分隔 |
| fileDelete     | boolean          | 否       | true          | 移除或重新上传时，是否真实删除文件                           |
| fileSize       | java.lang.String | 否       | 无            | 允许上传的文件大小,单位为 MB，默认为 30M                     |
| fileTypes      | java.lang.String | 否       | 无            | 允许上传的文件类型，使用文件扩展名分号分割设置，例如(.doc;.docx;.jpg;.png) |
| needSuffix     | boolean          | 否       | false         | 上传后文件是否有后缀名                                       |

## JS API

加载多个文件及其信息

```js
simpleupload.loadFile('10006438');
```

绑定上传完成后的回调事件

```js
$("#simpleupload").afterAction(function(e, file){
	alert(file.name + "|" + file.fileId); // 从 file 对象中获取文件名和上传后的文件 id
});
```

