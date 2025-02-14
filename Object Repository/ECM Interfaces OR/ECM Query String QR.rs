<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>ECM Query String QR</name>
   <tag></tag>
   <elementGuidId>1c5a43b6-b99f-42fb-b13b-d80cd60ece34</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <katalonVersion>10.1.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://smartmessaging.etisalat.ae:9095/campaignService/campaigns/qs?msgCategory=4.5&amp;channel=2.1&amp;recipient=971554573936&amp;contentType=3.1&amp;dr=false&amp;msg=Query String Automated Test &amp;user=aabaker&amp;pswd=Yousif@123&amp;dndCategory=Campaign&amp;sender=engageX</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


WS.verifyResponseStatusCode(response, 201)

assertThat(response.getStatusCode()).isEqualTo(201)


assertThat(response.getResponseText()).contains('jobId')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
