<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>ECM Submit Bulk SMS API QR</name>
   <tag></tag>
   <elementGuidId>105e5ec2-fd7f-44b4-bf89-e9323267e7e1</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;msgCategory\&quot;: \&quot;4.5\&quot;,\n    \&quot;contentType\&quot;: \&quot;3.7\&quot;,\n    \&quot;senderAddr\&quot;: \&quot;engageX\&quot;,\n    \&quot;dndCategory\&quot;: \&quot;Campaign\&quot;,\n    \&quot;priority\&quot;: 1,\n    \&quot;clientTxnId\&quot;: \&quot;112346587963\&quot;,\n    \&quot;schTime\&quot;: \&quot;\&quot;,\n    \&quot;expiryDt\&quot;: \&quot;\&quot;,\n    \&quot;desc\&quot;: \&quot;This is the description for campaign\&quot;,\n    \&quot;campaignName\&quot;: \&quot;test campaign\&quot;,\n    \&quot;wapUrl\&quot;: \&quot;www.mywebsite.com\&quot;,\n    \&quot;jobId\&quot;: \&quot;2111546927702796\&quot;,\n    \&quot;jobCost\&quot;: \&quot;1000.00\&quot;,\n    \&quot;recipientCnt\&quot;: \&quot;1\&quot;,\n    \&quot;recipients\&quot;: [\n        \&quot;971554573936\&quot;\n    ],\n    \&quot;recipientFileUrl\&quot;: \&quot;\&quot;,\n    \&quot;msg\&quot;: {\n        \&quot;en\&quot;: \&quot;Health Check: Bulk SMS Automated Test\&quot;,\n        \&quot;ar\&quot;: \&quot; \&quot;\n    },\n    \&quot;dr\&quot;: \&quot;1\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${jwt_token}</value>
      <webElementGuid>96b4eb01-80ef-49b8-92be-5ad85f551e04</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>69138618-26af-4aa6-9515-04ebe0472d34</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.1.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://smartmessaging.etisalat.ae:5676/campaigns/submissions/sms/b/1</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
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
