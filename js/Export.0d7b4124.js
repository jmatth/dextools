(window["webpackJsonp"]=window["webpackJsonp"]||[]).push([["Export"],{6595:function(t,e,a){"use strict";var o=a("a840"),i=a.n(o);i.a},a415:function(t,e,a){"use strict";a.r(e);var o=function(){var t=this,e=t.$createElement,a=t._self._c||e;return a("v-container",[a("v-row",[a("v-col",{attrs:{cols:"12"}},[a("ExportDialog")],1)],1)],1)},i=[],s=a("276c"),n=a("920b"),r=a("92a6"),c=a("9ab4"),l=function(){var t=this,e=t.$createElement,a=t._self._c||e;return a("v-card",[a("v-card-title",{attrs:{"primary-title":""}},[t._v(" Export ")]),a("v-divider"),a("v-card-text",[a("v-text-field",{attrs:{label:"Enter your name",value:t.userName},on:{input:t.updateUserName}}),a("p",[t._v(" Use the buttons below to export to ICS (for importing into calendar applications) or email (to register for events). For email, you can also copy the text below. ")]),t.disableEmail?t._e():a("span",[t._v("To: "),a("a",{attrs:{href:"mailto:"+t.conEmail}},[t._v(t._s(t.conEmail))])]),a("v-divider",{staticStyle:{"margin-bottom":"5px","margin-top":"2px"}}),a("pre",{ref:"emailText",staticClass:"email-text"},[t._v(t._s(t.emailText))])],1),a("v-divider"),a("v-card-actions",[a("v-spacer"),a("v-btn",{staticClass:"mr-2",on:{click:function(e){return t.$store.state.agenda.exportIcs(t.$store.state.conName)}}},[t._v(" ICS ")]),a("v-tooltip",{attrs:{top:"",disabled:!t.disableEmail},scopedSlots:t._u([{key:"activator",fn:function(e){var o=e.on;return[a("a",t._g({attrs:{href:t.mailtoLink}},o),[a("v-btn",{staticClass:"mr-2",attrs:{disabled:t.disableEmail},on:{click:function(e){t.exportDialog=!1}}},[t._v(" Email ")])],1)]}}])},[a("span",[t._v(t._s(this.$store.state.conName)+" is not accepting event registrations at this time.")])]),a("v-tooltip",{attrs:{top:""},scopedSlots:t._u([{key:"activator",fn:function(e){var o=e.on;return[a("v-btn",t._g({on:{click:t.copyEmailText}},{on:o}),[t._v(" Copy ")])]}}]),model:{value:t.shouldShowCopyMessage,callback:function(e){t.shouldShowCopyMessage=e},expression:"shouldShowCopyMessage"}},[a("span",[t._v(t._s(this.copyMessage))])])],1)],1)},u=[],p=(a("99af"),a("13d5"),a("e954")),d=a("60a3"),m=a("2ef0"),v=function(t){Object(n["a"])(a,t);var e=Object(r["a"])(a);function a(){var t;return Object(s["a"])(this,a),t=e.apply(this,arguments),t.exportDialog=!1,t.shouldShowCopyMessage=!1,t.copyMessage="",t.userName="",t}return Object(p["a"])(a,[{key:"created",value:function(){this.userName=this.$store.state.userName,this.updateUserNameStore=Object(m["debounce"])(this.updateUserNameStore,3e3)}},{key:"updateUserName",value:function(t){this.userName=t,this.updateUserNameStore(t)}},{key:"updateUserNameStore",value:function(t){this.$store.commit("setUserName",t)}},{key:"showCopyTooltip",value:function(t){this.copyMessage=t,this.shouldShowCopyMessage=!0,setTimeout(function(){this.shouldShowCopyMessage=!1}.bind(this),5e3)}},{key:"copyEmailText",value:function(){var t=this.$refs.emailText;this.$copyText(this.emailText,t).then(this.onCopy,this.onFailedCopy)}},{key:"onCopy",value:function(){this.showCopyTooltip("Email text copied to clipboard.")}},{key:"onFailedCopy",value:function(){this.showCopyTooltip("Error, please copy the email text manually.")}},{key:"emailText",get:function(){var t=this.userName||"<YOUR NAME>";return this.$store.state.agenda.events.reduce((function(t,e){return t+"\n".concat(e.code)}),"Name: ".concat(t,"\n\nEvents:"))}},{key:"conEmail",get:function(){return this.$store.state.conEmail}},{key:"mailtoLink",get:function(){if(this.disableEmail)return"#";var t=encodeURI("".concat(this.$store.state.conName," Registration")),e=encodeURI(this.emailText);return"mailto:".concat(this.conEmail,"?subject=").concat(t,"&body=").concat(e)}},{key:"disableEmail",get:function(){return!this.$store.state.conEmail}}]),a}(d["c"]);v=Object(c["a"])([d["a"]],v);var h=v,f=h,y=(a("6595"),a("2877")),b=a("6544"),x=a.n(b),g=a("8336"),C=a("b0af"),E=a("99d9"),_=a("ce7e"),k=a("2fa4"),w=a("8654"),N=a("3a2f"),T=Object(y["a"])(f,l,u,!1,null,"b7ebd5de",null),S=T.exports;x()(T,{VBtn:g["a"],VCard:C["a"],VCardActions:E["a"],VCardText:E["b"],VCardTitle:E["c"],VDivider:_["a"],VSpacer:k["a"],VTextField:w["a"],VTooltip:N["a"]});var j=function(t){Object(n["a"])(a,t);var e=Object(r["a"])(a);function a(){return Object(s["a"])(this,a),e.apply(this,arguments)}return a}(d["c"]);j=Object(c["a"])([Object(d["a"])({components:{ExportDialog:S}})],j);var O=j,$=O,V=a("62ad"),U=a("a523"),M=a("0fd9"),D=Object(y["a"])($,o,i,!1,null,null,null);e["default"]=D.exports;x()(D,{VCol:V["a"],VContainer:U["a"],VRow:M["a"]})},a840:function(t,e,a){}}]);
//# sourceMappingURL=Export.0d7b4124.js.map