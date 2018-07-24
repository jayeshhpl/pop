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
import sun.util.logging.resources.logging as logging

WebUI.openBrowser('')

WebUI.navigateToUrl('https://www.popssoftware.com/')

WebUI.click(findTestObject('Object Repository/Page_Popsoftware/Page_Popsoftware/a_Sign In'))

WebUI.setText(findTestObject('Object Repository/Page_Popsoftware/Page_Popsoftware  Popsoftware/input_email (1)'), 'company_new_1@yopmail.com')

not_run: WebUI.setEncryptedText(findTestObject('Object Repository/Page_Popsoftware/Page_Popsoftware  Popsoftware/input_password'), 
    '9NLz+4tGZcQ=')

WebUI.click(findTestObject('Object Repository/Page_Popsoftware/Page_Popsoftware  Popsoftware/div_Password'))

WebUI.setEncryptedText(findTestObject('Object Repository/Page_Popsoftware/Page_Popsoftware  Popsoftware/input_password'), 
    'IAVjK6XtNsc6U8iSanTxMg==')

WebUI.click(findTestObject('Object Repository/Page_Popsoftware/Page_Popsoftware  Popsoftware/button_Login'))

not_run: boolean welcometodashboard = WebUI.verifyElementPresent(findTestObject('Object Repository/Page_Popsoftware/Page_Welcome to HPL Site  HPL Site/strong_HPL Site'), 
    20, FailureHandling.CONTINUE_ON_FAILURE)

