# 前台-统一会话管理

## 实现原理
![IE8IconPosition](../static/img/doc/3_1-1.png) 

- 统一会话的基本原理是在过滤器中自定义实现 Request 请求对象中的 createSession 和 getSession 实现，将会话数据存到远端分布式缓存中，实现 WEB 应用无状态化。
- 自定义 HttpSession 实现 HttpSession,Serializable 接口，并支持新建和修改判断。
- 每个 HTTP 请求线程创建或从分布式缓存中获取 HttpSession 数据后存放在线程上下文，在 HTTP 请求线程完成后，判断 HttpSession 是否为新建或有修改来决定是否需要做回存处理，避免和缓存的频繁交互。

## 服务部署
统一会话管理需要单独部署会话 Redis 缓存服务，由于会话数据属于关键性数据，在生产环境进行会话 Redis 缓存部署时，通常采取双中心进程做主从互备，并在主节点故障时自动切换到备节点。

测试开发环境只需部署单节点即可。

![IE8IconPosition](../static/img/doc/3_1-2.png) 

双中心主备节点同步

- 正常情况下，Web 应用连接到两个中心的主节点，会话数据根据会话标识 HASH 散列均匀分布在8个主节点进程上
- 当主节点故障时，Web 应用与主节点心跳检测失败，自动切换连接到主节点对应的备节点。
- 每个 HTTP 请求线程创建或从分布式缓存中获取 HttpSession 数据后存放在线程上下文，在 HTTP 请求线程完成后，判断 HttpSession 是否- 为新建或有修改来决定是否需要做回存处理，避免和缓存的频繁交互。 								

## 相关配置

### 1.会话选项配置

会话选项配置在 WEB 应用的 sna.properties 配置文件中，配置内容如下：
```js
sna.app.name=NGBOSS
sna.session.cache=redis
sna.session.timeout=14400
```

其中：

- sna.app.name 是接入的系统名，只需要在 NGBOSS 外框应用配置
- sna.session.cache 是使用的缓存类型，固定配置为 redis
- sna.session.timeout 为会话超时时间，单位为秒，默认不配置为180秒，若要配置其它值，则需保持每个 WEB 应用下的配置一致。

### 2.Web过滤器配置

WEB 应用需要配置统一会话管理的过滤器
```html

<!--前台应用SNA单点登录 Session处理Filter-->
<filter>
	<filter-name>SessionFilter</filter-name>
	<filter-class>com.ailk.sna.filter.SessionFilter</filter-class>
</filter>

<filter-mapping>
	<filter-name>SessionFilter</filter-name>
	<servlet-name>quickstart</servlet-name>
</filter-mapping>

<filter-mapping>
	<filter-name>SessionFilter</filter-name>
	<servlet-name>impexp</servlet-name>
</filter-mapping>

<filter-mapping>
	<filter-name>SessionFilter</filter-name>
	<servlet-name>attach</servlet-name>
</filter-mapping>

<filter-mapping>
	<filter-name>SessionFilter</filter-name>
	<servlet-name>handler</servlet-name>
</filter-mapping>
```


统一会话管理过滤器要配置在其它 WEB 过滤器特别是 SessionVerifyFilter 过滤器之前。filter-mapping 配置需要做统一会话处理的 servlet-name。
### 3.缓存服务连接配置

缓存服务连接配置在 WEB 应用的 redis.xml 配置文件中

开发测试环境配置：
```html
<?xml version='1.O' encoding='UIF-8'?>
<redis>
	<default-datacenter>default-datacenter</default-datacenter>
	<datacenter name="default-datacenter">
		<cluster name="sna" desc="会话缓存">
			<heartbeat-second>2</heartbeat-second>
			<pool-size>5</pool-size>
			<address master="10.220.67.211:6379" />
		</cluster>
	</datacenter>
</redis>
```
生产环境配置：

```html

<?xml version= '1.O' encoding='UIF-8'?>
<redis>
	<default-datacenter>default-datacenter</default-datacenter>
	<datacenter name="default-datacenter">
		<cluster name="sna" desc="会话缓存">
			<heartbeat-second>5</heartbeat-second>
			<pool-size>2</pool-size>
			<address master="10.143.34.28:13001" slave="10.143.35.28:13011" />
			<address master="10.143.34.28:13002" slave="l0.143.35.28:13012" />
			<address master="10.143.34.28:13003" slave="10.143.35.28:13013" />
			<address master="10.143.34.28:13004" slave="10.143.35.28:13014" />
			<address master="10.143.34.29:13001" slave="l0.143.35.29:13011" />
			<address master="10.143.34.29:13002" slave="10.143.35.29:13012" />
			<address master="10.143.34.29:13003" slave="10.143.35.29:13013" />
			<address master="10.143.34.29:13004" slave="10.143.35.29:13014" />
			<address master="10.143.35.28:13001" slave="10.143.34.28:13011" />
			<address master="10.143.35.28:13002" slave="10.143.34.28:13012" />
			<address master="10.143.35.28:13003" slave="10.143.34.28:13013" />
			<address master="10.143.35.28:13004" slave="l0.143.34.28:13014" />
			<address master="10.143.35.29:13001" slave="10.143.34.29:13011" />
			<address master="10.143.35.29:13002" slave="10.143.34.29:13012" />
			<address master="10.143.35.29:13003" slave="l0.143.34.29:13013" />
			<address master="10.143.35.29:13004" slave="10.143.34.29:13014" />
		</cluster>
	</datacenter> 
</redis>
```
