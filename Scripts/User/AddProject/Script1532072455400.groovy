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

WebUI.click(findTestObject('Object Repository/DashboardPage/ProjectOBJ/Page_Popsoftware/a_Sign In'))

WebUI.setText(findTestObject('Object Repository/DashboardPage/ProjectOBJ/Page_Popsoftware  Popsoftware/input_email'), 'company_new_1@yopmail.com')

WebUI.setEncryptedText(findTestObject('Object Repository/DashboardPage/ProjectOBJ/Page_Popsoftware  Popsoftware/input_password'), 
    'IAVjK6XtNsc6U8iSanTxMg==')

WebUI.click(findTestObject('Object Repository/DashboardPage/ProjectOBJ/Page_Popsoftware  Popsoftware/button_Login'))

WebUI.click(findTestObject('Object Repository/DashboardPage/ProjectOBJ/Page_Welcome to HPL Site  HPL Site/span_Projects'))

WebUI.click(findTestObject('Object Repository/DashboardPage/ProjectOBJ/Page_Welcome to HPL Site  HPL Site/a_Add Projects'))

WebUI.switchToWindowTitle('Projects | HPL Site')

WebUI.selectOptionByValue(findTestObject('Object Repository/DashboardPage/ProjectOBJ/Page_Projects  HPL Site/select_Customer'), 
    'number:0', true)

WebUI.setText(findTestObject('Object Repository/DashboardPage/ProjectOBJ/Page_Projects  HPL Site/input_company_name'), 'abcd')

WebUI.setText(findTestObject('Object Repository/DashboardPage/ProjectOBJ/Page_Projects  HPL Site/input_first_name'), 'val')

WebUI.setText(findTestObject('Object Repository/DashboardPage/ProjectOBJ/Page_Projects  HPL Site/input_middle_initial'), 
    'b')

WebUI.setText(findTestObject('Object Repository/DashboardPage/ProjectOBJ/Page_Projects  HPL Site/input_last_name'), 'bole')

WebUI.setText(findTestObject('Object Repository/DashboardPage/ProjectOBJ/Page_Projects  HPL Site/input_email'), 'val@yop.com')

WebUI.selectOptionByValue(findTestObject('Object Repository/DashboardPage/ProjectOBJ/Page_Projects  HPL Site/select_Home'), 
    'number:7', true)

WebUI.setText(findTestObject('Object Repository/DashboardPage/ProjectOBJ/Page_Projects  HPL Site/input_phone_number'), '54545454554')

WebUI.setText(findTestObject('Object Repository/DashboardPage/ProjectOBJ/Page_Projects  HPL Site/input_ext'), '54')

WebUI.setText(findTestObject('Object Repository/DashboardPage/ProjectOBJ/Page_Projects  HPL Site/input_address'), '123rd Street')

WebUI.setText(findTestObject('Object Repository/DashboardPage/ProjectOBJ/Page_Projects  HPL Site/input_apt_suite_no'), '3rd')

WebUI.setText(findTestObject('Object Repository/DashboardPage/ProjectOBJ/Page_Projects  HPL Site/input_city'), 'new york')

WebUI.setText(findTestObject('Object Repository/DashboardPage/ProjectOBJ/Page_Projects  HPL Site/input_state'), 'new york')

WebUI.setText(findTestObject('Object Repository/DashboardPage/ProjectOBJ/Page_Projects  HPL Site/input_country_id'), 'united states')

WebUI.setText(findTestObject('Object Repository/DashboardPage/ProjectOBJ/Page_Projects  HPL Site/input_zip'), '54545')

WebUI.click(findTestObject('Object Repository/DashboardPage/ProjectOBJ/Page_Projects  HPL Site/input_is_billing_same'))

WebUI.setText(findTestObject('Object Repository/DashboardPage/ProjectOBJ/Page_Projects  HPL Site/input_reference_no'), '43434')

WebUI.selectOptionByValue(findTestObject('Object Repository/DashboardPage/ProjectOBJ/Page_Projects  HPL Site/select_Test Status'), 
    'number:11', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/DashboardPage/ProjectOBJ/Page_Projects  HPL Site/select_Board Up'), 
    'number:12', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/DashboardPage/ProjectOBJ/Page_Projects  HPL Site/select_Head'), 
    'number:13', true)

WebUI.setText(findTestObject('Object Repository/DashboardPage/ProjectOBJ/Page_Projects  HPL Site/textarea_project_description'), 
    'rtrtrtr')

WebUI.click(findTestObject('Object Repository/DashboardPage/ProjectOBJ/Page_Projects  HPL Site/span_Save  Next'))

Thread.sleep(5000)

WebUI.selectOptionByValue(findTestObject('Object Repository/DashboardPage/ProjectOBJ/Page_Projects  HPL Site/select_SalesNew CategoryPlumbe'), 
    'number:59', true)

WebUI.setText(findTestObject('Object Repository/DashboardPage/ProjectOBJ/Page_Projects  HPL Site/input_job_title'), 'sales')

WebUI.setText(findTestObject('Object Repository/DashboardPage/ProjectOBJ/Page_Projects  HPL Site/input_insurance_company_name'), 
    'abcs')

WebUI.setText(findTestObject('Object Repository/DashboardPage/ProjectOBJ/Page_Projects  HPL Site/input_insurance_claim_no'), 
    '34343')

WebUI.setText(findTestObject('Object Repository/DashboardPage/ProjectOBJ/Page_Projects  HPL Site/input_insurance_policy_no'), 
    '34343')

WebUI.setText(findTestObject('Object Repository/DashboardPage/ProjectOBJ/Page_Projects  HPL Site/input_lock_box_no'), '34343')

WebUI.setText(findTestObject('Object Repository/DashboardPage/ProjectOBJ/Page_Projects  HPL Site/input_property_age'), '34343')

WebUI.setText(findTestObject('Object Repository/DashboardPage/ProjectOBJ/Page_Projects  HPL Site/input_permit_no'), '3434343')

WebUI.setText(findTestObject('Object Repository/DashboardPage/ProjectOBJ/Page_Projects  HPL Site/input_parcel_number'), 
    '4343434')

WebUI.setText(findTestObject('Object Repository/DashboardPage/ProjectOBJ/Page_Projects  HPL Site/input_project_revenue'), 
    '34343')

WebUI.setText(findTestObject('Object Repository/DashboardPage/ProjectOBJ/Page_Projects  HPL Site/input_moisture_date_of_loss'), 
    'July 20,2018')

WebUI.click(findTestObject('Object Repository/DashboardPage/ProjectOBJ/Page_Projects  HPL Site/i_fa fa-check-square-o fa-btn'))

WebUI.click(findTestObject('Object Repository/DashboardPage/ProjectOBJ/Page_Projects  HPL Site/button_OK'))

WebUI.closeBrowser()

