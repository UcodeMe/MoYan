# 导入（@SimpleImport / @Import）
按照一定的模版向数据库批量添加数据。

## HTML 
<a href="#/example/component/import" target="_blank">查看演示</a>

平铺模式

<a href="#/example/component/import-input" target="_blank">查看演示</a>
```html
<!-- @SimpleImport 组件为平铺模式，可以放在表单区域 -->
<span jwcid="@SimpleImport"
	name="simpleImport" 
	taskId="999999998"
	configFile="export/task.xml"
	ftpCode="CustomerCentre"
></span>
```

按钮模式

<a href="#/example/component/import-button" target="_blank">查看演示</a>
```html
<span name="myImport" 
	taskId="999999999" 
	cond="queryPart"
	jwcid="@Import" 
	configFile="import/task.xml" 
	templateFile="ognl:'examples/template/' + urlEncode('任务模板') + '.xls'" 
	ftpCode="CustomerCentre"
></span>
```

## 属性

平铺模式 

| 参数名      | 参数类型         | 是否必须 | 默认值      | 描述                                                         |
| ----------- | ---------------- | -------- | ----------- | ------------------------------------------------------------ |
| name        | java.lang.String | 是       | 无          | 组件实例名，同一个页面内不能重复                             |
| class       | java.lang.String | 否       | 无          | 自定义样式名                                                 |
| style       | java.lang.String | 否       | 无          | 自定义样式                                                   |
| disabled    | boolean          | 否       | false       | 是否禁用                                                     |
| taskId      | java.lang.String | 是       | 无          | 导入异步任务编号                                             |
| cond        | java.lang.String | 否       | 无          | 导入时需要传入的条件参数区域 id                              |
| params      | java.lang.Object | 否       | 无          | 导入时候携带的额外参数                                       |
| auto        | boolean          | 否       | true        | 是否上传文件后自动进行导入操作                               |
| ftpCode     | java.lang.String | 否       | 无          | ftp 站点编码，使用 FtpGateWay 方式的请参考 FtpGateWay 配置文档 |
| filePath    | java.lang.String | 否       | 无          | 文件导入时的文件上传远端路径                                 |
| fileSize    | java.lang.String | 否       | 无          | 文件大小限制单位为 MB，默认为30M                             |
| fileType    | java.lang.String | 否       | "excel"     | 文件类型,可以设置为 excel 或 txt、csv 或 excel               |
| templateFile| java.lang.String | 是       | 无          | 导入模板文件下载路径，如果有中文需要使用 urlEncode 对中文进行编码 |
| configFile  | java.lang.String | 是       | 无          | 导入模板定义配置文件                                         |
| posX        | java.lang.String | 否       | 无          | 导入 Excel 文件的 X 坐标偏移量                               |
| posY        | java.lang.String | 否       | 无          | 导入 Excel 文件的 Y 坐标偏移量                               |
| txtSplit    | java.lang.String | 否       | 制表符 "\t" | 文本文件导入时的列分隔符                                     |
| pushMode    | boolean | 否       | false | 设置导入导出的进度是不是用推送方式更新给客户端浏览器                                     |

按钮模式

| 参数名      | 参数类型         | 是否必须 | 默认值      | 描述                                                         |
| ----------- | ---------------- | -------- | ----------- | ------------------------------------------------------------ |
| name        | java.lang.String | 是       | 无          | 组件实例名，同一个页面内不能重复                             |
| class       | java.lang.String | 否       | 无          | 自定义样式名                                                 |
| style       | java.lang.String | 否       | 无          | 自定义样式                                                   |
| buttonText  | java.lang.String | 否       | "导入"      | 按钮文字                                                     |
| icoMode     | boolean          | 否       | false       | 是否使用图标模式（无按钮和文字）                             |
| disabled    | boolean          | 否       | false       | 是否禁用                                                     |
| taskId      | java.lang.String | 是       | 无          | 导入异步任务编号                                             |
| cond        | java.lang.String | 否       | 无          | 导入时需要传入的条件参数区域 id                              |
| params      | java.lang.Object | 否       | 无          | 导入时候携带的额外参数                                       |
| auto        | boolean          | 否       | true        | 是否上传文件后自动进行导入操作                               |
| ftpCode     | java.lang.String | 否       | 无          | ftp 站点编码，使用 FtpGateWay 方式的请参考 FtpGateWay 配置文档 |
| filePath    | java.lang.String | 否       | 无          | 文件导入时的文件上传远端路径                                 |
| fileSize    | java.lang.String | 否       | 无          | 文件大小限制单位为 MB,默认为30M                              |
| fileType    | java.lang.String | 否       | "excel"     | 文件类型,可以设置为 excel 或 txt、csv 或 excel               |
| templateFile| java.lang.String | 是       | 无          | 导入模板文件下载路径，如果有中文需要使用 urlEncode 对中文进行编码 |
| configFile  | java.lang.String | 否       | 无          | 导入模板定义配置文件                                         |
| posX        | java.lang.String | 否       | 无          | 导入 Excel 文件的 X 坐标偏移量                               |
| posY        | java.lang.String | 否       | 无          | 导入 Excel 文件的 Y 坐标偏移量                               |
| txtSplit    | java.lang.String | 否       | 制表符 "\t" | 文本文件导入时的列分隔符                                     |

配置说明 

| 属性     | 说明                                                         |
| -------- | ------------------------------------------------------------ |
| name     | 数据映射名                                                   |
| desc     | 表格列头描述                                                 |
| type     | 表格列类型（1:String 2: NUMERIC 3:Datatime）                 |
| align    | 表格列横向排列方式（1:left 2:center 3:right）                |
| width    | 表格列宽度                                                   |
| format   | 表格列数据格式（如果是数字，格式为 0.00,#.## 类似数据格式形式，如果为时间，格式为 yyyy-MM-dd 类似形式） |
| scale    | 放比例，如将单位为分的数字表示成元，相当于缩小 100，则值为 scale = 100 |
| nullable | 是否可以不填 option[yes\|no] id                              |
| equsize  | 输入值的定长位数限制，如 equsize="10"                        |
| minsize  | 输入值的最小位数限制，如 minsize="10"                        |
| maxsize  | 输入值的最大位数限制，如 maxsize="100"                       |

