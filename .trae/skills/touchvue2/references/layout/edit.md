# 受理（l_edit）
基本的业务受理布局方式，便于自动适应手机界面。

# HTML

<a href="#/example/layout/edit" target="_blank">查看演示</a>

```html
<div class="l_edit">
	<div class="l_editMain">
		<!-- 插入 c_list-form 组件 -->
	</div>
	<div class="l_editPlace"><span class="e_ico-cmlogo"></span></div>
</div>
```
## 符合案例

<a href="#/example/layout/edit-mix" target="_blank">查看演示</a>

```html
<div class="l_edit">
	<div class="c_header">
		<div class="back">页面标题</div>
	</div>
	<div class="l_editMain">
		<div class="c_space"></div>
		<div class="c_list c_list-line c_list-border">
			<ul>
				<li class="link">
					<div class="label">角色ID</div>
					<div class="value"><input type="text" /></div>
				</li>
				<li class="link">
					<div class="label">角色类型</div>
					<div class="value">

						<span jwcid="@Select" name="roleType" source="ognl:IDType" textField="text" valueField="value" addDefault="true" desc="选择项目"></span>
					</div>
				</li>
				<li class="link">
					<div class="label">角色性别</div>
					<div class="value">
						<span name="mySwitch3" jwcid="@Switch" onText="女" disabled="false" offText="男" onColor="red" offColor="blue" value="off"></span>
					</div>
				</li>
				<li class="link" ontap="showPopup('myPopup','myPopup_item',true)">
					<div class="label">角色生活社区（单选）</div>
					<div class="value">
						<input type="text" readonly id="singleNameInput" />
					</div>
					<div class="more"></div>
				</li>
				<li class="link" ontap="showPopup('myPopup','myPopup_item1',true)">
					<div class="label">基础装备配置（多选）</div>
					<div class="value x_to" id="myValue"></div>
					<div class="more"></div>
				</li>
			</ul>
		</div>
		<div class="c_space"></div>
		<div class="c_submit c_submit-full">
			<button type="button" class="e_button-r e_button-l e_button-navy">重置</button>
			<button type="button" class="e_button-r e_button-l e_button-green">提交</button>
		</div>
	</div>
	<div class="l_editPlace"><span class="e_ico-cmlogo"></span></div>
</div>
<div jwcid="@Popup" name="myPopup" class="c_popup c_popup-half">
	<div class="c_popupGroup">
		<div id="myPopup_item" class="c_popupItem">
			<div class="c_header">
				<div class="back" ontap="backPopup(this)">选择生活社区</div>
			</div>
			<div class="c_scroll c_scroll-float c_scroll-header">
				<div class="c_list c_list-line">
					<ul>
						<li class="link" jwcid="@Foreach" source="ognl:community" value="ognl:com" element="li" ontap="pick(this);">
							<div class="main">
								<div class="title"><span jwcid="@Insert" value="ognl:com.text"></span></div>
							</div>
						</li>
					</ul>
				</div>
				<div class="c_line"></div>
			</div>
		</div>
		<div id="myPopup_item1" class="c_popupItem">
			<div class="c_header">
				<div class="back" ontap="backPopup(this)">基础装备配置</div>
			</div>
			<div class="c_scroll c_scroll-float c_scroll-header">
				<div class="l_padding">
					<div class="c_list c_list-border c_list-line c_list-col-2  c_list-phone-col-1 c_list-fixWrapSpace">
						<ul>
							<li class="link" jwcid="@Foreach" source="ognl:equipment" value="ognl:equip" element="li">
								<label class="group link">
									<div class="main">
										<div class="title"><span jwcid="@Insert" value="ognl:equip.text"></span></div>
									</div>
									<div class="fn">
										<input type="checkbox" onclick="this.blur()" onchange="pickPaytype(this);" />
									</div>
								</label>
							</li>
						</ul>
					</div>
					<div class="c_space"></div>
					<div class="c_submit c_submit-full">
						<button type="button" class="e_button-r e_button-l e_button-navy" ontap="reset();">重置</button>
						<button type="button" class="e_button-r e_button-l e_button-blue" ontap="hidePopup(this);">确定</button>
					</div>
					<div class="c_space"></div>
				</div>
			</div>
		</div>
	</div>
</div>
```

## 兼容手机

```html
<!-- 受理模版 开始 -->
<div class="l_edit">
	<div class="c_header e_show-phone">
		<div class="back" ontap="closeNav();">页面标题</div>
	</div>
	<span class="l_editMain">
		...
	</span>
	<span class="l_editPlace">...</span>
</div>
<!-- 受理模版 结束 -->
```

