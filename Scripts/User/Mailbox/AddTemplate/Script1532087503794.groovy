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

WebUI.click(findTestObject('Object Repository/DashboardPage/MailboxOBJ/Page_Compose  HPL Site/Page_Compose  HPL Site/Page_Welcome to HPL Site  HPL Site/a_Mailbox'))

WebUI.click(findTestObject('Object Repository/DashboardPage/MailboxOBJ/Page_Compose  HPL Site/Page_Compose  HPL Site/Page_Welcome to HPL Site  HPL Site/a_Compose'))

WebUI.switchToWindowTitle('Compose | HPL Site')

WebUI.selectOptionByValue(findTestObject('Object Repository/DashboardPage/MailboxOBJ/Page_Compose  HPL Site/Page_Compose  HPL Site/Page_Compose  HPL Site/select_Select'), 
    '1', true)

WebUI.click(findTestObject('Object Repository/DashboardPage/MailboxOBJ/Page_Compose  HPL Site/Page_Compose  HPL Site/Page_Compose  HPL Site/ul_select2-selection__rendered_1'))

WebUI.click(findTestObject('Object Repository/DashboardPage/MailboxOBJ/Page_Compose  HPL Site/Page_Compose  HPL Site/Page_Compose  HPL Site/li_Test Contact test_adminyopm_1'))

WebUI.selectOptionByValue(findTestObject('Object Repository/DashboardPage/MailboxOBJ/Page_Compose  HPL Site/Page_Compose  HPL Site/Page_Compose  HPL Site/select_Please selectFirst Temp'), 
    '3', true)

WebUI.click(findTestObject('Object Repository/DashboardPage/MailboxOBJ/Page_Compose  HPL Site/Page_Compose  HPL Site/Page_Compose  HPL Site/span_Email'))

WebUI.closeBrowser()

