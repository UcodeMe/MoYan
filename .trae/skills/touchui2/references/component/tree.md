# 树（c_tree / @Tree）
以直观的方式展示、控制多级分支结构构成的数据。

## JWC 静态树方式

<a href="#/example/component/tree" target="_blank">查看演示</a>


```html
<div jwcid="@Tree"
	name="myTree" 
	expandPath="root" 
	checkBoxType="radio" 
	listener="loadTreeData" 
	params="ognl:info"
	oncheckBoxAction="return window.confirm('确定选中/取消当前项吗？')"
></div>
```

 ## JWC 动态树方式

```html
<div jwcid="@Tree"
	name="myTree"
	expandPath="HNAN"
	async="true"
	listener="loadSimpleTree"
	params="ognl:info"
></div>	
```

## JS 方式

html
```html
<div id="myTree" class="c_tree"></div>
```
js
```js
// 新建 myTree
window["myTree"] = new Wade.Tree("myTree")

// 设置数据
window["myTree"].data = {
	"root": {
		"text": "root", "showcheck": "true", "id": "root", "haschild": "true", "order": "0", "complete": "false", "value": "root", "expand": "false", "groupid": null, "dataid": "root", "href": null, "checked": "false", "disabled": "false",
		"childNodes": {
			"node": {
				"text": "班级视图", "showcheck": "true", "id": "node", "haschild": "true", "order": "0", "complete": "false", "value": "node", "expand": "false", "groupid": null, "dataid": "root●node", "href": null, "checked": "false", "disabled": "false",
				"childNodes": {
					"node_teacher": {
						"text": "班级教师", "showcheck": "true", "id": "node_teacher", "haschild": "false", "order": "4", "complete": "false", "value": "node_teacher", "expand": "false", "groupid": null, "dataid": "root●node●node_teacher", "href": null, "checked": "false", "disabled": "false"
					}
				}
			}
		}
	}
};

// 初始化 myTree
window["myTree"].init();
```



Java
```
public void loadTreeData(IRequestCycle cycle) {

	/** 定义树 */
	TreeItem root1 = new TreeItem("root", null, "root", null ,true);
	/** 定义根节点 */
	TreeItem node1 = new TreeItem("node", root1, "班级视图", null ,true);
	/** 定义根节点下的子节点1 */
	TreeItem nod_desktop1 = new TreeItem("node_desktop", node1, "班级首页", null,null ,true);
	/** 定义子节点1下的子节点 */
	new TreeItem("nod_desktop_1", nod_desktop1, "首页节点 1", null,null ,true);
	new TreeItem("nod_desktop_2", nod_desktop1, "首页节点 2", null,null ,true);
	new TreeItem("nod_desktop_3", nod_desktop1, "首页节点 3", null,null ,true);

	/** 定义根节点下的子节点1 */
	TreeItem node_about = new TreeItem("node_about", node1, "班级概况", null,null ,true);
	/** 定义子节点1下的子节点 */
	new TreeItem("node_about_1", node_about, "班级概况 1", null,null ,true);
	new TreeItem("node_about_2", node_about, "班级概况 2", null,null ,true);
	new TreeItem("node_about_3", node_about, "班级概况 3", null,null,true);

	/** 定义根节点下的其他子节点 */
	new TreeItem("node_basic", node1, "基本信息", null,null ,true);
	new TreeItem("node_staff", node1, "班级学员", null,null ,true);
	new TreeItem("node_teacher", node1, "班级教师", null,null ,true);
	new TreeItem("node_curricula", node1, "班级课程",null,null ,true);
	new TreeItem("node_datum", node1, "班级资料",null,null ,true);
	new TreeItem("node_paper", node1, "班级试卷", null,null ,true);
	new TreeItem("node_exam", node1, "班级考卷",null,null,true);
	new TreeItem("node_result", node1, "成绩统计",null,null ,true);
	
	TreeParam param = TreeParam.getTreeParam(cycle);

	Map treeData = TreeFactory.buildTreeData(param, new TreeItem[]{root1});

	setAjax(treeData);
}



public void loadSimpleTree(IRequestCycle cycle) throws Exception {	

	TreeParam param = TreeParam.getTreeParam(cycle);
	
	String parentId = param.getParentNodeId();
	String dataId = param.getParentDataId();
	
	/** 若没有上级节点，表示第一次载入，此时初始根节点 */
	if (parentId == null) {	
		
		IData root = new DataMap();
		String rootId = "HNAN";
		
		root.put("AREA_CODE", rootId);
		root.put("AREA_TEXT", "全省");		

		/** 根据 rootId 获取子节点数量 */
		root.put("NODE_COUNT", 1);
		
		setAjax( TreeFactory.buildTreeData(param, new DatasetList(root), "AREA_CODE", "AREA_TEXT", "NODE_COUNT"));		
		
	} else {
	
		int nodeCount = 1;
		if(dataId != null && !"".equals(dataId) && dataId.split("" + TreeParam.getDataSeparatorChar()).length > 3 ){
			nodeCount = 0;
		}
		
		IDataset ds = new DatasetList();
		
		for(int i = 0; i < 10; i ++){

			IData d = new DataMap();
			d.put("AREA_CODE", i);
			d.put("AREA_TEXT", "区域" + i);
			
			d.put("NODE_COUNT", nodeCount);
			
			ds.add(d);
		}

		setAjax(TreeFactory.buildTreeData(param, ds, "AREA_CODE", "AREA_TEXT", "NODE_COUNT", true));		
	}	
}
```

## JWC 方式属性

| 参数名       | 参数类型         | 是否必须 | 默认值     | 描述                                                         |
| ------------ | ---------------- | -------- | ---------- | ------------------------------------------------------------ |
| name         | java.lang.String | 是       | 无         | 组件实例名，同一个页面中不能重复                             |
| class        | java.lang.String | 否       | 无         | 自定义样式名                                                 |
| style        | java.lang.String | 否       | 无         | 自定义样式                                                   |
| page         | java.lang.String | 否       | 无         | 树组件加载数据的页面 pageName                                |
| listener     | java.lang.Object | 否       | 无         | 树组件加载数据的页面监听方法                                 |
| component    | java.lang.String | 否       | 无         | 树组件加载数据的组件 Id                                      |
| clazz        | java.lang.String | 否       | 无         | 树组件加载数据的 HttpHandler 类路径                          |
| method       | java.lang.String | 否       | 无         | 树组件加载数据的 HttpHandler 方法名                          |
| showCheckBox | boolean          | 否       | true       | 是否显示复选或单选框                                         |
| checkBoxName | java.lang.String | 否       | 无         | 复选或单选框的 name 名称                                     |
| checkBoxType | java.lang.String | 否       | 'checkbox' | 选框类型，默认为 checkbox，可设置为单选框 radio              |
| async        | boolean          | 否       | false      | 是否异步加载每层树节点数据，默认为 false                     |
| params       | java.lang.Object | 否       | 无         | 加载树数据时携带的参数，可以是 String或Map<String, Object>类型 |
| expandPath   | java.lang.String | 否       | 无         | 默认展开路径，必须是每层树节点id用 '-' 符号分隔的形式        |
| init         | boolean          | 否       | true       | 是否初始化，默认为 true                                      |
| afterAction         | java.lang.String          | 否       | 无       |  显示时执行的方法                                      |


## JS API

绑定节点文本点击事件

```js
$("#myTree").textAction(function(e, nodeData){
	alert(new $.DataMap(nodeData));
	return true;
});
```

绑定选框选择事件

```js
$("#myTree").checkBoxAction(function(e, nodeData){
	return window.confirm("确定选中/取消当前项吗？");
});
```

绑定展开按钮点击事件

可以设置异步加载时的特定参数 					

```js
$("#myTree").expandAction(function(e, nodeData){
	myTree.setParam("AAA", "Value");
});
```

新增树节点

```js
// API:
myTree.append(id, text, value, showcheck, pdataid);

// 示例：
myTree.append("append_id", "新增", null, true, "root");
myTree.append({"id": "append_id", "text": "新增", "showcheck": "true"}, "root")
```

| 参数名    | 参数类型 | 是否必须 | 默认值 | 描述           |
| --------- | -------- | -------- | ------ | -------------- |
| id        | string   | 是       | 无     | 节点标识       |
| text      | string   | 是       | 无     | 节点文本       |
| value     | string   | 是       | 无     | 节点值         |
| showcheck | boolean  | 否       | true   | 是否显示选择框 |
| padataid  | string   | 是       | 无     | 父节点 dataid  |

删除树节点

可以结合textAction删除选中的树节点 					

```js
myTree.remove("root");
```

清空所有树节点

可以结合textAction删除选中的树节点 					

```js
myTree.empty();
```

重载数据

```js
myTree.setParam(key, val); // 设置树数据加载时的额外参数
myTree.empty(); // 清空树
myTree.expandPath = "root"; 指定expandPath
myTree.init();  // 重新加载树
```

展开到某个节点路径

```js
myTree.expandByPath(path, separator, callback); //path:节点id路径，separator:节点id分隔符，默认为'-', callback:展开完成后回调函数

例如：
myTree.expandByPath('root_node1_node2', '_');
```

## 样式

| 参数名          | 值              | 默认值 | 描述     |演示     |
| --------------- | --------------- | ------ | -------- |--- |
| c_tree-border   | c_tree-border   | 无     | 边框设置 |<a href="#/example/component/tree-border" target="_blank">查看</a>|


