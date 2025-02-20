<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Create Issue</name>
   <tag></tag>
   <elementGuidId>db561171-d146-4bf6-80c7-1b2f539d9ea0</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;&quot;,
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
      <webElementGuid>02f84628-3725-4828-af35-5b0cffbbce00</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic YWFiYWtlckBlYW5kLmNvbTpBVEFUVDN4RmZHRjBBWDBxUVVEZFlENnBsVW5SUjhtTDNUM1FuQlRsTGNIMzVZNHQ5SnZDem0zX3VNZEpvSVV6cF9KMlZxNVMwS19PM3NPd0R0Q1RaRHJoM3JyUVItR0ZEaUY2SVpfSlJjeWIwNTJlUjJ4a2V2TjdQekc5MGFSb1k0dXIyYTY2Tks0T1dEYW9zWkk2NlBIS0JTQUVnYS1RVk9vLWVfSHJXQV9QU1JBMlBOWFcxSzA9MEFGOTgyOEE=</value>
      <webElementGuid>3b4d4f7d-4e28-4096-945c-82dd4bad8400</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.1.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://eand-enterprise.atlassian.net/rest/servicedeskapi/request</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'${email}:${token}'</defaultValue>
      <description></description>
      <id>dfe92228-0d44-4bd2-9390-65c174c39e48</id>
      <masked>false</masked>
      <name>auth</name>
   </variables>
   <variables>
      <defaultValue>'RFB'</defaultValue>
      <description></description>
      <id>ed0c1513-6c0e-479a-ae27-10a5435f8180</id>
      <masked>false</masked>
      <name>projectKey </name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>5e82bd59-2bd0-4c20-830a-6de35d191d48</id>
      <masked>false</masked>
      <name>summary </name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>e2395587-51eb-4dd3-aaa1-298c34191a19</id>
      <masked>false</masked>
      <name>description </name>
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
