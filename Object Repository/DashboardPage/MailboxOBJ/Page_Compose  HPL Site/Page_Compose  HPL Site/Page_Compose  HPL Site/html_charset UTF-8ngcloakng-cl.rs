<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>html_charset UTF-8ngcloakng-cl</name>
   <tag></tag>
   <elementGuidId>bfde38a8-9df1-4d8a-bbd1-a832b4f1b84b</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>html</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>@charset &quot;UTF-8&quot;;[ng\:cloak],[ng-cloak],[data-ng-cloak],[x-ng-cloak],.ng-cloak,.x-ng-cloak,.ng-hide:not(.ng-hide-animate){display:none !important;}ng\:form{display:block;}.ng-animate-shim{visibility:hidden;}.ng-anchor{position:absolute;}
    


    Compose | HPL Site





































    
    


.cke{visibility:hidden;}iframe#_hjRemoteVarsFrame {display: none !important; width: 1px !important; height: 1px !important; opacity: 0 !important; pointer-events: none !important;}#katalon{font-family:monospace;font-size:13px;background-color:rgba(0,0,0,.7);position:fixed;top:0;left:0;right:0;display:block;z-index:999999999;line-height: normal} #katalon div{padding:0;margin:0;color:#fff;} #katalon kbd{display:inline-block;padding:3px 5px;font:13px Consolas,&quot;Liberation Mono&quot;,Menlo,Courier,monospace;line-height:10px;color:#555;vertical-align:middle;background-color:#fcfcfc;border:1px solid #ccc;border-bottom-color:#bbb;border-radius:3px;box-shadow:inset 0 -1px 0 #bbb;font-weight: bold} div#katalon-elementInfoDiv {color: lightblue; padding: 5px}

  



    
                    


                
                 H
                
                 HPL Site
            
        

        
            
                Toggle navigation
                
                
                
            
            
                
                 HPL Site
            


            
                

                    
    






.tooltip1 {
    position: relative;
    display: inline-block;
    border-bottom: 1px dotted black;
}

.tooltip1 .tooltiptext {
    visibility: hidden;
    width: 120px;
    background-color: black;
    color: #fff;
    text-align: center;
    border-radius: 6px;
    padding: 5px 0;
    
    /* Position the tooltip */
    position: absolute;
    z-index: 1;
    top: 100%;
    left: 50%;
    margin-left: -60px;
}

.tooltip1:hover .tooltiptext {
    visibility: visible;
}





    
        
            Need Help! Click Here
        

    

    
    
        
            
    
        
            You have 0 notification
        
        
                    
        
             Show All
        
    
    



    

        
            
            
                Rich Heirman
               
                
                                        
                                    
                
                                        
                                    
            
        
    
    
        
        
                            
                        Rich Heirman
        
        
        
            
                
                My Profile
            
        
        
        
        
        

            
                
                Logout
            

        
    



    
    $('#logout').click(function(){
        swal({
            title: &quot;Are you sure?&quot;,
            text: &quot;You will be logged out!&quot;,
            type: &quot;warning&quot;,
            showCancelButton: true,
            confirmButtonClass: &quot;btn-danger&quot;,
            confirmButtonText: &quot;Logout&quot;,
            closeOnConfirm: false
        },
        function(){
            var url = &quot;/logout&quot;;
            window.location.href = url;
        });

    });
    
                
            
                                        
                    

    
         Search . . .type 3 or more letters          
                            
        ContactProjectLeadEmployeeBusiness CategoriesAll
        
    
    


    

        angular.module('popApp')
                /*.config(function($interpolateProvider) {
                    $interpolateProvider.startSymbol('//');
                    $interpolateProvider.endSymbol('//');
                })*/.controller('searchController',function($scope,$http){
                        $scope.searchContents=[];
                    $scope.searchByTypes = [&quot;Contact&quot;, &quot;Project&quot;, &quot;Lead&quot;,&quot;Employee&quot;,&quot;Business Categories&quot;,&quot;All&quot;];
                        $scope.emptyResults = function () {
                            $scope.searchContents=[];
                        }
                        $scope.getSearchContent = function(search_text){

                                if(search_text === '' || search_text.length&lt;1){//alert('df');
                                   // $('.search_no_result').hide();
                                    angular.element('.search_no_result').css('display', 'none');

                                    $scope.searchContents=[];

                                } else if(search_text.length>2){
                                $http.post('/general-search',{'search_text':search_text,'search_type':$scope.selectedType}).then(function(res){

                                        $('.loader_search').show();


                                    if(res.data.length > 0 &amp;&amp; res.data!='project' &amp;&amp; res.data!='leads'){
                                            $scope.searchContents = res.data;
                                            $scope.no_result =  false;

                                        }else{
                                            $scope.searchContents=[];
                                            $scope.no_result = &quot;No Result Found!!!&quot;;
                                            
                                        }
                                        $('.loader_search').hide();
                                 });
                                }else{
                                        $scope.searchContents=[];
                                        $('.loader_search').hide();
                                }
                        };


        });

        $(&quot;body&quot;).click
        (
            function(e)
            {
                if(e.target.className !== &quot;main-search-wrapper&quot;)
                {
                    $('.search_no_result').hide();
                }
            }
        );
    

                
                    
    

    
    
        
        
            

                            
    
                    
            
    
        Rich Heirman
        
        
            
            
                                    
                        
                            
                        
                    
                            
        
    


     
    
        
             Dashboard
            
        
    

    
        
            
            Suggestions / Difficulties
        
    

            
            
                
                User
                
              
            
            
            
                                    
                         Add User
                    
                                                     User List
                    
                            
        
    
            
            
                
                Contacts
                
              
            
            
            
                                    
                         Add Contact
                    
                                                     Contact List
                    
                            
        
    
            
            
                
                Leads
                
              
            
            
            
                                    
                         Add Lead
                    
                                                     Lead List
                    
                            
        
    
            
            
                
                Projects
                
              
            
            
            
                                    
                         Add Projects
                    
                                                     Projects List
                    
                            
        
    
            
            
                
                Invoices
                
              
            
            
            
                                    
                         Create Invoice
                    
                
                                     Invoice List
                    
                
                                     Payment Log
                    
                            
        
    
        
        
            
            Services
            
              
            
        
        
                        
             Services
            
                                    
             Category
            
                    
    
                
            
                
                Equipment / Inventory
                
              
            
            
            
                                    
                         Inventory List
                    
                            
        
    
    
        
            
                
                Timesheet
                
              
            
            
            
                                    
                         New Entry
                    
                                                    
                         Timesheet Log
                    
                    
                         Report 
                    
                
            
        
                
            
                
                Notes
                
              
            
            
            
                                    
                         New Note
                    
                                                    
                         Note List
                    
                            
        
                
            
                
                Mailbox
                
              
            
            
            
                                    
                         Compose
                    
                                                    
                         Sent Mail
                    
                                                    
                         Mail Template List
                    
                            
        
    
            
            
                
                Events
                
              
            
            
            
                                    
                         Event Calendar
                    
                                                    
                         Event List
                    
                            
        
    

    
         
            Configuration
        
            
                 Permission
                
            
        
    
              
            
                 Settings
                
            
        


     
            
            
                
                 File Manager 
            
        
    
            

            
        
        
    


    
    
        
        
            
                Compose 
            
            
            
    

    
        

        

        
            

                
                    Entity Type
                    
                        
                            Select
                            
                                                            Contacts
                                                            Leads
                                                            Projects
                                                    
                        
                    
                
            
        

        
            
                
                    Clients
                    
                        Select OptionFred MathewsSelect OptionGordon GattieSelect OptionTest ContactSelect OptionAssign ContactSelect OptionChrist BentonSelect OptionDaniel HardmanSelect Optionk pSelect OptionPrit Cust test priSelect OptionCheck ContractorSelect Optiontest_1 Test_1LastSelect Optionsams sSelect OptionTest ContactSelect OptionRetest LiveSelect OptionRetest Again testSelect OptionRetest Pri 12121Select Optiontest testSelect OptionHyphen check testSelect Optionretest test retestSelect OptionPhone number TestSelect Optiontest testSelect OptionPHN PHNSelect OptionERER REERRSelect OptionTest testSelect OptionTest pn PNSelect OptionTest DOL Test DOLSelect OptionTest iiii test iiiiSelect OptionTest Project TestSelect OptionTest Business Catgy CatgySelect OptionEmp Test Test EMpSelect OptionTest TestSelect Optionretest addn info infoSelect OptionPp ColaradoSelect OptionTest TestSelect OptionDOL DOLSelect Optionjohn doeSelect OptionUpton GoldenSelect OptionLucius RichSelect OptionNash SosaSelect OptionIra RomeroSelect Optiondhiraj_cust karkera_custSelect Optionthirdparty newSelect Optionthirdparty lastSelect OptionPrit DoeSelect OptionCheck BillingSelect OptionCheck BillingSelect OptionLeads BillingSelect OptionTest Billing thirdpartySelect OptionBilling differentSelect OptionTestBilling SameSelect OptionTestBilling DifferentSelect OptionLeadBilling SameSelect OptionProject WithoutBillingSelect Option3rd partyy W/O titleSelect OptionThane CooleySelect OptionDOL CheckSelect OptionDOL1 CheckSelect OptionDOL without DOL CheckSelect OptionRetest NirbSelect Optionretest ProjSelect OptionRetest Addrs formattingSelect Optiontestnew billing billingSelect OptionLead address TestSelect OptionTest address TestSelect OptionAddr test TestSelect Optionfirsttest lasttestSelect Optiontest testSelect OptionProject Details BillingInsuranceSelect OptionTest TestSelect OptionTest TestSelect Optiontest12 test12Select Optionabc xyzSelect Optiontest testSelect Optionpop pop1Select Optionpop pop1Select Optionmy ProjSelect Optionmy contactSelect OptionTestInsBill Ins BillSelect Optiontest projcontactSelect OptionTest detailsSelect OptionNew Pro projSelect Option3rd party contact ContractSelect OptionFinalINS BIL Ins BIlSelect Option3rdparyy TestSelect OptionContactdetaols testSelect OptionTest category TestSelect OptionTest Contact test ContactSelect OptionNew contact KPSelect OptionNew proj KPSelect OptionNew proj KPSelect OptionCategory search TestSelect OptionWithout Business CategorySelect OptionPhone number test testSelect OptionSacha JohnstonSelect OptionPhone number TestSelect OptionTestC TestCSelect OptionTestC TestCSelect OptionTestnew testSelect OptionTest TestSelect OptionTest TestSelect OptionTest Add test AddSelect OptionTest Proj contact Test ProjSelect Optiontest Subs Test SubsSelect OptionTestSubsProj SubsProjSelect OptionTax Cal CalSelect OptionTest New ins InsSelect OptioniPad test iPad testSelect OptionTime check Time checkSelect OptionTime check again Time check againSelect OptionEmployee time retest retestSelect OptionTest email Test emailSelect OptionTest test Lst nameSelect OptionTest test Lst nameSelect OptionTest test Lst nameSelect OptionTest tst lst nameSelect Optionadasda dasdasSelect Optionasdas adasdsSelect Optionadasdas sadasdasSelect OptionTest TestSelect OptionDesiree CoffeySelect OptionTest test RetestSelect OptionLead test Test leadSelect OptionTest project TestSelect OptionRetest retestSelect OptionEmail retest retestSelect OptionEmail retest retestSelect Optionemail retest retestSelect OptionRetest retestSelect Optionretest retestSelect OptionTest Test TestSelect OptionJeniffer LopesSelect Optiondasdsd dasdsdSelect Optiontest email SubcontractorSelect OptionTest ProjSelect OptionNew EmplyoyeSelect OptionTest EmpSelect OptionActive UserSelect OptionActive UserSelect OptionActive UserSelect OptionTest TestSelect Optiondadas dsadsadSelect OptionTest mobile MobSelect OptionNew lead RetestSelect OptionSubcontracting TestSelect OptionNew proj live TestSelect OptionTest category TestSelect Optiontest custSelect Optiontest testSelect Optiontestuser testSelect OptionInv test TeestSelect Optiontesttt testtttSelect OptionTest user retestSelect OptionTest user retestSelect OptionLOcation testSelect OptionEmp1 Location testSelect OptionTest TestSelect OptionTest retestSelect OptionProj retestSelect OptionProj newSelect OptionEmp NextSelect OptionLead EmpSelect OptionGit HubSelect OptionRich projcontactSelect Optiontestemp123 retest1Select OptionJohn TestSelect Optiontest testSelect OptionTest JphnSelect OptionTest JohnSelect OptionTest JohnSelect OptionContact number testSelect Optionretest DoeSelect OptionRetest JohnSelect OptionTest JohnSelect OptionLeads RetestSelect OptionSub contractor retestSelect Optionretest FinalSelect OptionAbc 'Dmen retestSelect OptionProject SubcontractorSelect OptionTest TestSelect OptionTest TestSelect OptionProj con TestSelect OptionComp DirectorySelect OptionCompany DirSelect OptionMary DoeSelect OptionMavrecik D'mac-Select OptionMader TestSelect Optiontest testSelect Optiontest testSelect OptionTest TestSelect Option1234567789 12354566777Select Option123 Josh testSelect OptionJohn'Test D'melloSelect OptionJosh TestSelect Optiontest testSelect OptionRichard HouseSelect OptionTest testSelect Optiondasdsa dadasSelect Optionadada AdadasSelect Optionasdsa dsadasSelect Optionsdasdas dasdsaSelect Optionaa aaSelect Optiona aSelect Optiona aSelect Optiontest_name last_nameSelect OptionKermit ThompsonSelect OptionJermaine ChambersSelect OptionAlyssa PerkinsSelect OptionTest P PrinceSelect Optiontestsedf JoerSelect OptionDirect AssociationSelect OptionDirect Association1Select Optionasdsda asdasdasSelect Optionsdasdas dadsaSelect Optiontest TestSelect OptionTest TestagaibnSelect OptionTest DoeSelect OptionEmp CreatedSelect OptionCustomer CustSelect OptionTest LeadSelect Optionadasdas asdasdSelect Optionasdasdas sdasdasSelect Optionjay loffmanSelect Optionjay loffmanSelect Optionjay loffmanSelect Optionjay loffmanSelect Optionsds dsdsdsSelect Optionssdsdsd sdsdSelect Optionsdxsadsd sdsadsadSelect Optionjay loffmanSelect Optiondsfdfdsf dsfdsfdsfSelect Optionjay loffmanSelect Optionsadsadsa dsdsadsSelect Optionjay loffmanSelect Optionjay loffmanSelect Optionhem bagginsSelect Optionhem bagginsSelect Optionhem bagginsSelect Optionslade WilsonSelect Optionslade WilsonSelect Optionsadsadsad sdsadsadsadSelect Optionadsadsads sdsadsaSelect Optionval boleSelect Optionval boleSelect Optionval boleSelect Optionval bole
                        
                    
                
            
        


        
            

                
                To  
                    

                    
                                                Fred Mathews &lt;fred@yopmail.com> 
                                                Gordon Gattie &lt;> 
                                                Test Contact &lt;test_admin@yopmail.com> 
                                                Assign Contact &lt;assign_contact@yopmail.com> 
                                                Christ Benton &lt;test_admin@yopmail.com> 
                                                Daniel Hardman &lt;> 
                                                k p &lt;kiranpatil@heypayless.com> 
                                                Prit Cust test pri &lt;> 
                                                Check Contractor &lt;test.test-test1@yopmail.com> 
                                                test_1 Test_1Last &lt;test_project@yopmail.com> 
                                                sams s &lt;> 
                                                Test Contact &lt;> 
                                                Retest Live &lt;retestlive@yopmail.com> 
                                                Retest Again test &lt;> 
                                                Retest Pri 12121 &lt;> 
                                                test test &lt;> 
                                                Hyphen check test &lt;test.test-test-test@yopmail.com> 
                                                retest test retest &lt;> 
                                                Phone number Test &lt;> 
                                                test test &lt;> 
                                                PHN PHN &lt;> 
                                                ERER REERR &lt;> 
                                                Test test &lt;> 
                                                Test pn PN &lt;> 
                                                Test DOL Test DOL &lt;> 
                                                Test iiii test iiii &lt;> 
                                                Test Project Test &lt;> 
                                                Test Business Catgy Catgy &lt;> 
                                                Emp Test Test EMp &lt;> 
                                                Test Test &lt;> 
                                                retest addn info info &lt;> 
                                                Pp Colarado &lt;> 
                                                Test Test &lt;> 
                                                DOL DOL &lt;> 
                                                john doe &lt;> 
                                                Upton Golden &lt;lugiboso@yopmail.com> 
                                                Lucius Rich &lt;guxajin@yopmail.com> 
                                                Nash Sosa &lt;danymuqyha@yopmail.com> 
                                                Ira Romero &lt;fygoqeza@yopmail.com> 
                                                dhiraj_cust karkera_cust &lt;dhiraj@gmail.com> 
                                                thirdparty new &lt;> 
                                                thirdparty last &lt;> 
                                                Prit Doe &lt;> 
                                                Check Billing &lt;> 
                                                Check Billing &lt;> 
                                                Leads Billing &lt;> 
                                                Test Billing thirdparty &lt;> 
                                                Billing different &lt;> 
                                                TestBilling Same &lt;> 
                                                TestBilling Different &lt;> 
                                                LeadBilling Same &lt;> 
                                                Project WithoutBilling &lt;> 
                                                3rd partyy W/O title &lt;> 
                                                Thane Cooley &lt;lacuras@yopmail.com> 
                                                DOL Check &lt;> 
                                                DOL1 Check &lt;> 
                                                DOL without DOL Check &lt;> 
                                                Retest Nirb &lt;> 
                                                retest Proj &lt;> 
                                                Retest Addrs formatting &lt;> 
                                                testnew billing billing &lt;> 
                                                Lead address Test &lt;> 
                                                Test address Test &lt;> 
                                                Addr test Test &lt;> 
                                                firsttest lasttest &lt;test@testing.com> 
                                                test test &lt;test@testin.com> 
                                                Project Details BillingInsurance &lt;> 
                                                Test Test &lt;> 
                                                Test Test &lt;> 
                                                test12 test12 &lt;> 
                                                abc xyz &lt;> 
                                                test test &lt;> 
                                                pop pop1 &lt;> 
                                                pop pop1 &lt;> 
                                                my Proj &lt;> 
                                                my contact &lt;> 
                                                TestInsBill Ins Bill &lt;> 
                                                test projcontact &lt;> 
                                                Test details &lt;testdetails@yopmail.com> 
                                                New Pro proj &lt;> 
                                                3rd party contact Contract &lt;> 
                                                FinalINS BIL Ins BIl &lt;> 
                                                3rdparyy Test &lt;> 
                                                Contactdetaols test &lt;> 
                                                Test category Test &lt;> 
                                                Test Contact test Contact &lt;> 
                                                New contact KP &lt;> 
                                                New proj KP &lt;> 
                                                New proj KP &lt;> 
                                                Category search Test &lt;> 
                                                Without Business Category &lt;> 
                                                Phone number test test &lt;> 
                                                Sacha Johnston &lt;qydobu@yopmail.com> 
                                                Phone number Test &lt;> 
                                                TestC TestC &lt;> 
                                                TestC TestC &lt;> 
                                                Testnew test &lt;> 
                                                Test Test &lt;test@yopmail.com> 
                                                Test Test &lt;test@yopmail.com> 
                                                Test Add test Add &lt;> 
                                                Test Proj contact Test Proj &lt;> 
                                                test Subs Test Subs &lt;subs@yopmail.com> 
                                                TestSubsProj SubsProj &lt;> 
                                                Tax Cal Cal &lt;> 
                                                Test New ins Ins &lt;> 
                                                iPad test iPad test &lt;> 
                                                Time check Time check &lt;> 
                                                Time check again Time check again &lt;> 
                                                Employee time retest retest &lt;> 
                                                Test email Test email &lt;testemail@yopmail.com> 
                                                Test test Lst name &lt;testemail1@yopmail.com> 
                                                Test test Lst name &lt;test.email@yopmail.com> 
                                                Test test Lst name &lt;test.email-live@yopmail.com> 
                                                Test tst lst name &lt;test.email1@yopmail.com> 
                                                adasda dasdas &lt;testemail@yop2mail.com> 
                                                asdas adasds &lt;test.email123@yopmail.com> 
                                                adasdas sadasdas &lt;test.email111@yopmail.com> 
                                                Test Test &lt;testemail123@yopmail.com> 
                                                Desiree Coffey &lt;jennifer@phoenix2restore.com> 
                                                Test test Retest &lt;jenni-test@test2-og.com> 
                                                Lead test Test lead &lt;Lead-test@yopmail-2.co.in> 
                                                Test project Test &lt;1_3test@test-2.co.in> 
                                                Retest retest &lt;Retest12-12@yopmail.co.in> 
                                                Email retest retest &lt;Email-retest12-12@2yopmail.co.in> 
                                                Email retest retest &lt;leads.Leads-123.123@Yop2mail.com> 
                                                email retest retest &lt;Projects.Projects2-1234@2Yopmail2.co.in.sa> 
                                                Retest retest &lt;proj.proj22@2yopmail.co.in> 
                                                retest retest &lt;retest@yopmail.com> 
                                                Test Test Test &lt;Test.Test123@Yopmail2.co.in> 
                                                Jeniffer Lopes &lt;Jennifer.Test123-12@yopmail.co.in> 
                                                dasdsd dasdsd &lt;test@test.com> 
                                                test email Subcontractor &lt;Test.test123-12@Yop2Mail.com> 
                                                Test Proj &lt;Project.Teast12-12@2YopMail.co.in> 
                                                New Emplyoye &lt;new.Employee@2Yopmail.com> 
                                                Test Emp &lt;New.Test12-12@yop212mail.com> 
                                                Active User &lt;Active.user12@Yopmail.com> 
                                                Active User &lt;Active.user12123@Yopmail.com> 
                                                Active User &lt;Active.user1111@Yopmail.com> 
                                                Test Test &lt;PPtest@yopmail.com> 
                                                dadas dsadsad &lt;adasd2dasdd@asdasd.com> 
                                                Test mobile Mob &lt;mobile@yopmail.com> 
                                                New lead Retest &lt;newlead@yopmail.com> 
                                                Subcontracting Test &lt;subcontractor@yopmail.com> 
                                                New proj live Test &lt;newproj@yopmail.com> 
                                                Test category Test &lt;abc@yopmail.com> 
                                                test cust &lt;testcust@youpmail.com> 
                                                test test &lt;testtest@youpmail.com> 
                                                testuser test &lt;testusertest@youpmail.com> 
                                                Inv test Teest &lt;inv@yopmail.com> 
                                                testtt testttt &lt;testttt@yopmail.com> 
                                                Test user retest &lt;testuserlive@yopmail.com> 
                                                Test user retest &lt;testuserlive12@yopmail.com> 
                                                LOcation test &lt;testlocation@yopmail.com> 
                                                Emp1 Location test &lt;Emplocation@yopmail.com> 
                                                Test Test &lt;Save.Next@yopmail.com> 
                                                Test retest &lt;Save.Next-Lead@yopmail.com> 
                                                Proj retest &lt;Save.Next-Project@yopmail.com> 
                                                Proj new &lt;save.next-Proj@yopmail.com> 
                                                Emp Next &lt;save.next-emp@yopmail.com> 
                                                Lead Emp &lt;save.next-emp1@yopmail.com> 
                                                Git Hub &lt;create.contact-test@yopmail.com> 
                                                Rich projcontact &lt;proj.details@yopmail.com> 
                                                testemp123 retest1 &lt;test-Test123@yopmail.com> 
                                                John Test &lt;johntest@yopmail.com> 
                                                test test &lt;testemail123@yop2mail.com> 
                                                Test Jphn &lt;testjohn@yop2Mail.com> 
                                                Test John &lt;testjohn2@yop2Mail.com> 
                                                Test John &lt;testjohn1@yop2Mail.com> 
                                                Contact number test &lt;testcon@yopmail.com> 
                                                retest Doe &lt;Test.contact-123@yop2mail.co> 
                                                Retest John &lt;test.Contact-13@yop2mail.com> 
                                                Test John &lt;test.Contact@yop2mail.com> 
                                                Leads Retest &lt;Test.Leads@yopmail.com> 
                                                Sub contractor retest &lt;finalretest@yopmail.com> 
                                                retest Final &lt;enr.men@yopmail.com> 
                                                Abc 'Dmen retest &lt;Test.Project-12.2@yopmail.com> 
                                                Project Subcontractor &lt;test_employee@yopmail.com> 
                                                Test Test &lt;Test.Project-11@yopmail.com> 
                                                Test Test &lt;enr.men11@yopmail.com> 
                                                Proj con Test &lt;jit@yopmail.com> 
                                                Comp Directory &lt;comp.directory@yopmal.com> 
                                                Company Dir &lt;comp.dir@yopmail.com> 
                                                Mary Doe &lt;marydoe@yopmail.com> 
                                                Mavrecik D'mac- &lt;sparrow@yopmail.com> 
                                                Mader Test &lt;testmader@yopmail.com> 
                                                test test &lt;test@tes1221t.com> 
                                                test test &lt;testtest@yopmail.com> 
                                                Test Test &lt;Josh.Test@greentecrestoration.com> 
                                                1234567789 12354566777 &lt;123445@yop2mail.co> 
                                                123 Josh test &lt;test123@yopmail.com> 
                                                John'Test D'mello &lt;John.Test-123@yop2mail.com> 
                                                Josh Test &lt;Josh@Test.com> 
                                                test test &lt;test@testsetes.com> 
                                                Richard House &lt;nifewilavy@yopmail.com> 
                                                Test test &lt;Test.Test@yopmail.com> 
                                                dasdsa dadas &lt;sdasd@adsa.com> 
                                                adada Adadas &lt;dasd@adsa.com> 
                                                asdsa dsadas &lt;sdsd@sdsda.com> 
                                                sdasdas dasdsa &lt;dasdas@yopmail.com> 
                                                aa aa &lt;aaa@aaa.com> 
                                                a a &lt;a@aaa.com> 
                                                a a &lt;a@a.com> 
                                                test_name last_name &lt;someemail@yopmail.com> 
                                                Kermit Thompson &lt;tygyvocufe@yopmail.com> 
                                                Jermaine Chambers &lt;> 
                                                Alyssa Perkins &lt;qizykabi@yopmail.com> 
                                                Test P Prince &lt;Test.prince@yopmail.com> 
                                                testsedf Joer &lt;test.joel@yopmail.com> 
                                                Direct Association &lt;Direct.1@yopmail.com> 
                                                Direct Association1 &lt;Direct.2@yopmail.com> 
                                                asdsda asdasdas &lt;sdadsa@sdfas.com> 
                                                sdasdas dadsa &lt;dsadasd@ads.com> 
                                                test Test &lt;test.123@yopmail.com> 
                                                Test Testagaibn &lt;cad@adas.com> 
                                                Test Doe &lt;Test.doe123@yopmail.com> 
                                                Emp Created &lt;Emp.test@yopmail.com> 
                                                Customer Cust &lt;Test.Cust@yopmail.com> 
                                                Test Lead &lt;Test.Lead@yopmail.com> 
                                                adasdas asdasd &lt;sdasdas@asdasd.com> 
                                                asdasdas sdasdas &lt;sdasdas@adasd.com> 
                                                jay loffman &lt;jay@yopmail.com> 
                                                jay loffman &lt;jay@yopmail.com> 
                                                jay loffman &lt;jay@yopmail.com> 
                                                jay loffman &lt;jay@yopmail.com> 
                                                sds dsdsds &lt;dsds@y.com> 
                                                ssdsdsd sdsd &lt;dsdsds@y.com> 
                                                sdxsadsd sdsadsad &lt;sadasd@y.com> 
                                                jay loffman &lt;jay@yopmail.com> 
                                                dsfdfdsf dsfdsfdsf &lt;test@yopmail.como> 
                                                jay loffman &lt;jay@yopmail.com> 
                                                sadsadsa dsdsads &lt;sdsds@y.com> 
                                                jay loffman &lt;jay@yopmail.com> 
                                                jay loffman &lt;jay@yopmail.com> 
                                                hem baggins &lt;hem@yopmail.com> 
                                                hem baggins &lt;hem@yopmail.com> 
                                                hem baggins &lt;hem@yopmail.com> 
                                                slade Wilson &lt;Slade@yopmail.com> 
                                                slade Wilson &lt;Slade@yopmail.com> 
                                                sadsadsad sdsadsadsad &lt;sadsa@y.com> 
                                                adsadsads sdsadsa &lt;sadsad@y.com> 
                                                val bole &lt;val@yop.com> 
                                                val bole &lt;val@yop.com> 
                                                val bole &lt;val@yop.com> 
                                                val bole &lt;val@yop.com> 
                                                                           
                    ×Test Contact &lt;test_admin@yopmail.com>    
                   
                   

                                                      
                
            
        

        
            
                
                CC  
                    
                    
                                                 Fred Mathews &lt;fred@yopmail.com>
                                                 Gordon Gattie &lt;>
                                                 Test Contact &lt;test_admin@yopmail.com>
                                                 Assign Contact &lt;assign_contact@yopmail.com>
                                                 Christ Benton &lt;test_admin@yopmail.com>
                                                 Daniel Hardman &lt;>
                                                 k p &lt;kiranpatil@heypayless.com>
                                                 Prit Cust test pri &lt;>
                                                 Check Contractor &lt;test.test-test1@yopmail.com>
                                                 test_1 Test_1Last &lt;test_project@yopmail.com>
                                                 sams s &lt;>
                                                 Test Contact &lt;>
                                                 Retest Live &lt;retestlive@yopmail.com>
                                                 Retest Again test &lt;>
                                                 Retest Pri 12121 &lt;>
                                                 test test &lt;>
                                                 Hyphen check test &lt;test.test-test-test@yopmail.com>
                                                 retest test retest &lt;>
                                                 Phone number Test &lt;>
                                                 test test &lt;>
                                                 PHN PHN &lt;>
                                                 ERER REERR &lt;>
                                                 Test test &lt;>
                                                 Test pn PN &lt;>
                                                 Test DOL Test DOL &lt;>
                                                 Test iiii test iiii &lt;>
                                                 Test Project Test &lt;>
                                                 Test Business Catgy Catgy &lt;>
                                                 Emp Test Test EMp &lt;>
                                                 Test Test &lt;>
                                                 retest addn info info &lt;>
                                                 Pp Colarado &lt;>
                                                 Test Test &lt;>
                                                 DOL DOL &lt;>
                                                 john doe &lt;>
                                                 Upton Golden &lt;lugiboso@yopmail.com>
                                                 Lucius Rich &lt;guxajin@yopmail.com>
                                                 Nash Sosa &lt;danymuqyha@yopmail.com>
                                                 Ira Romero &lt;fygoqeza@yopmail.com>
                                                 dhiraj_cust karkera_cust &lt;dhiraj@gmail.com>
                                                 thirdparty new &lt;>
                                                 thirdparty last &lt;>
                                                 Prit Doe &lt;>
                                                 Check Billing &lt;>
                                                 Check Billing &lt;>
                                                 Leads Billing &lt;>
                                                 Test Billing thirdparty &lt;>
                                                 Billing different &lt;>
                                                 TestBilling Same &lt;>
                                                 TestBilling Different &lt;>
                                                 LeadBilling Same &lt;>
                                                 Project WithoutBilling &lt;>
                                                 3rd partyy W/O title &lt;>
                                                 Thane Cooley &lt;lacuras@yopmail.com>
                                                 DOL Check &lt;>
                                                 DOL1 Check &lt;>
                                                 DOL without DOL Check &lt;>
                                                 Retest Nirb &lt;>
                                                 retest Proj &lt;>
                                                 Retest Addrs formatting &lt;>
                                                 testnew billing billing &lt;>
                                                 Lead address Test &lt;>
                                                 Test address Test &lt;>
                                                 Addr test Test &lt;>
                                                 firsttest lasttest &lt;test@testing.com>
                                                 test test &lt;test@testin.com>
                                                 Project Details BillingInsurance &lt;>
                                                 Test Test &lt;>
                                                 Test Test &lt;>
                                                 test12 test12 &lt;>
                                                 abc xyz &lt;>
                                                 test test &lt;>
                                                 pop pop1 &lt;>
                                                 pop pop1 &lt;>
                                                 my Proj &lt;>
                                                 my contact &lt;>
                                                 TestInsBill Ins Bill &lt;>
                                                 test projcontact &lt;>
                                                 Test details &lt;testdetails@yopmail.com>
                                                 New Pro proj &lt;>
                                                 3rd party contact Contract &lt;>
                                                 FinalINS BIL Ins BIl &lt;>
                                                 3rdparyy Test &lt;>
                                                 Contactdetaols test &lt;>
                                                 Test category Test &lt;>
                                                 Test Contact test Contact &lt;>
                                                 New contact KP &lt;>
                                                 New proj KP &lt;>
                                                 New proj KP &lt;>
                                                 Category search Test &lt;>
                                                 Without Business Category &lt;>
                                                 Phone number test test &lt;>
                                                 Sacha Johnston &lt;qydobu@yopmail.com>
                                                 Phone number Test &lt;>
                                                 TestC TestC &lt;>
                                                 TestC TestC &lt;>
                                                 Testnew test &lt;>
                                                 Test Test &lt;test@yopmail.com>
                                                 Test Test &lt;test@yopmail.com>
                                                 Test Add test Add &lt;>
                                                 Test Proj contact Test Proj &lt;>
                                                 test Subs Test Subs &lt;subs@yopmail.com>
                                                 TestSubsProj SubsProj &lt;>
                                                 Tax Cal Cal &lt;>
                                                 Test New ins Ins &lt;>
                                                 iPad test iPad test &lt;>
                                                 Time check Time check &lt;>
                                                 Time check again Time check again &lt;>
                                                 Employee time retest retest &lt;>
                                                 Test email Test email &lt;testemail@yopmail.com>
                                                 Test test Lst name &lt;testemail1@yopmail.com>
                                                 Test test Lst name &lt;test.email@yopmail.com>
                                                 Test test Lst name &lt;test.email-live@yopmail.com>
                                                 Test tst lst name &lt;test.email1@yopmail.com>
                                                 adasda dasdas &lt;testemail@yop2mail.com>
                                                 asdas adasds &lt;test.email123@yopmail.com>
                                                 adasdas sadasdas &lt;test.email111@yopmail.com>
                                                 Test Test &lt;testemail123@yopmail.com>
                                                 Desiree Coffey &lt;jennifer@phoenix2restore.com>
                                                 Test test Retest &lt;jenni-test@test2-og.com>
                                                 Lead test Test lead &lt;Lead-test@yopmail-2.co.in>
                                                 Test project Test &lt;1_3test@test-2.co.in>
                                                 Retest retest &lt;Retest12-12@yopmail.co.in>
                                                 Email retest retest &lt;Email-retest12-12@2yopmail.co.in>
                                                 Email retest retest &lt;leads.Leads-123.123@Yop2mail.com>
                                                 email retest retest &lt;Projects.Projects2-1234@2Yopmail2.co.in.sa>
                                                 Retest retest &lt;proj.proj22@2yopmail.co.in>
                                                 retest retest &lt;retest@yopmail.com>
                                                 Test Test Test &lt;Test.Test123@Yopmail2.co.in>
                                                 Jeniffer Lopes &lt;Jennifer.Test123-12@yopmail.co.in>
                                                 dasdsd dasdsd &lt;test@test.com>
                                                 test email Subcontractor &lt;Test.test123-12@Yop2Mail.com>
                                                 Test Proj &lt;Project.Teast12-12@2YopMail.co.in>
                                                 New Emplyoye &lt;new.Employee@2Yopmail.com>
                                                 Test Emp &lt;New.Test12-12@yop212mail.com>
                                                 Active User &lt;Active.user12@Yopmail.com>
                                                 Active User &lt;Active.user12123@Yopmail.com>
                                                 Active User &lt;Active.user1111@Yopmail.com>
                                                 Test Test &lt;PPtest@yopmail.com>
                                                 dadas dsadsad &lt;adasd2dasdd@asdasd.com>
                                                 Test mobile Mob &lt;mobile@yopmail.com>
                                                 New lead Retest &lt;newlead@yopmail.com>
                                                 Subcontracting Test &lt;subcontractor@yopmail.com>
                                                 New proj live Test &lt;newproj@yopmail.com>
                                                 Test category Test &lt;abc@yopmail.com>
                                                 test cust &lt;testcust@youpmail.com>
                                                 test test &lt;testtest@youpmail.com>
                                                 testuser test &lt;testusertest@youpmail.com>
                                                 Inv test Teest &lt;inv@yopmail.com>
                                                 testtt testttt &lt;testttt@yopmail.com>
                                                 Test user retest &lt;testuserlive@yopmail.com>
                                                 Test user retest &lt;testuserlive12@yopmail.com>
                                                 LOcation test &lt;testlocation@yopmail.com>
                                                 Emp1 Location test &lt;Emplocation@yopmail.com>
                                                 Test Test &lt;Save.Next@yopmail.com>
                                                 Test retest &lt;Save.Next-Lead@yopmail.com>
                                                 Proj retest &lt;Save.Next-Project@yopmail.com>
                                                 Proj new &lt;save.next-Proj@yopmail.com>
                                                 Emp Next &lt;save.next-emp@yopmail.com>
                                                 Lead Emp &lt;save.next-emp1@yopmail.com>
                                                 Git Hub &lt;create.contact-test@yopmail.com>
                                                 Rich projcontact &lt;proj.details@yopmail.com>
                                                 testemp123 retest1 &lt;test-Test123@yopmail.com>
                                                 John Test &lt;johntest@yopmail.com>
                                                 test test &lt;testemail123@yop2mail.com>
                                                 Test Jphn &lt;testjohn@yop2Mail.com>
                                                 Test John &lt;testjohn2@yop2Mail.com>
                                                 Test John &lt;testjohn1@yop2Mail.com>
                                                 Contact number test &lt;testcon@yopmail.com>
                                                 retest Doe &lt;Test.contact-123@yop2mail.co>
                                                 Retest John &lt;test.Contact-13@yop2mail.com>
                                                 Test John &lt;test.Contact@yop2mail.com>
                                                 Leads Retest &lt;Test.Leads@yopmail.com>
                                                 Sub contractor retest &lt;finalretest@yopmail.com>
                                                 retest Final &lt;enr.men@yopmail.com>
                                                 Abc 'Dmen retest &lt;Test.Project-12.2@yopmail.com>
                                                 Project Subcontractor &lt;test_employee@yopmail.com>
                                                 Test Test &lt;Test.Project-11@yopmail.com>
                                                 Test Test &lt;enr.men11@yopmail.com>
                                                 Proj con Test &lt;jit@yopmail.com>
                                                 Comp Directory &lt;comp.directory@yopmal.com>
                                                 Company Dir &lt;comp.dir@yopmail.com>
                                                 Mary Doe &lt;marydoe@yopmail.com>
                                                 Mavrecik D'mac- &lt;sparrow@yopmail.com>
                                                 Mader Test &lt;testmader@yopmail.com>
                                                 test test &lt;test@tes1221t.com>
                                                 test test &lt;testtest@yopmail.com>
                                                 Test Test &lt;Josh.Test@greentecrestoration.com>
                                                 1234567789 12354566777 &lt;123445@yop2mail.co>
                                                 123 Josh test &lt;test123@yopmail.com>
                                                 John'Test D'mello &lt;John.Test-123@yop2mail.com>
                                                 Josh Test &lt;Josh@Test.com>
                                                 test test &lt;test@testsetes.com>
                                                 Richard House &lt;nifewilavy@yopmail.com>
                                                 Test test &lt;Test.Test@yopmail.com>
                                                 dasdsa dadas &lt;sdasd@adsa.com>
                                                 adada Adadas &lt;dasd@adsa.com>
                                                 asdsa dsadas &lt;sdsd@sdsda.com>
                                                 sdasdas dasdsa &lt;dasdas@yopmail.com>
                                                 aa aa &lt;aaa@aaa.com>
                                                 a a &lt;a@aaa.com>
                                                 a a &lt;a@a.com>
                                                 test_name last_name &lt;someemail@yopmail.com>
                                                 Kermit Thompson &lt;tygyvocufe@yopmail.com>
                                                 Jermaine Chambers &lt;>
                                                 Alyssa Perkins &lt;qizykabi@yopmail.com>
                                                 Test P Prince &lt;Test.prince@yopmail.com>
                                                 testsedf Joer &lt;test.joel@yopmail.com>
                                                 Direct Association &lt;Direct.1@yopmail.com>
                                                 Direct Association1 &lt;Direct.2@yopmail.com>
                                                 asdsda asdasdas &lt;sdadsa@sdfas.com>
                                                 sdasdas dadsa &lt;dsadasd@ads.com>
                                                 test Test &lt;test.123@yopmail.com>
                                                 Test Testagaibn &lt;cad@adas.com>
                                                 Test Doe &lt;Test.doe123@yopmail.com>
                                                 Emp Created &lt;Emp.test@yopmail.com>
                                                 Customer Cust &lt;Test.Cust@yopmail.com>
                                                 Test Lead &lt;Test.Lead@yopmail.com>
                                                 adasdas asdasd &lt;sdasdas@asdasd.com>
                                                 asdasdas sdasdas &lt;sdasdas@adasd.com>
                                                 jay loffman &lt;jay@yopmail.com>
                                                 jay loffman &lt;jay@yopmail.com>
                                                 jay loffman &lt;jay@yopmail.com>
                                                 jay loffman &lt;jay@yopmail.com>
                                                 sds dsdsds &lt;dsds@y.com>
                                                 ssdsdsd sdsd &lt;dsdsds@y.com>
                                                 sdxsadsd sdsadsad &lt;sadasd@y.com>
                                                 jay loffman &lt;jay@yopmail.com>
                                                 dsfdfdsf dsfdsfdsf &lt;test@yopmail.como>
                                                 jay loffman &lt;jay@yopmail.com>
                                                 sadsadsa dsdsads &lt;sdsds@y.com>
                                                 jay loffman &lt;jay@yopmail.com>
                                                 jay loffman &lt;jay@yopmail.com>
                                                 hem baggins &lt;hem@yopmail.com>
                                                 hem baggins &lt;hem@yopmail.com>
                                                 hem baggins &lt;hem@yopmail.com>
                                                 slade Wilson &lt;Slade@yopmail.com>
                                                 slade Wilson &lt;Slade@yopmail.com>
                                                 sadsadsad sdsadsadsad &lt;sadsa@y.com>
                                                 adsadsads sdsadsa &lt;sadsad@y.com>
                                                 val bole &lt;val@yop.com>
                                                 val bole &lt;val@yop.com>
                                                 val bole &lt;val@yop.com>
                                                 val bole &lt;val@yop.com>
                                
                                                          
                               
                        
                                                      
                
            
        

        
            
                
                
                    
                    
                         
                         
                        
                    
                        
                    
                
            
        

            
                
                    
                        Template Title
                        
                            Please selectFirst TemplatePri templateLive TempTest Pri3s MI templateretestWelcome email format
                            
                        
                    
                
            

        
            
                
                Subject  
                    
                               
                        
                                                      
                
            
        

        

        
            
                
                Message  
                    
                    --&lt;br>Regards warmfully,&lt;br />
&lt;br />
XYZRich Text Editor, messageEditor toolbarsClipboard/Undo CutKeyboard shortcut Command+X CopyKeyboard shortcut Command+C PasteKeyboard shortcut Command+V Paste as plain textKeyboard shortcut Command+Shift+V Paste from Word UndoKeyboard shortcut Command+Z RedoKeyboard shortcut Command+YEditing Spell Check As You TypeLinks LinkKeyboard shortcut Command+L Unlink AnchorInsert Image Table Insert Horizontal Line Insert Special CharacterTools MaximizeDocument SourceBasic Styles BoldKeyboard shortcut Command+B ItalicKeyboard shortcut Command+I Strikethrough Remove FormatParagraph Insert/Remove Numbered List Insert/Remove Bulleted List Decrease Indent Increase Indent Block QuoteStylesStylesStylesFormatFormatabout About CKEditorPress ALT 0 for help◢Elements path            
                        
                                                      
                
            
        

        
            
                
                Attachment
                    
                     
                                          
                     
                
            
        

        
        
            
                 Back
                 Email
            
              
        
    
    
    

        
        
        
    
    













    



    
        
            
                ×
                File Manager
            
            
            
        
    






















$.fn.modal.prototype.constructor.Constructor.DEFAULTS.backdrop = 'static';
$.fn.modal.prototype.constructor.Constructor.DEFAULTS.keyboard =  false;






        $(document).ready(function(){
        var semaphore = false;
        setInterval(function(){
            if(semaphore === true) return false;
            semaphore = true;
            $.get('/check-auth').success(function (res) {
                semaphore=false;
                if(res === '2'){
//                    swal('Session timed out!',' Please log in to continue!','info');
                    swal({title: &quot;Session timed out!&quot;, text: &quot;Please log in to continue!&quot;, type: &quot;info&quot;},
                            function(){
                                window.location.reload(true);
                            }
                    );
                }
            });
            $.get('/notifications/check-notifications').success(function(res)
            {
                if(res.count)
                {
                    if($('#read_notification > span').length > 0)
                    {
                        $('#read_notification').find('span').text(res.count);
                        $('#read_notification').prop('title', 'You have ' + res.count + ' notification');
                    }
                    else
                    {
                        $('#read_notification').prop('title', 'You have ' + res.count + ' notification');
                        $('#read_notification').append('&lt;span class=&quot;badge&quot; style=&quot;margin-top: 10px;margin-left: -7px&quot;>'+res.count+'&lt;/span>');
                    }

                }
                else
                {
                    if($('#read_notification > span').length > 0)
                    {
                        $('#read_notification').find('span').remove();
                    }
                }
            });
        },10000);
    });
        

    /*/!* Session Logout *!/
    // Logout the user.
    function IdleTimeout() {
        window.location = logoutUrl;
    }

    var intervalTimer;
    var displayText = &quot;You will Be Redirected to Dashboard in next #1 Minutes&quot;
    // Set timeout variables.
    var timoutWarning = 2*60*1000; // Display warning in 2 Mins.
    var timoutNow = 10*60*1000; // Timeout in 20 mins.
    var logoutUrl =  &quot;http://www.popssoftware.com&quot; + '/logout'; // URL to page.

    var warningTimer;
    var timeoutTimer;

    $(document).ready(function () {
        //Increment the idle time counter every minute.
        var idleInterval = StartTimers(); // 1 minute

        //Zero the idle timer on mouse movement.
        $(this).mousemove(function (e) {
            ResetTimers();
        });
        $(this).keypress(function (e) {
            ResetTimers();
        });
    });

    // Start timers.
    function StartTimers() {
        warningTimer = setTimeout(&quot;IdleWarning()&quot;, timoutWarning);
        timeoutTimer = setTimeout(&quot;IdleTimeout()&quot;, timoutNow);
    }

    // Reset timers.
    function ResetTimers() {
        clearTimeout(warningTimer);
        clearTimeout(timeoutTimer);
        $('.timer_div').hide();
        clearInterval(intervalTimer);
        StartTimers();
    }

    // Show idle timeout warning dialog.
    function IdleWarning() {
        $('.timer_div').show();
        var timer = 20*60, minutes, seconds;
        intervalTimer = setInterval(function () {
            minutes = parseInt(timer / 60, 10);
            seconds = parseInt(timer % 60, 10);

            minutes = minutes &lt; 10 ? &quot;0&quot; + minutes : minutes;
            seconds = seconds &lt; 10 ? &quot;0&quot; + seconds : seconds;

            displayText = minutes + &quot;:&quot; + seconds;

            if (--timer &lt; 0) {
                IdleTimeout();
            }
            $('.time_left').text(&quot;You will Be Redirected to Dashboard in next &quot;+displayText);

        }, 1000);

    }
    /!* End Session Logout *!/*/





 (function(h,o,t,j,a,r){ h.hj=h.hj||function(){(h.hj.q=h.hj.q||[]).push(arguments)}; h._hjSettings={hjid:945361,hjsv:6}; a=o.getElementsByTagName('head')[0]; r=o.createElement('script');r.async=1; r.src=t+h._hjSettings.hjid+j+h._hjSettings.hjsv; a.appendChild(r); })(window,document,'https://static.hotjar.com/c/hotjar-','.js?sv=');





















    

        var oTable;
        var type;
        var route_type;
        var route = &quot;&quot;;

        $(document).ready(function () {
            var post_data = {
                _token:'gtkTnZlkHEAZZbPyxh723yv0h4KwjHgEvlDH7t1E',
                route: route
            };

            
                oTable = $('#data').DataTable({
                responsive: true,
                dom: 'Blfrtip',
                &quot;lengthMenu&quot;: [[10, 25, 50,100, -1], [10, 25, 50,100, &quot;All&quot;]],
                buttons: [
                    {
                        extend: 'excel', text: 'Export to Excel', exportOptions: {
                        columns: ':visible :not(:last-child)'
                    }
                    },
                    { extend: 'pdf', text: 'Export to PDF',exportOptions: {
                        columns: ':visible :not(:last-child)'
                    }
                                        }
                ],
                 &quot;processing&quot;: true,
                &quot;serverSide&quot;: true,
                &quot;order&quot;: [],
                &quot;ajax&quot;: {
                    &quot;url&quot;:'https://www.popssoftware.com/mailbox' + ((typeof $('#data').attr('data-id') != &quot;undefined&quot;) ? &quot;/&quot; +
                    $('#id').val() + &quot;/&quot; + $('#data').attr('data-id') : &quot;/data&quot;),
                    &quot;type&quot;:'GET',
                    data:post_data
                },

                
                                &quot;columnDefs&quot;:[
                                                    { &quot;width&quot;: &quot;20%&quot;, &quot;targets&quot;: [1] },
                                                    { &quot;width&quot;: &quot;20%&quot;, &quot;targets&quot;: [2] },
                                                    { &quot;width&quot;: &quot;15%&quot;, &quot;targets&quot;: [3] },
                                                    { &quot;width&quot;: &quot;30%&quot;, &quot;targets&quot;: [4] },
                                                    { &quot;width&quot;: &quot;10%&quot;, &quot;targets&quot;: [5] },
                                        ],
                
                                &quot;columns&quot;:
                    [
                                                {

                            data:&quot;id&quot;,
                            name:&quot;id&quot;,
                            'visible':&quot;&quot;,
                            'orderable':true
                        },
                                                {

                            data:&quot;from&quot;,
                            name:&quot;from&quot;,
                            'visible':&quot;1&quot;,
                            'orderable':true
                        },
                                                {

                            data:&quot;to&quot;,
                            name:&quot;to&quot;,
                            'visible':&quot;1&quot;,
                            'orderable':true
                        },
                                                {

                            data:&quot;subject&quot;,
                            name:&quot;subject&quot;,
                            'visible':&quot;1&quot;,
                            'orderable':true
                        },
                                                {

                            data:&quot;message&quot;,
                            name:&quot;message&quot;,
                            'visible':&quot;1&quot;,
                            'orderable':true
                        },
                                                {

                            data:&quot;date&quot;,
                            name:&quot;date&quot;,
                            'visible':&quot;1&quot;,
                            'orderable':true
                        },
                                                {

                            data:&quot;action&quot;,
                            name:&quot;action&quot;,
                            'visible':&quot;1&quot;,
                            'orderable':true
                        },
                                            ]
                            });
        });

//        $('#data tbody').on('click', '.reactivate_it', function () {
        $(document).on('click', '.reactivate_it', function () {
                var client_id = this.id;
                swal({
                        html:true,
                        title: &quot;Are you sure?&quot;,
                        text: &quot;Company will be restored.&quot;,
                        type: &quot;warning&quot;,
                        showCancelButton: true,
                        confirmButtonColor: &quot;#DD6B55&quot;,
                        confirmButtonText: &quot;Yes&quot;,
                        closeOnConfirm: false,
                        showLoaderOnConfirm: true
                    },
                    function () {
                        $.ajax({
                            url: '/master_sales/reactivate-client/' + client_id,
                            type: 'POST',
                            data: {_token: 'gtkTnZlkHEAZZbPyxh723yv0h4KwjHgEvlDH7t1E'},
                            success: function (result) {
                                swal({
                                    title: &quot;Re-Activate!&quot;,
                                    text: &quot;Client has been re-activated.&quot;,
                                    type: &quot;success&quot;
                                }, function() {
                                    window.location.reload(true);
                                });

                            }
                        });
                    });
            });

//        $('#data tbody').on('click', '.delete_it', function () {
        $(document).on('click', '.delete_it', function () {
            var contact_id = this.id;
            $.ajax({
                url: '/contacts/get-all-associations/' + contact_id,
                type: 'POST',
                data: { _token: 'gtkTnZlkHEAZZbPyxh723yv0h4KwjHgEvlDH7t1E'},

                success:function(data_res){
                    swal({
                            html:true,
                            title: &quot;Are you sure?&quot;,
                            text: data_res,
                            type: &quot;warning&quot;,
                            showCancelButton: true,
                            confirmButtonColor: &quot;#DD6B55&quot;,
                            confirmButtonText: &quot;Yes&quot;,
                            closeOnConfirm: false,
                            showLoaderOnConfirm: true
                        },
                        function () {
                            $.ajax({
                                url: '/contacts/' + contact_id,
                                type: 'POST',
                                data: {_method: 'delete', _token: 'gtkTnZlkHEAZZbPyxh723yv0h4KwjHgEvlDH7t1E'},
                                success: function (result) {
                                    swal(&quot;Deleted!&quot;, &quot;&quot;, &quot;success&quot;);
                                    window.location.reload(true);
                                }
                            });
                        });
                }
            });
        });

//        $('#data tbody').on('click', '.delete_sales_lead', function () {
        $(document).on('click', '.delete_sales_lead', function () {
            var id = this.id;
            swal({
                        title: &quot;Are you sure?&quot;,
                        text: &quot;&quot;,
                        type: &quot;warning&quot;,
                        showCancelButton: true,
                        confirmButtonColor: &quot;#DD6B55&quot;,
                        confirmButtonText: &quot;Yes&quot;,
                        closeOnConfirm: false,
                        showLoaderOnConfirm: true
                    },
                    function(){
                        $.ajax({
                            url: '/sales-leads/delete',
                            type: 'POST',
                            data: {_method: 'delete', _token :'gtkTnZlkHEAZZbPyxh723yv0h4KwjHgEvlDH7t1E', 'id' : id},
                            success: function(result) {
                                swal(&quot;Lead deleted successfully!&quot;, &quot;&quot;, &quot;success&quot;);
                                oTable.ajax.reload();
                            }
                        });
                    });
        } );


//        $('#data tbody').on('click', '.convert_to_lead', function () {
        $(document).on('click', '.convert_to_lead', function () {

            var hash_id = this.id.split('_')[1];

            swal({
                    title: &quot;Are you sure?&quot;,
                    text: &quot;This contact will be converted to lead&quot;,
                    type: &quot;warning&quot;,
                    showCancelButton: true,
                    confirmButtonColor: &quot;#DD6B55&quot;,
                    confirmButtonText: &quot;Yes&quot;,
                    closeOnConfirm: false,
                    showLoaderOnConfirm: true
                },
                function(){
                    $.ajax({
                        url: '/contacts/convert-to-lead/'+hash_id,
                        type: 'POST',
                        data: {_method: 'post', _token :'gtkTnZlkHEAZZbPyxh723yv0h4KwjHgEvlDH7t1E'},
                        success: function(result) {
                            swal(&quot;Success!&quot;, &quot;This contact is converted to lead.&quot;, &quot;success&quot;);
                            window.location = 'https://www.popssoftware.com/leads/'+result.message;
                        }
                    });
                });
        } );

        $(document).on('click', '.convert_to_project_from_contact', function () {
            var hash_id = this.id.split('_')[1];
            type = 'contact';
            $('#contact_hash_id').val(hash_id);
            $('#add-required-project-data').modal({backdrop: 'static', keyboard: false});
        } );

//        $('#data tbody').on('click', '.convert_to_project_from_lead', function () {
        $(document).on('click', '.convert_to_project_from_lead', function () {

            var hash_id = this.id.split('_')[1];
            type = 'lead';
            $('#contact_hash_id').val(hash_id);
            $('#add-required-project-data').modal({backdrop: 'static', keyboard: false});

        } );



        $(document).on('click', '.delete_project', function () {
            var hash_id = this.id;
            swal({
                    title: &quot;Are you sure?&quot;,
                    text: &quot;&quot;,
                    type: &quot;warning&quot;,
                    showCancelButton: true,
                    confirmButtonColor: &quot;#DD6B55&quot;,
                    confirmButtonText: &quot;Yes&quot;,
                    closeOnConfirm: false,
                    showLoaderOnConfirm: true
                },
                function(){
                    $.ajax({
                        url: '/projects/'+hash_id+'/delete',
                        type: 'POST',
                        data: {_method: 'delete', _token :'gtkTnZlkHEAZZbPyxh723yv0h4KwjHgEvlDH7t1E'},
                        success: function(result) {
                            swal(&quot;Deleted!&quot;, &quot;&quot;, &quot;success&quot;);
                            window.location.reload(true);
                        }
                    });
                });
        } );
        $(document).on('click', '.duplicate_project', function () {
            var hash_id = this.id;
            swal({
                    title: &quot;Are you sure?&quot;,
                    text: &quot;This project will be duplicated&quot;,
                    type: &quot;warning&quot;,
                    showCancelButton: true,
                    confirmButtonColor: &quot;#DD6B55&quot;,
                    confirmButtonText: &quot;Yes&quot;,
                    closeOnConfirm: false,
                    showLoaderOnConfirm: true
                },
                function(){
                    $.ajax({
                        url: '/projects/'+hash_id+'/duplicate',
                        type: 'POST',
                        data: { _token :'gtkTnZlkHEAZZbPyxh723yv0h4KwjHgEvlDH7t1E'},
                        success: function(result) {
                            if(result.error===2) {
                                console.error('something went wrong'); return;
                            }
                            swal(&quot;Duplicated!&quot;, &quot;The new project id is &quot;+result.message, &quot;success&quot;);
                            window.location.reload(true);
                        }
                    });
                });
        } );

        $(document).on('click', '.delete_lead', function () {
            var hash_id = this.id;
            swal({
                    title: &quot;Are you sure?&quot;,
                    text: &quot;&quot;,
                    type: &quot;warning&quot;,
                    showCancelButton: true,
                    confirmButtonColor: &quot;#DD6B55&quot;,
                    confirmButtonText: &quot;Yes&quot;,
                    closeOnConfirm: false,
                    showLoaderOnConfirm: true
                },
                function(){
                    $.ajax({
                        url: '/leads/'+hash_id+'/delete',
                        type: 'POST',
                        data: {_method: 'delete', _token :'gtkTnZlkHEAZZbPyxh723yv0h4KwjHgEvlDH7t1E'},
                        success: function(result) {
                            swal(&quot;Deleted!&quot;, &quot;&quot;, &quot;success&quot;);
                            window.location.reload(true);
                        }
                    });
                });
        } );

        $(document).on('click', '.delete_lead', function () {
            var hash_id = this.id;
            swal({
                        title: &quot;Are you sure?&quot;,
                        text: &quot;&quot;,
                        type: &quot;warning&quot;,
                        showCancelButton: true,
                        confirmButtonColor: &quot;#DD6B55&quot;,
                        confirmButtonText: &quot;Yes&quot;,
                        closeOnConfirm: false,
                        showLoaderOnConfirm: true
                    },
                    function(){
                        $.ajax({
                            url: '/leads/'+hash_id+'/delete',
                            type: 'POST',
                            data: {_method: 'delete', _token :'gtkTnZlkHEAZZbPyxh723yv0h4KwjHgEvlDH7t1E'},
                            success: function(result) {
                                swal(&quot;Deleted!&quot;, &quot;&quot;, &quot;success&quot;);
                                window.location.reload(true);
                            }
                        });
                    });
        } );


        //for validation

        /* $('form').bootstrapValidator({
             icon: {
                 valid: 'glyphicon glyphicon-ok',
                 invalid: 'glyphicon glyphicon-remove',
                 validating: 'glyphicon glyphicon-refresh'
             }
         });*/


    
 



 $(document).ready(function(){

     // KiranP:-> Max File Upload Side validation
     $(document).on('change','.attachment',function(){
                $(&quot;.attachment-block&quot;).hide();
               var fp = $(this);
               var lg = fp[0].files.length; // get length
               var items = fp[0].files;
               var fileSize = 0;
           
              if(lg > 0) {
               for (var i = 0; i &lt; lg; i++) {
                   fileSize = fileSize+items[i].size; // get file size
               }

               if(fileSize > 10485760) {
                    $(&quot;.attachment-block&quot;).html('File size must not be more than 10 MB');
                    $(&quot;.attachment-block&quot;).show();
                    $(this).val('');
               }
           }
        });
      // KiranP:-> File Upload Side validation ends here..

     $(&quot;.tags-field&quot;).select2();

    CKEDITOR.replace( 'message',
         {
          customConfig : 'config.js',
          /*toolbar : 'simple'*/
        });

    var content = $('textarea#message' ).val();


    $('#template_title').on('change', function(e){

            var template_title = e.target.value;

            var url = $('#hidden').val();
            var name =$(&quot;#to option:selected&quot;).val();

            if(template_title) {
                $.ajax({
                    url: url+'/'+template_title,
                    type: &quot;GET&quot;,
                    dataType: &quot;json&quot;,
                    success:function(data) {  

                    CKEDITOR.instances['message'].setData(data['template_text']+content);
                    $('#subject').val(data['template_subject']);
          
                    }
                });
                }
            });

     $('#entity_type').on('change', function(e){
         var entity_type = e.target.value;
         if(entity_type) {
             $.ajax({
                 url: '/notes/ajax/'+entity_type,
                 type: &quot;GET&quot;,
                 dataType: &quot;json&quot;,
                 success:function(data) {
                     $('#clients').empty();
                     $.each(data, function(key, value) {
                         if(value == null){
                             $('#clients').append('&lt;option value=&quot;&quot;>Nothing to select&lt;/option>');
                         }else{
                             $('#clients').append('Select Option');
                             $('#clients').append('&lt;option value=&quot;'+ key +'&quot;>'+ value +'&lt;/option>');
                         }
                     });
                 }
             });
         }else{
             $('#clients').empty();
         }
     });


        });





    $(document).ready(function () {
        $('.serach-menu').click(function(event){
            event.stopPropagation();
            $('.center-search').slideToggle(&quot;fast&quot;);
        });
        $(&quot;.center-search&quot;).on(&quot;click&quot;, function (event) {
            event.stopPropagation();
        });
        $('.date').datetimepicker({format: 'DD.MM.GGGG',
            icons: {
                time: &quot;fa fa-clock-o&quot;,
                date: &quot;fa fa-calendar&quot;,
                up: &quot;fa fa-arrow-up&quot;,
                down: &quot;fa fa-arrow-down&quot;
            }});
        $('.datetime').datetimepicker({format: 'DD.MM.GGGG HH:mm',
            icons: {
                time: &quot;fa fa-clock-o&quot;,
                date: &quot;fa fa-calendar&quot;,
                up: &quot;fa fa-arrow-up&quot;,
                down: &quot;fa fa-arrow-down&quot;
            }});
    });
    /*$(document).on(&quot;click&quot;, function () {
        $(&quot;.center-search&quot;).slideUp();
    });*/

    /*Notification icon on click mark as read*/

    $('body').on('click','#read_notification', function(e)
    {
        $this = $(this);
        $.ajax({
            url:'/notifications/get-unread-notification',
            success: function(data)
            {
                if(data.status == 'success')
                {
                    var html = '';
                    var notititle = '&lt;li class=&quot;dropdown-title notification-count-title&quot;>You have '+data.notifications.length+' notification&lt;/li>';
                    var notifooter = '&lt;li class=&quot;dropdown-title notification-footer&quot;>&lt;a href=&quot;/notifications&quot;> Show All&lt;/a>&lt;/li>';
                    $(data.notifications).each(function(k,v)
                    {
                        html += '&lt;li class=&quot;dropdown-title&quot; data-id=&quot;'+v.id+'&quot;>'+v.notification.content+'&lt;/li>'
                    });
                    if(html == '')
                    {
                        $('#notifications').find('.notifications-messages').html('&lt;li class=&quot;dropdown-title notification-count-title&quot;>No Notification&lt;/li>'+notifooter);
                    }
                    else
                    {
                        $('#notifications').find('.notifications-messages').html(notititle+'&lt;span class=&quot;notification-wrapper&quot;>'+html+'&lt;/span>'+notifooter);
                    }
                    $this.find('span').remove();
                }
            }
        });
    });

    $('body').on('click','.addToCalendar', function(e)
    {
        e.preventDefault();
        $this = $(this);
        var id = $this.attr('data-id');
        swal({title: &quot;Are you sure?&quot;,
                text: &quot;You want to add this event to calendar&quot;,
                type: &quot;warning&quot;,
                showCancelButton: true,
                closeOnConfirm: false,
                confirmButtonText: &quot;Yes&quot;,
                cancelButtonText: &quot;No&quot;,
                confirmButtonColor: &quot;#DD6B55&quot;,
                animation: &quot;slide-from-top&quot;
            },
            function(isConfirm){
                if(isConfirm) {
                    $.ajax({
                        'url':'/calendar/event/accept',
                        'data':{'event_id':id,'_token':'gtkTnZlkHEAZZbPyxh723yv0h4KwjHgEvlDH7t1E'},
                        'type':'POST',
                        'success': function(data)
                        {
                            if(data.status == 'success')
                            {
                                $('body #calendar').fullCalendar('refetchEvents');
                                swal(&quot;Success&quot;, &quot;Event added to Calendar&quot;, &quot;success&quot;);
                            }
                            else if(data.status == 'accepted')
                            {
                                swal(&quot;Oops&quot;, &quot;Event already added to Calendar&quot;, &quot;warning&quot;);
                            }
                            else
                            {
                                swal(&quot;Oops&quot;, &quot;Event could not be added to Calendar&quot;, &quot;error&quot;);
                            }
                        }
                    });
                }else{
                    swal(&quot;Success&quot;, &quot;Cancelled&quot;, &quot;warning&quot;);
                }


            }
        );

    });


/html[1]</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]</value>
   </webElementProperties>
</WebElementEntity>
