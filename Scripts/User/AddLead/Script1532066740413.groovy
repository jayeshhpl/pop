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

WebUI.click(findTestObject('DashboardPage/LeadsOBJ/Page_Edit user  HPL Site/a_Leads'))

WebUI.click(findTestObject('DashboardPage/LeadsOBJ/Page_Edit user  HPL Site/a_Add Lead'))

WebUI.switchToWindowTitle('Leads | HPL Site')

WebUI.selectOptionByValue(findTestObject('DashboardPage/LeadsOBJ/Page_Leads  HPL Site/select_Customer'), 'number:0', true)

WebUI.setText(findTestObject('DashboardPage/LeadsOBJ/Page_Leads  HPL Site/input_first_name'), 'jay')

WebUI.setText(findTestObject('DashboardPage/LeadsOBJ/Page_Leads  HPL Site/input_middle_initial'), 'D')

WebUI.setText(findTestObject('DashboardPage/LeadsOBJ/Page_Leads  HPL Site/input_last_name'), 'loffman')

WebUI.setText(findTestObject('DashboardPage/LeadsOBJ/Page_Leads  HPL Site/input_email'), 'jay@yopmail.com')

WebUI.selectOptionByValue(findTestObject('DashboardPage/LeadsOBJ/Page_Leads  HPL Site/select_Home'), 'number:6', true)

WebUI.setText(findTestObject('DashboardPage/LeadsOBJ/Page_Leads  HPL Site/input_phone_number'), '43434343434')

WebUI.setText(findTestObject('DashboardPage/LeadsOBJ/Page_Leads  HPL Site/input_ext'), '55')

WebUI.setText(findTestObject('DashboardPage/LeadsOBJ/Page_Leads  HPL Site/input_address'), '123th street')

WebUI.setText(findTestObject('DashboardPage/LeadsOBJ/Page_Leads  HPL Site/input_apt_suite_no'), '4th')

WebUI.setText(findTestObject('DashboardPage/LeadsOBJ/Page_Leads  HPL Site/input_city'), 'Arlington')

WebUI.setText(findTestObject('DashboardPage/LeadsOBJ/Page_Leads  HPL Site/input_state'), 'california')

WebUI.setText(findTestObject('DashboardPage/LeadsOBJ/Page_Leads  HPL Site/input_country_id'), 'United states')

WebUI.setText(findTestObject('DashboardPage/LeadsOBJ/Page_Leads  HPL Site/input_zip'), '12121')

WebUI.click(findTestObject('DashboardPage/LeadsOBJ/Page_Leads  HPL Site/input_is_billing_same'))

WebUI.setText(findTestObject('DashboardPage/LeadsOBJ/Page_Leads  HPL Site/input_lock_box_no'), '1234')

WebUI.setText(findTestObject('DashboardPage/LeadsOBJ/Page_Leads  HPL Site/input_estimate_revenue'), '12')

WebUI.selectOptionByValue(findTestObject('DashboardPage/LeadsOBJ/Page_Leads  HPL Site/select_Head'), 'number:13', true)

WebUI.setText(findTestObject('DashboardPage/LeadsOBJ/Page_Leads  HPL Site/textarea_estimate_description'), 'test')

WebUI.click(findTestObject('DashboardPage/LeadsOBJ/Page_Leads  HPL Site/span_Save  Next'))

Thread.sleep(5000)

WebUI.selectOptionByValue(findTestObject('Object Repository/testobj/Page_Leads  HPL Site/select_SalesNew CategoryPlumbe'),
	'number:243', true)

WebUI.setText(findTestObject('DashboardPage/LeadsOBJ/Page_Leads  HPL Site/input_company_name'), 'samcorp')

WebUI.setText(findTestObject('DashboardPage/LeadsOBJ/Page_Leads  HPL Site/input_job_title'), 'sam')

WebUI.setText(findTestObject('DashboardPage/LeadsOBJ/Page_Leads  HPL Site/input_insurance_company_name'), 'abcd')

WebUI.setText(findTestObject('DashboardPage/LeadsOBJ/Page_Leads  HPL Site/input_insurance_claim_no'), '2112121212')

WebUI.setText(findTestObject('DashboardPage/LeadsOBJ/Page_Leads  HPL Site/input_insurance_policy_no'), '1212121212')

WebUI.click(findTestObject('DashboardPage/LeadsOBJ/Page_Leads  HPL Site/span_Submit'))

Thread.sleep(8000)

WebUI.click(findTestObject('DashboardPage/LeadsOBJ/Page_Leads  HPL Site/button_OK'))

