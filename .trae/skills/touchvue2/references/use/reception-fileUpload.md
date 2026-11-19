# 前台-文件上传下载

## 功能说明
![IE8IconPosition](../static/img/doc/4_1-1.png)

- 客户端上传下载时与服务端 FileServlet 交互
- FileServlet 反射调用IFileAction接口的实现类，完成文件的上传、下载、查询、删除等操作
- IFileAction 接口的实现类读取数据库表中的 FTP 站点配置，将文件上传到 FTP 站点或对 FTP 站点的文件进行下载、删除等操作。
- IFileAction 接口的实现类对文件记录表进行操作，维护上传文件的信息。

## 相关配置
### 1.Web Servlet配置

web.xml 中配置 FileServlet
```html
<!--文件处理配置：上传下载，所有前台模块都需要配置-->
<servlet>
	<servlet-name>attach</servlet-name>
	<servlet-class>com.ailk.web.servlet.file.FileServlet</servlet-class>
	<load-on-startup>0</load-on-startup>
</servlet>
<servlet-mapping>
	<servlet-name>attach</servlet-name> 
	<url-pattern>/attach</url-pattern>
</servlet-mapping>
```
### 2.应用接口实现类配置

global.properties 中配置：fileman.fileaction=com.ai.baseframe.ipaas.ftpmgr.FtpFileAction
### 3.数据库表参数配置
base.bs_ftp 表配置站点信息

![IE8IconPosition](../static/img/doc/4_1-3.png)

base.bs_ftp_path 表配置路径配置

![IE8IconPosition](../static/img/doc/4_1-4.png)