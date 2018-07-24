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

WebUI.click(findTestObject('Object Repository/DashboardPage/InventoryOBJ/Page_Welcome to HPL Site  HPL Site/a_Equipment  Inventory'))

WebUI.click(findTestObject('Object Repository/DashboardPage/InventoryOBJ/Page_Welcome to HPL Site  HPL Site/a_Inventory List'))

WebUI.click(findTestObject('Object Repository/DashboardPage/InventoryOBJ/Page_Inventories  HPL Site/span_New'))

WebUI.setText(findTestObject('Object Repository/DashboardPage/InventoryOBJ/Page_New Inventory  HPL Site/input_product_name'), 
    'newinventory')

WebUI.setText(findTestObject('Object Repository/DashboardPage/InventoryOBJ/Page_New Inventory  HPL Site/input_item_no'), 
    '12')

WebUI.setText(findTestObject('Object Repository/DashboardPage/InventoryOBJ/Page_New Inventory  HPL Site/input_sale_price'), 
    '1')

WebUI.setText(findTestObject('Object Repository/DashboardPage/InventoryOBJ/Page_New Inventory  HPL Site/textarea_description'), 
    'test')

WebUI.setText(findTestObject('Object Repository/DashboardPage/InventoryOBJ/Page_New Inventory  HPL Site/input_manufacturer'), 
    'test')

WebUI.setText(findTestObject('Object Repository/DashboardPage/InventoryOBJ/Page_New Inventory  HPL Site/input_make'), 'test')

WebUI.setText(findTestObject('Object Repository/DashboardPage/InventoryOBJ/Page_New Inventory  HPL Site/input_model'), 'test')

WebUI.setText(findTestObject('Object Repository/DashboardPage/InventoryOBJ/Page_New Inventory  HPL Site/input_serial'), 
    'test1234')

WebUI.setText(findTestObject('Object Repository/DashboardPage/InventoryOBJ/Page_New Inventory  HPL Site/input_age'), '1')

WebUI.click(findTestObject('Object Repository/DashboardPage/InventoryOBJ/Page_New Inventory  HPL Site/span_OK'))

WebUI.closeBrowser()

