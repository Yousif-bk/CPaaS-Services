<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>KSA Sumbit Bulk SMS API OR</name>
   <tag></tag>
   <elementGuidId>1657c18a-503e-4e62-97cc-3a4c64f0a0e4</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;msg\&quot;: \&quot;Welcome to CPAAS demo setup\&quot;,\n    \&quot;sender\&quot;: \&quot;EandOPS\&quot;,\n    \&quot;category\&quot;: \&quot;TXN\&quot;,\n    \&quot;clientTxnId\&quot;: \&quot;981292172637263762736\&quot;,\n    \&quot;drCallback\&quot;: \&quot;http://172.25.47.198:8088/api/dummy\&quot;,\n    \&quot;name\&quot;: \&quot;bulk api listing 1\&quot;,\n    \&quot;description\&quot;: \&quot;desc\&quot;,\n    \&quot;scheduleType\&quot;: \&quot;oneTime\&quot;,\n    \&quot;expDuration\&quot;: \&quot;2h\&quot;,\n    \&quot;recipients\&quot;: {\n        \&quot;inline\&quot;: [\n            \&quot;971554573936\&quot;\n        ],\n        \&quot;distLists\&quot;: [],\n        \&quot;contacts\&quot;: [],\n        \&quot;groups\&quot;: [],\n        \&quot;files\&quot;: []\n    }\n}\u0027&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>1269383e-160a-4c36-89ea-3f21ed92fb65</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${access_token}</value>
      <webElementGuid>52b5edff-7097-4171-a4ae-149b6e42751f</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.1.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://engagex.eandenterprise.sa/api/v1/campaigns/sms/send</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>b75071b4-6cb5-40a4-8acc-9d06cb75072d</id>
      <masked>false</masked>
      <name>access_token</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
