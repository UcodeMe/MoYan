# 多文件上传（@Upload）
将多个文件传送到服务器并返回 URL 地址。
注意：已上传文件数据在参数库 ucr_param.WD_F_FTPFILE 表 

## JWC 方式

<a href="#/example/component/upload" target="_blank">查看演示</a>

```html
<span jwcid="@Upload"
	name="fileupload"
	title="文件上传"
	ftpCode="CustomerCentre"
	fileTypes=".doc;.docx;.jpg;.png"
></span>
```

## JS 方式

html
```html
<!-- 触发按钮 -->
<span class="e_mix" ontap="showPopup('UI-popup','UI-popup-upload');">
	<input id="upfileValue" name="upfileValue" type="text" value="" />
	<span class="e_ico-browse"></span>
</span>


<!-- 弹出界面，必须放置在 body 直接子对象的位置 -->
<div class="c_popup" id="UI-popup">
	<div class="c_popupBg" id="UI-popup_bg"></div>
	<div class="c_popupBox">
		<div class="c_popupWrapper" id="UI-popup_wrapper">
			<div class="c_popupGroup">
				<div class="c_popupItem" id="UI-popup-upload">
					<div class="l_float" x-wade-uicomponent="upload" name="fileupload" id="fileupload">
						<div class="c_header">
							<div class="back" ontap="backPopup(this)">文件上传</div>
							<div class="fn">
								<form><input type="file" multiple="true" style="position:absolute; width:0px; height:0px; left:-99999px; top:-99999px;" /></form>
								<span class="e_flash">
									<button class="e_button-blue" type="button">
										<span class="e_ico-browse"></span><span>选择文件</span>
									</button>
								</span>
								<button class="e_button-blue" type="button"><span class="e_ico-upload"></span><span>开始上传</span></button>
							</div>
						</div>
						<div class="c_tip c_tip-blue" style="height:4em;">
							<div class="content"></div>
						</div>
						<div class="c_scroll c_scroll-submit c_scroll-white c_scroll-float" style="top:7.57em;">
							<div class="c_scrollContent">
								<div class="c_list c_list-line">
									<ul>

									</ul>
								</div>
								<div class="c_line"></div>
							</div>
							<div class="c_msg c_msg-s c_msg-full c_msg-loading" style="display:none">
								<div class="wrapper">
									<div class="emote"></div>
									<div class="info">
										<div class="text">
											<div class="title">正加载文件列表..</div>
											<div class="content"></div>
										</div>
									</div>
								</div>
							</div>	
						</div>
					</div>
					<div class="l_bottom">
						<div class="c_submit c_submit-full">
							<button tag="clear" type="buttton" class="e_button-l e_button-navy">清除</button>
							<button tag="cancel" type="button" ontap="backPopup(this)" class="e_button-l e_button-navy">取消</button>
							<button tag="ok" type="button" class="e_button-l e_button-blue">确定</button>
						</div>
					</div>
				</div>
			</div>
		</div>
	</div>
</div>
```

js

```js
window["fileupload"] = new Wade.Upload("fileupload", {
	ftpCode:"ResourceCentre",
	filePath:"import",
	fileTypes:".doc;.docx;.jpg;.png"
});
```

## 属性

| 参数名         | 参数类型         | 是否必须 | 默认值        | 描述                                                         |
| -------------- | ---------------- | -------- | ------------- | ------------------------------------------------------------ |
| name           | java.lang.String | 是       | 无            | 组件实例名，同一个页面内不能重复                             |
| title          | java.lang.String | 否       | ''            | 多文件上传组件标题                                           |
| multiple       | boolean          | 否       | false         | 是否批量上传                                                 |
| readonly       | boolean          | 否       | false         | 是否只读                                                     |
| disabled       | boolean          | 否       | false         | 是否禁用                                                     |
| ftpCode        | java.lang.String | 否       | 无            | ftp站点编码                                                  |
| filePath       | java.lang.String | 否       | 无            | 上传文件路径                                                 |
| copyToFtpCode  | java.lang.String | 否       | 无            | 上传文件后复制文件到另一个站点的 ftp 站点编码，多个用","号分隔 |
| copyToFilePath | java.lang.String | 否       | upload/attach | 上传文件后复制文件到另一个站点的远端相对路径，多个用","号分隔 |
| fileDelete     | boolean          | 否       | true          | 移除或重新上传时，是否真实删除文件                           |
| fileRepeat     | boolean          | 否       | false         | 是否允许重复上传同一个文件                                   |
| fileLimit      | java.lang.String | 否       | false         | 上传文件最大个数，默认为 10 个                               |
| fileTypes      | java.lang.String | 否       | 无            | 允许上传的文件类型，使用文件扩展名分号分割设置，例如(.doc;.docx;.jpg;.png) |
| fileSize       | java.lang.String | 否       | 无            | 允许上传的文件大小，单位为 MB，默认为 30M                    |
| needSuffix     | boolean          | 否       | false         | 上传后文件是否有后缀名                                       |
| needBatchDownload     | boolean          | 否       | false         | 批量下载、仅下载通过upload组件上传成功的文件                                       |

## JS API

加载多个文件及其信息

```js
fileupload.loadFile('10006318,10006389,10006339');
```

绑定确定按钮事件

```js
$("#fileupload").select(function(){
	//this.fileSet() 返回已上传文件的json Array对象,如   [{name:name1, id:id1}, {name:name2,id:id2}]
	//alert( $.DatasetList(this.fileSet()) );
	//this.val() 返回已上传文件的名称和Id对象 {NAME : "name1,name2,name3", ID : "id1,id2,id3"}
	var obj = this.val();
	$("#upfileText").text(obj.NAME);
	$("#upfileValue").val(obj.ID);
});
```

绑定上传组件清除按钮事件

```js
$("#fileupload").clear(function(){
	$("#upfileText").text("");
	$("#upfileValue").val("");
});
```

清空上传组件文件
```js
fileupload.reset() // js删除  ( 打开同一个upload的popup, 需要先reset再loadFile(fileids),否则原有文件会累加 )
fileupload.reset(true) // 同时删除远端文件
```

下载ftp上的文件
```js
$.Upload.downloadFile(fileId, fileName, needSuffix)
```