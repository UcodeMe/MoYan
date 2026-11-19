# 表格之中如何实现缩略文字

## 问题分析
在表格中经常会出现这样的情况，单元格的文字过多，显示的文字字段过长，为了使表格更具有可读性，需要在此处缩略文字。

缩略前
![IE8IconPosition](../static/img/doc/tablelong.png) <br/>

缩略后
![IE8IconPosition](../static/img/doc/tableEllipsis.png) <br/>

## 解决方案

在对应字段位置加上 ``e_ellipsis`` 样式参数，注意：是加在 ``td > span.e_ellipsis`` 位置,宽度在 span 上加 style。

例子：

```html
<div class="c_table c_table-border c_table-lite">
	<table>
		<thead>
			<tr>
				<th>thead</th>
				<th>thead</th>
				<th>thead</th>
				<th>thead</th>
				<th>thead</th>
				<th>thead</th>
				<th>thead</th>
				<th>thead</th>
				
			</tr>
		</thead>
		<tbody>
			<tr>
				<td>cell</td>
				<td><span style="width:10em;" class="e_ellipsis">cellcellcellcellcellcellcellcellcellcellcellcellcellcellcell</span></td>
				<td>cell</td>
				<td>cell</td>
				<td>cell</td>
				<td>cell</td>
				<td>cell</td>
				<td>cell</td>
			</tr>
		</tbody>
	</table>
</div>
```


