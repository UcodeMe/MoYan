# 端到端示例：查询表单 + 局部刷新表格

本示例用于串联 TouchUI2 常用链路：响应式表单（含校验）→ 点击查询 → `$.ajax.submit` → 刷新 `@Part` 区域 → `@Foreach` 渲染表格行。

## 页面骨架

```html
<!DOCTYPE HTML>
<html size="s">
<head>
  <meta http-equiv="X-UA-Compatible" content="IE=edge,chrome=1" />
  <meta name="viewport" content="width=device-width, initial-scale=1, minimum-scale=1, maximum-scale=1" />
  <meta http-equiv="Content-Type" content="text/html; charset=UTF-8" />

  <link href="TouchUI/css/base.css" rel="stylesheet" type="text/css" />
  <script type="text/javascript" src="TouchUI/js/jq/jquery-1.9.1.js"></script>
  <script type="text/javascript" src="TouchUI/js/jq/adapter-1.9.1.js"></script>
  <script type="text/javascript" src="TouchUI/js/jcl.js"></script>
  <script type="text/javascript" src="TouchUI/js/i18n/code.zh_CN.js"></script>
  <script type="text/javascript" src="TouchUI/js/jcl-plugins.js"></script>
  <script type="text/javascript" src="TouchUI/js/jcl-ui.js"></script>
  <script type="text/javascript" src="TouchUI/js/jcl-jwc.js"></script>
  <script type="text/javascript" src="assets/js/config.js"></script>
</head>
<body jwcid="@Body">

  <div id="queryArea" class="c_form c_form-col-3 c_form-phone-col-1" jwcid="@Form">
    <div class="c_formItem">
      <span class="c_label">客户号</span>
      <input jwcid="@TextField" type="text" name="CUST_ID" desc="客户号" nullable="no" maxsize="30" datatype="text" />
    </div>
    <div class="c_formItem">
      <span class="c_label">状态</span>
      <span jwcid="@Select" name="STATUS" source="ognl:statusOptions" addDefault="true" textField="text" valueField="value" desc="状态"></span>
    </div>
    <div class="c_formItem">
      <span jwcid="@Button" id="btnQuery" value="查询"></span>
    </div>
  </div>

  <div id="resultPart" jwcid="@Part">
    <table class="c_table" jwcid="@Table" name="resultTable">
      <thead>
        <tr>
          <th col="CUST_ID"><span>客户号</span></th>
          <th col="CUST_NAME"><span>客户名称</span></th>
        </tr>
      </thead>
      <tbody>
        <tr jwcid="@Foreach" source="ognl:rows" value="ognl:row" element="tr">
          <td><span jwcid="@Insert" value="ognl:row.CUST_ID"></span></td>
          <td><span jwcid="@Insert" value="ognl:row.CUST_NAME"></span></td>
        </tr>
      </tbody>
    </table>
  </div>

  <script type="text/javascript" src="assets/js/data.js"></script>
  <script type="text/javascript">
    Wade.setRatio();
    Wade.JWC.init(true);

    $("#btnQuery").tap(function(){
      if(!$.validate.verifyAll($("#queryArea")[0])) return;

      $.ajax.submit(
        "queryArea",
        "query",
        {},
        "resultPart",
        function(resp){},
        function(err){}
      );
    });
  </script>
</body>
</html>
```

## data.js（示意）

```js
var statusOptions = [
  { text: "全部", value: "" },
  { text: "正常", value: "N" },
  { text: "冻结", value: "F" }
];

var rows = [];
```

## 排查要点

- `Wade.setRatio()` 必须在页面可用时执行；`Wade.JWC.init(true)` 用于前端 JWC 解析
- `@Part` 的 id（本例 `resultPart`）需要传入 `$.ajax.submit` 的 partids 参数
- 校验失败时 `$.validate.verifyAll` 返回 false；应停止提交

## 相关文档

- `use/jwc.md`
- `component/form.md`
- `component/table.md`
- `logical/part.md`
- `logical/foreach.md`
- `js-api/ajax.md`
- `js-api/verify.md`
- `js-api/dom.md`

