<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Nexus Non-Bulk Email API OR</name>
   <tag></tag>
   <elementGuidId>c7e3f4d1-341f-49b2-9ca4-57905ab7156b</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;name\&quot;: \&quot;Testcampaign1\&quot;,\n    \&quot;description\&quot;: \&quot;description\&quot;,\n    \&quot;emailBody\&quot;: \&quot;This is a test email from Beeru\&quot;,\n    \&quot;category\&quot;: \&quot;TXN\&quot;,\n    \&quot;emailFromAddr\&quot;:\&quot;test12456@eande-engagex.com\&quot;,\n    \&quot;emailToAddr\&quot;: \&quot;yusuf.babiker0@gmail.com\&quot;,\n    \&quot;subject\&quot;: \&quot;Test email non bulk \&quot;,\n    \&quot;displayName\&quot;: \&quot;CpaasEmail1\&quot;,\n    \&quot;clientTxnId\&quot;:\&quot;7112345678901236\&quot;,\n    \&quot;sourceType\&quot; : \&quot;1\&quot;,\n    \&quot;expDuration\&quot;: \&quot;1d\&quot;\n}&quot;,
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
      <webElementGuid>1c68e76e-ad02-482f-b988-5c5a5b2c3156</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${authToken}</value>
      <webElementGuid>a3985d22-34dd-455e-8839-ef24316a2015</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.1.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://nexus.eandenterprise.com/api/v1/email/send</restUrl>
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
      <id>fdc8d83b-6de5-4a25-b6ca-1d18f3b15f4f</id>
      <masked>false</masked>
      <name>authToken</name>
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
