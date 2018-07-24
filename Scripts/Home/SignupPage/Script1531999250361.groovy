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

WebUI.navigateToUrl('https://www.popssoftware.com/')

WebUI.click(findTestObject('Object Repository/Page_Popsoftware/a_Sign Up'))

WebUI.setText(findTestObject('Object Repository/Page_Popsoftware  Popsoftware/input_first_name'), 'Sam')

WebUI.setText(findTestObject('Object Repository/Page_Popsoftware  Popsoftware/input_last_name'), 'wilson')

WebUI.setText(findTestObject('Object Repository/Page_Popsoftware  Popsoftware/input_company_name'), 'samcorp')

WebUI.setText(findTestObject('Object Repository/Page_Popsoftware  Popsoftware/input_company_address'), '123')

WebUI.click(findTestObject('Object Repository/Page_Popsoftware  Popsoftware/div_Email'))

WebUI.setText(findTestObject('Object Repository/Page_Popsoftware  Popsoftware/input_email'), 'sam@yopmail.com')

WebUI.setText(findTestObject('Object Repository/Page_Popsoftware  Popsoftware/input_phone_no'), '76767676222')

WebUI.setText(findTestObject('Object Repository/Page_Popsoftware  Popsoftware/input_no_of_employees'), '1000')

WebUI.setEncryptedText(findTestObject('Object Repository/Page_Popsoftware  Popsoftware/input_password'), 'aeHFOx8jV/A=')

WebUI.setEncryptedText(findTestObject('Object Repository/Page_Popsoftware  Popsoftware/input_password_confirmation'), 'aeHFOx8jV/A=')

WebUI.click(findTestObject('Object Repository/Page_Popsoftware  Popsoftware/button_Sign Up'))

WebUI.click(findTestObject('Object Repository/Page_Popsoftware  Popsoftware/div_Verification email sent Pl'))

WebUI.closeBrowser()

