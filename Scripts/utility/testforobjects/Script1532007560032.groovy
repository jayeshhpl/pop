import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.checkpoint.CheckpointFactory as CheckpointFactory
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as MobileBuiltInKeywords
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testcase.TestCaseFactory as TestCaseFactory
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testdata.TestDataFactory as TestDataFactory
import com.kms.katalon.core.testobject.ObjectRepository as ObjectRepository
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WSBuiltInKeywords
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUiBuiltInKeywords
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable

WebUI.openBrowser('')

WebUI.selectOptionByValue(findTestObject('Object Repository/testobj/Page_Leads  HPL Site/select_SalesNew CategoryPlumbe'), 
    'number:244', true)

WebUI.setText(findTestObject('Object Repository/testobj/Page_Leads  HPL Site/input_company_name'), 'dfsdf')

WebUI.selectOptionByValue(findTestObject('Object Repository/testobj/Page_Leads  HPL Site/select_SalesNew CategoryPlumbe'), 
    'number:198', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/testobj/Page_Leads  HPL Site/select_SalesNew CategoryPlumbe'), 
    'number:134', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/testobj/Page_Leads  HPL Site/select_SalesNew CategoryPlumbe'), 
    'number:60', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/testobj/Page_Leads  HPL Site/select_SalesNew CategoryPlumbe'), 
    'number:59', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/testobj/Page_Leads  HPL Site/select_SalesNew CategoryPlumbe'), 
    'number:243', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/testobj/Page_Leads  HPL Site/select_SalesNew CategoryPlumbe'), 
    'number:245', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/testobj/Page_Leads  HPL Site/select_SalesNew CategoryPlumbe'), 
    'number:246', true)

