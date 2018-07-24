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



WebUI.callTestCase(findTestCase('utility/LoginPage_utility'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('Object Repository/DashboardPage/ContactsOBJ/Page_Welcome to HPL Site  HPL Site/a_Contacts'))

WebUI.click(findTestObject('Object Repository/DashboardPage/ContactsOBJ/Page_Welcome to HPL Site  HPL Site/a_Add Contact'))

WebUI.switchToWindowTitle('Contact | HPL Site')

WebUI.selectOptionByValue(findTestObject('Object Repository/DashboardPage/ContactsOBJ/Page_Contact  HPL Site/select_Customer'), 
    'number:0', true)

WebUI.setText(findTestObject('Object Repository/DashboardPage/ContactsOBJ/Page_Contact  HPL Site/input_first_name'), 'hem')

WebUI.setText(findTestObject('Object Repository/DashboardPage/ContactsOBJ/Page_Contact  HPL Site/input_middle_initial'), 
    'b')

WebUI.setText(findTestObject('Object Repository/DashboardPage/ContactsOBJ/Page_Contact  HPL Site/input_last_name'), 'baggins')

WebUI.setText(findTestObject('Object Repository/DashboardPage/ContactsOBJ/Page_Contact  HPL Site/input_email'), 'hem@yopmail.com')

WebUI.selectOptionByValue(findTestObject('Object Repository/DashboardPage/ContactsOBJ/Page_Contact  HPL Site/select_Home'), 
    'number:6', true)

WebUI.setText(findTestObject('Object Repository/DashboardPage/ContactsOBJ/Page_Contact  HPL Site/input_phone_number'), '3434343434343')

WebUI.setText(findTestObject('Object Repository/DashboardPage/ContactsOBJ/Page_Contact  HPL Site/input_ext'), '21')

WebUI.setText(findTestObject('Object Repository/DashboardPage/ContactsOBJ/Page_Contact  HPL Site/input_address'), '123rd street')

WebUI.setText(findTestObject('Object Repository/DashboardPage/ContactsOBJ/Page_Contact  HPL Site/input_apt_suite_no'), '4th')

WebUI.click(findTestObject('Object Repository/DashboardPage/ContactsOBJ/Page_Contact  HPL Site/input_is_billing_same'))

WebUI.click(findTestObject('Object Repository/DashboardPage/ContactsOBJ/Page_Contact  HPL Site/span_Save  Next'))

Thread.sleep(5000)

WebUI.selectOptionByValue(findTestObject('Object Repository/DashboardPage/ContactsOBJ/Page_Contact  HPL Site/select_SalesNew CategoryPlumbe'), 
    'number:60', true)

WebUI.setText(findTestObject('Object Repository/DashboardPage/ContactsOBJ/Page_Contact  HPL Site/input_company_name'), 'ABCD')

WebUI.setText(findTestObject('Object Repository/DashboardPage/ContactsOBJ/Page_Contact  HPL Site/input_job_title'), 'Tester')

WebUI.click(findTestObject('Object Repository/DashboardPage/ContactsOBJ/Page_Contact  HPL Site/span_Submit'))

WebUI.click(findTestObject('Object Repository/DashboardPage/ContactsOBJ/Page_Contact  HPL Site/button_OK'))

WebUI.closeBrowser()

