# 前台-客户端消息推送
![IE8IconPosition](../static/img/doc/6_1-1.png)

- 客户端通过 WebSocket 或 HTTP 长连接连接到消息推送服务器
- 每个客户端有唯一的会话标识，并根据其它业务属性，比如工号，部门，地州，进行分组。
- 应用进程通过 http 接口调用发对消息推送
- 消息推送服务将消息根据发送目标的分组信息将消息下发到客户端。
- 客户端接收到消息后执行回调处理消息内容

## 服务部署

客户端消息推送应用目录结构：

| \|-app                              |                                      |
| ----------------------------------- | ------------------------------------ |
| \|---bin                            | //启停脚本目录                       |
| \|------monitor-comet-server.sh     | //HTTP 长连接消息推送服务进程检查脚本 |
| \|------monitor-websocket-server.sh | //Websocket 消息推送服务进程检查脚本  |
| \|------start-comet-server.sh       | //HTTP 长连接消息推送服务进程启动脚本 |
| \|------stop-comet-server.sh        | //HTTP 长连接消息推送服务进程停止脚本 |
| \|------start-websocket-server.sh   | //Websocket 消息推送服务进程启动脚本  |
| \|------stop-websocket-server.sh    | //Websocket 消息随同服务进程停止脚本  |
| \|---log                            | //日志目录                           |
| \|-etc                              | //配置文件目录                       |
| \|---acl.conf                       | //推送接口访问控制                   |
| \|---comet-server.properties        | //HTTP 长连接消息推送服务配置         |
| \|---websocket-server.properties    | //Websocket 消息推送服务配置          |
| \|---log4j.properties               | //日志配置                           |
| \|-lib                              | //相关 jar 包目录                      |
| \|-sbin                             | //应用程序和公用脚本目录             |
| \|---cronolog                       | //日志切割程序                       |
| \|---monitor_process.sh             | //应用进程检测脚本                   |
| \|---setEnv.sh                      | //应用环境变量设置脚本               |

## 相关配置
### 服务端配置：
acl.conf配置
```html
[remote-address]
10.174.59.0/24
10.174.59.0/24
127.0.0.1
```
配置允许调用 HTTP 推送接口的 IP 段
### comet-server.properties 配置：
该配置为 HTTP 长连接消息推送服务配置
```html
comet.server.worker.clazz=com.ai.ipaas.busiframe.message.comet.server.impl.Worker
comet.server.http.push=true
comet.server.usemg=false
comet.server.acl=acl.conf
```
其中：

- comet.server.worker.clazz为 Worker 类实例，不许要修改
- comet.server.http.push 为是否打开 HTTP 推送接口，需要配置为 true
- comet.server.usemq 为是否使用消息队列服务接收推送消息，配置为 false
- comet.server.acl 为访问控制配置文件名，不需要修改

### websocket-server.properties 配置：
```html
websocket.server.worker.clazz=com.ai.ipaas.busiframe.message.webaocket.server.imp1.Worker
websocket.server.http.push=true
websocket.server.usemq=false
websocket.server.acl=acl.conf
```
其中：

- websocket.server.worker.clazz 为 Worker 类实例，不需要修改
- websocket.server.http.push 为是否打开 HTTP 推送接口，需要配置为 true
- websocket.server.usemq 为是否使用消息队列服务接收推送消息，配置为 false
- websocket.server.acl 为访问控制配置文件名，不需要修改


### websocket-server.properties 配置：
```html
websocket.server.worker.clazz=com.ai.ipaas.busiframe.message.webaocket.server.imp1.Worker
websocket.server.http.push=true
websocket.server.usemq=false
websocket.server.acl=acl.conf
```
其中：

- websocket.server.worker.clazz 为 Worker 类实例，不需要修改
- websocket.server.http.push 为是否打开 HTTP 推送接口，需要配置为 true
- websocket.server.usemq 为是否使用消息队列服务接收推送消息，配置为 false
- websocket.server.acl 为访问控制配置文件名，不需要修改

### 应用配置：

应用配置为 msg.xml 配置文件，配置内容如下：

```html
<?xml version = '1.O' encoding = 'UIF-8'?>
<msg>
	<!-- 默认分组 -->
	<default-group>group1</default-group>
	<group name="group1">
		<!--消息类型 -->
		<type>websocket</type>
		<!--接入地址 -->
		<access>10.220.67.211:16111</access>
		<cluster>
			<!-- 群集地址-->
		<address>10.220.67.211 :16111</address>
		</cluster>
	</group>
</msg>

```
其中：
- default-group 为默认分组配置，通常只有一组
- type 为客户端消息推送服务类型，可选：websocket, comet (HTTP 长连接)
- access 为客户端消息推送服务客户端接入地址，如果有前置负载均衡，则此处需要配置为负载均衡的地址
- cluster 为客户端消息推送服务集群地址，每个服务实例地址配置一条

## 连接和推送

### 客户端连接API(JS)：
示例：

```js
var params = "ssessionId=" + (window.WSID ? window.WSID : "");
	params += "sstaffId=" + (window.staffId ? window.staffId : "") ;
	params += "sdepartCode=" + (window.departCode ? window .departCode : "") ;
	params += "separchyCode=" + (window.staffEparchyCode ? window.staffEparchyCode : "") ;
var rpc = new $.RPC({
	address: addr,
	type: type,
	session: params,
	message: function (data) {
	//todo 处理消息数据
	}
});
```

其中：

- addr 和 type 分别为通过 服务端 JAVA API 的 ``MessageFactory.getDefaultAccess()``;和 ``MessageFactory.getDefautType()``
- 获取 msg.xml 中的配置 sessionId, staffId, departCode,eparchyCode 为固定参数，分别为会话标识，工号，部门编码，地州编码。
- message 方法为消息接收的回调函数，形参 data 为接收到的消息内容，数据为 JSON 对象

### 消息推送API（JAVA）：

```js

//推送消息给客户端

Map<String, Object> content = new HashMap<String, Object>();
content.put("TYPE",BusinessConstants.MESSAGE.MSGTYPE_IMPORT);
content.put("STATUS","ok");
content.put("DOWNLOAD_URL",errorUrl);
content.put("ERROR_INFO",errorInfo);
Map<String,Object>message=new HashMap<String,Object>();
message.put("SESSION_ID",rpcSessionId);
message.put("CONTENT",content);
try{
	MessageFactory.sendMessage(message);
}catch(Exception ex1) {
	log.error(ex1) ;
}
```
content 为消息内容, message 为消息体，SESSION_ID 为接收消息的客户端连接会话标识

支持的发送维度：

- SESSION_ID: 发送给特定会话标识的客户端连接
- STAFF_ID: 发送给指定工号的所有客户端连接
- DEPART_CODE： 发送给指定部门标识的客户端连接
- EPARCHY_CODE: 发送给指定地州编码的客户端连接