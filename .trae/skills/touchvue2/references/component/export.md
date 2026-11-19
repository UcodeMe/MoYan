# 导出（@SimpleExport / @Export）
将一组数据以一定的格式下载到本地。

## HTML
<a href="#/example/component/export" target="_blank">查看演示</a>

平铺模式

<a href="#/example/component/export-input" target="_blank">查看演示</a>

```html
<!-- @SimpleExport 组件为平铺模式，可以放在表单区域 -->
<li>
	<div class="label">导出</div>
	<div class="value">
		<span name="simpleExport" 
		jwcid="@SimpleExport"
		taskId="999999998"
		configFile="export/task.xml"
		ftpCode="CustomerCentre"
		/>
	</div>
</li>	
```

按钮模式

<a href="#/example/component/export-button" target="_blank">查看演示</a>

```html
<!-- @Export 组件为按钮下拉形式，通常放在功能按钮区域 -->
<span jwcid="@Export"
	name="myExport"
	taskId="999999998"
	configFile="export/task.xml"
	ftpCode="CustomerCentre"
></span>
```

## 属性

平铺模式 

| 参数名     | 参数类型         | 是否必须 | 默认值  | 描述                                                         |
| ---------- | ---------------- | -------- | ------- | ------------------------------------------------------------ |
| name       | java.lang.String | 是       | 无      | 组件实例名，同一个页面内不能重复                             |
| class      | java.lang.String | 否       | 无      | 自定义样式名                                                 |
| style      | java.lang.String | 否       | 无      | 自定义样式                                                   |
| disabled   | boolean          | 否       | false   | 是否禁用                                                     |
| taskId     | java.lang.String | 是       | 无      | 导出异步任务编号                                             |
| cond       | java.lang.String | 否       | 无      | 导出时需要传入的条件参数区域 id                              |
| params     | java.lang.Object | 否       | 无      | 导出时携带的额外参数                                         |
| ftpCode    | java.lang.String | 否       | 无      | ftp 站点编码，使用 FtpGateWay 方式的请参考 FtpGateWay 配置文档 |
| filePath   | java.lang.String | 否       | 无      | 文件导入路径                                                 |
| fileType   | java.lang.String | 否       | "excel" | 导出时的文件保存远端路径                                     |
| fileName   | java.lang.String | 否       | 无      | 导出文件名                                                   |
| configFile | java.lang.String | 是       | 无      | 导出模板定义配置文件                                         |
| posX       | java.lang.String | 否       | 无      | 导出 Excel 文件的 X 坐标偏移量                               |
| posY       | java.lang.String | 否       | 无      | 导出 Excel 文件的 Y 坐标偏移量                               |

按钮模式 

| 参数名     | 参数类型         | 是否必须 | 默认值  | 描述                                                         |
| ---------- | ---------------- | -------- | ------- | ------------------------------------------------------------ |
| name       | java.lang.String | 是       | 无      | 组件实例名，同一个页面内不能重复                             |
| class      | java.lang.String | 否       | 无      | 自定义样式名                                                 |
| style      | java.lang.String | 否       | 无      | 自定义样式                                                   |
| buttonText | java.lang.String | 否       | "导出"  | 按钮文字                                                     |
| icoMode    | boolean          | 否       | false   | 是否使用图标模式（无按钮和文字）                             |
| disabled   | boolean          | 否       | false   | 是否禁用                                                     |
| taskId     | java.lang.String | 是       | 无      | 导出异步任务编号                                             |
| cond       | java.lang.String | 否       | 无      | 导出时需要传入的条件参数区域 id                              |
| params     | java.lang.Object | 否       | 无      | 导出时候携带的额外参数                                       |
| ftpCode    | java.lang.String | 否       | 无      | ftp 站点编码，使用 FtpGateWay 方式的请参考 FtpGateWay 配置文档 |
| filePath   | java.lang.String | 否       | 无      | 文件导入路径                                                 |
| fileType   | java.lang.String | 否       | "excel" | 导出时的文件保存远端路径                                     |
| fileName   | java.lang.String | 否       | "excel" | 导出文件名                                                   |
| configFile | java.lang.String | 是       | 无      | 导出模板定义配置文件                                         |
| posX       | java.lang.String | 否       | 无      | 导出 Excel 文件的 X 坐标偏移量                               |
| posY       | java.lang.String | 否       | 无      | 导出 Excel 文件的 Y 坐标偏移量                               |

xml模板配置说明 

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