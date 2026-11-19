# 前台-导入导出
## 异步任务调度
### 1.功能说明
![IE8IconPosition](../static/img/doc/5_1-1.png)

- 异步任务调度是指进程间通过消息队列服务通信，实现由一个进程发起对其它进程上的异步任务执行调用。
- 消息队列服务由 MQ-Server 守护进程和 Redis 缓存服务构成。
- 客户端进程以 JVM 参数 –Dwade.server.name=xxx 的值作为进程名标识。
- MQ-Server 守护进程从数据库加载异步任务配置到 Redis 缓存、从 cluster-definition 配置文件加载客户端进程的集群配置到 Redis 缓存。
- 客户端进程根据集群配置监听 Redis 缓存上的集群消息数据队列和私有消息数据队列。
- 消息生产者进程发起任务调用时，发起进程将要调用的任务标识和调用参数插入到 Redis 缓存目标消息队列数据中。
- 消息消费者进程的监听读取到消息队列的任务调用消息，根据任务标识从Redis缓存中获取对应的任务配置信息，发起对任务类的反射调用。
- 异步任务执行过程中，任务执行进程将异步任务执行的状态和异常信息发送到 Redis 缓存的日志队列，MQ-Server 守护进程读取队列数据进行异步任务执行状态日志记录和状态更新。

### 2.服务部署

消息队列服务应用目录结构：

| \|-app                      |                                |
| --------------------------- | ------------------------------ |
| \|---server                 |                                |
| \|------bin                 |                                |
| \|---------start-redis.sh   | //启动 redis 缓存脚本            |
| \|---------stop-redis.sh    | //停止 redis 缓存脚本            |
| \|---------start-server.sh  | //启动 MQ-Server 进程脚本        |
| \|---------stop-server.sh   | //停止 MQ-Server 进程脚本        |
| \|------log                 | //日志目录                     |
| \|-db                       | //redis 缓存持久化存储目录      |
| \|-etc                      | //配置文件目录                 |
| \|---acl.conf               | //Redis 缓存访问控制配置        |
| \|---redis.conf             | //Redis 缓存服务配置            |
| \|---cluster-definition.xml | //客户端进程集群配置           |
| \|---database.xml           | //MQ-Server 数据库连接配置      |
| \|---redis.xml              | //MQ-Server Redis缓存连接配置  |
| \|---log4j.properties       | //MQ-Server 日志输出配置       |
| \|---                       |                                |
| \|-lib                      | //MQ-Server服务用到的jar包目录 |
| \|-sbin                     | //应用程序和公用脚本目录       |
| \|---cronolog               | //日志切割程序                 |
| \|---redis-server           | //Redis缓存服务程序            |
| \|---redis-cli              | //Redis缓存客户端程序          |
| \|---monitor_process.sh     | //应用进程检查脚本             |
| \|---setEnv.sh              | //应用环境变量设置脚本         |

消息队列服务的守护应用只能启动一个实例，在生产环境通常做一主一备的冷备模式。

Redis 服务部署：

消息队列服务的 Redis 缓存可以和后台守护进程部署在一起也可以单独部署，生产环境应该使用主备模式。

### 3.相关配置

1. MQ-Server 服务进程配置

a) 应用进程集群配置

cluster-definition.xml 配置客户端进程归属集群。

```html
<?xml version="1.O" encoding="UIF-8"?>
<clusters>
	<cluster name="cluster-web-all">
		<server name="web-10.220.67.212" />
		<server name="web-10.220.67.213" />
	</cluster>
</clusters>
```
其中：server 节点的 name 属性为客户端进程的 -Dwade.server.name JVM 参数值

b) 数据库连接配置

database.xml 配置 MQ-Server 进程的数据库连接
```html
<?xml version="1.O" encoding="UIF-8"?>
<database limit="3000000" trace="false">
	<cen1
		type="dbcp"
		driver="oracle.jdbc.driver.OracleDriver"
		url="jdbc:oracle:thin:@10.154.59.152:21007:crmtestdb"
		user="base"
		passwd="0s2oPQkN"
		initialSize="l"
		maxActive="10"
		maxIdle="5"
		maxWait="1000"
		dialect="oracle"
		/>
</database>
```
其中： url 为 oracle jdbc 连接串， user 为 数据库用户名，passwd 是数据库用户密码，initialSize 为初始连接数，maxActive 为最大连接数。

c) Redis 服务器连接配置

redis.xml 配置 MQ-Server 进程的 Redis 缓存连接配置
```html
<?xml version = '1.0' encoding = 'UTF-8'?>
<redis>
	<default-datacenter>center1</default-datacenter>
	<datacenter name="center1">
		<cluaster name="mq" deac="消息中间件" >
			<!-- 心跳时间, 単位:秒,默认:5 -->
			<heartbeat-second>5</heartbeat-second>
			<!--连接池活跃连接数,默i人:5,消息中间件配置不得小于5个! -->
			<pool-size>5</pool-size>
			<address master="127.0.0.1:5672"/>
		</cluster>
	</datacenter>
</redis>
```
## 2.应用进程配置
a) MQ 客户端配置

mq-client.xml 中配置是否为生产模式

```html
<?xml version="1.O" encoding="UIF-B"?>
<messagequeue>
	<productmode>true</productmode>
</messagequeue>
```

测试、生产环境配置为 true,本机开发配置为 false,配置为 false 时候，所有的异步任务调用均会回调到发起进程，便于开发调试。
b) Redis 服务器连接配置

redis.xml 配置应用到消息队列服务 Redis 缓存的链接配置

```html

<?xml version = '1.0' encoding='UTF-8'?>

<redis>
      <default-datacenter>default-datacenter</default-datacenter>
	<datacenter name="default-datacenter">
      <cluster name="mq" desce="消息中间件" >
      <!--心跳时间，单位：秒，默认：5-->
      <heartbeat-second>15</heartbeat-second>
      <!-- 连接池活跃连接数，默认：5，消息中间配置不得小于5个！ -->
	<pool-size>5</pool-size>
      <address master="l0. 174.43.43:5672" /></cluster>
</redis>

```
### 3.数据库表参数配置

base.wd_m_asynctask 表配置
![IE8IconPosition](../static/img/doc/5_1-7.png)

base.wd_m_asynctask 表配置异步任务信息，包括：

TASK_ID: 异步任务标识

TASK_NAME: 异步任务名称

DESTINATION: 异步任务执行目标应用集群

CLASS_NAME: 异步任务实现类

TIMEOUT_SECEND: 异步任务执行超时时间，单位秒

新增异步任务配置需要配置到该表，MQ-Server 守护进程每5分钟重新加载一次任务配置到 Redis 缓存。
### 4.日志记录

base.wd_m_asynctask_log 异步任务执行日志：
![IE8IconPosition](../static/img/doc/5_1-8.png)

异步任务执行状态记录到 base.wd_m_asynctask_log 表中，其中：

LOG_ID 是日志流水

EXECUTOR 是具体执行任务进程的进程标识

TASK_ID 是对应任务标识

RESULTINFO 是执行状态，包括以下几种状态：

- ACCEPT: 消息队列服务已经接收到任务调度消息
- RUNNING: 任务调度消息已经发送给执行进程
- SUCCESS: 任务执行成功完成
- 错误堆栈： 任务执行异常

PARAMS 为任务执行的参数集，为 JSON 格式字符串

SEND_TIME 任务调度消息发送时间

START_TIME 任务开始执行时间

END_TIME 任务执行完成时间

STATE 任务执行状态，R – 正在执行, X - 失败, F - 完成
### 5.任务开发
1. 实现异步任务逻辑类

异步任务开发只需要继承 com.ailk.mq.client.consume.AsyncTaskExecutor 类，实现服务类的抽象方法:

doAsyncTask(String taskId, Map param) throws Exception 即可。
```js
import java.util.Map;
import com.ailk.mq.client.consume.AsyncTaskExecutor;
public abstract class TaskExecutor extends AsyncTaskExecutor {
	@Override
	public void doAsyncTask(String taskId, Map<String, object> param) throws Exception {
		//todo 异步任务业务逻辑
	}
}
```
2. 将异步任务信息配置到异步任务配置表

需要将异步任务信息配置到 base.wd_m_asynctask 异步任务配置表

doAsyncTask(String taskId, Map param) throws Exception 即可。
3. 发起异步任务调用
异步任务调用类：

    com.ailk.mq.client.produce.AsyncTaskMultiCast 组播，集群中所有成员都会收到消息并执行
    com.ailk.mq.client.produce.AsyncTaskClusterCast 按照轮询规则，下发给集群下一实例执行
    com.ailk.mq.client.produce.AsyncTaskUniCast 给指定实例下发异步任务

API:

    AsyncTaskMultiCast.sendAsyncTask(String taskId, Map param)
    AsyncTaskClusterCast.sendAsyncTask(String taskId, Map param)
    AsyncTaskUniCast.sendAsyncTask(String taskId, Map param)

## 导入导出功能

导入导出功能是在异步任务调度的技术上实现的二次开发和封装，它结合了文件上传下载，异步任务调度，Excel 解析生成的功能。
### 1.功能说明
![IE8IconPosition](../static/img/doc/5_2-1.png)

- 前台用户将数据文件上传到文件服务器，并生成 file 表的文件记录
- 上传完成后，前台进程根据对应的导出任务标识发起导入异步任务调用，同时将上传的数据文件标识、数据解析配置xml文件路径和其它业务参数等信息作为异步任务执行参数发送给导入任务执行进程。
- 导入任务执行进程响应任务执行，并获得传入的参数，从文件服务器获取数据文件，并根据数据解析配置文件的配置解析数据文件中的数据，得到要执行导入的数据集。
- 导入任务执行导入的业务逻辑，调用业务服务将数据集入库或进行其它处理。
- 导入任务执行完成后，更新导入日志状态，如果有部分数据导入失败，则生成导入失败的数据文件并上传到文件服务器。
- 前台检测导入任务执行完成，提示用户成功或失败，并将部分失败的数据文件提供给用户下载。
  ![IE8IconPosition](../static/img/doc/5_2-2.png)
- 前台用户输入要导出生成的文件名并选择文件类型，并执行导出。
- 前台进程发起导出异步任务的调用，同时将数据解析配置xml文件路径和其它业务参数信息作为异步任务执行参数发送给导出执行进程。
- 导出执行进程响应异步任务执行，运行导出的业务逻辑，从数据源获得要导出的数据集。
- 导出任务结合数据集和数据解析配置 xml 文件，生成导出后的数据文件，并上传到文件服务器，生成文件记录。
- 导出任务执行完成后生成数据文件下链接，并更新导出日志的状态。
- 前台检测导出任务执行完成，将数据文件下载链接提供给用户下载。

### 2.相关配置

导入导出功能涉及的文件上传下载配置，异步任务配置参考前面相关功能说明部分的配置说明

数据解析配置 xml 文件：

数据解析配置 xml 文件是解析和生成 Excel 数据文件的关键配置，它定义了数据文件中的列和数据集中的列的字段对应关系，以及数据类型，数据校验规则。
```html
<?xml version="1.O" encoding="UTF-8"?>
<workbook>
      <sheet tag="bigcustimp" desc="大客户导入">
		<header isshow="true" height="300">
			<cell
				name="BIG_CUSI_ID"
				desc="大客户标识"
				type="1"
				nullable="no"
				align="1"
				width="5000"
				/>
			<cell
				name="CUST_ID"
				desc="客户标识"
				type="1"
				nullable="no"
				align="1"
				width="5000"
				/>
			<cell
				name="CUSI_NAME"
				desc="客户名称"
				type="1"
				nullable="yes"
				align="1"
				width="5000"
				/>
		</header>
	</sheet>
</workbook>
```

sheet 对应 Excel 文件的一个 sheet 标签，sheet 的 tag 用来区分不同的标签数据，sheet 的 desc 用于生成 sheet 的名称。

header 对应 Excel 中的表头，header 下的每一个 cell 表示一列

cell 的属性配置如下：

- name 表示字段名，和数据集中的字段名对应
- desc 表示列名，生成 Excel 文件时以此作为表头的列名
- type 为列数据类型，1 – String, 2 – Numeric, 3 – DateTime
- nullable 为导入时的校验属性，指定该列数据是否允许为空
- align 为单元格横向排列方式, 1 – left，2 – center， 3 – right

format 表格列数据格式（如果是数字，格式为 0.00,#.## 类似数据格式形式，如果为时间，格式为 yyyy-MM-dd 类似形式）

- equsize	输入值的定长位数限制，如 equsize="10"
- minsize	输入值的最小位数限制，如 minsize="10"
- maxsize	输入值的最大位数限制，如 maxsize="100"

数据解析配置文件位于应用工程的 config/export和config/import 目录下，应用发布后位于 WEB-INF/classes/export 和  WEB-INF/classes/import 目录下。

### 3.日志记录

导入导出任务执行会将日志记录在 base.wd_m_impexp_log 表中：
![IE8IconPosition](../static/img/doc/5_2-4.png)

base.wd_m_impexp_log 表

其中：

- LOG_ID 是日志标识
- FILE_ID 是导入的数据文件或导出生成的数据文件标识
- TASK_ID 是对应的导入导出异步任务标识
- LOG_PROG 是导入导出执行进度百分比值
- LOG_STATUS 是导入导出任务执行状态，ok 为成功，error 为异常
- ERROR_INFO 是导入导出的异常信息，若为导入任务，且执行成功，则会记录成功的数据条数和失败的数据条数，用逗号分隔。
- ERROR_STACK 为异常堆栈记录
- CREA_STAFF_ID 为任务调度发起操作员工号
- CREA_TIME 为任务调度发起时间

### 4.组件使用置

导入组件：
```html
<!-- @Import 组件为按钮下拉形式，通常放在功能按钮区域-->
<span name="myImport"
      taskId="999999999"
	cond=" queryPart"
	jwcid="@Import"
      configFile="import/task.xml"
      templateFile="ognl:'examples/template/' + urlEncode('任务模板') + '.xls'"
	ftpCode="CustomerCentre">
</span>
```

其中：

- taskId 为执行导入的异步任务标识
- cond 为业务参数区域 id
- configFile 为数据解析配置 xml 文件，使用相对路径配置
- templaeFile 为目标下载路径
- ftpCode 为文件上传下载FTP配置

导出组件：
```html
<!-- @Export 组件为按钮下拉形式，通常放在功能按钮区域 -->
<span jwcid="@Export"
	name="myExport"
	taskId="999999998"
	configFile="export/task.xml"
	ftpCode="CustomerCentre">
</span>
```
其中：

- taskId 为执行导出的异步任务标识
- configFile 为数据解析配置 xml 文件，使用相对路径配置
- ftpCode 为文件上传下载FTP配置
- 更多组件详细用法，请参考在线开发文档

### 5.常见问题

日期时间格式问题：

导入导出对于日期类型的数据，对于年月日部分只支持 yyyy-MM-dd 格式日期的读写，导入的数据文件中日期类型单元格数据和数据解析配置 xml 文件中的日期时间列配置要注意单元格格式和日期 format 格式配置。

数据校验错误信息：

对于导入过程中数据行的校验错误信息输出，需要在数据解析配置 xml 文件中定义固定一列 IMPORT_ERROR 来生成到导入失败的数据文件中
```html
<cell
	name="TMPORT_ERROR"
	desc="导入错误"
	type="1"
	align="1"
	width="10000"
	/>	
```