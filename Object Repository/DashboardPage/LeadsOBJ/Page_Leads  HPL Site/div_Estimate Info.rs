<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_Estimate Info</name>
   <tag></tag>
   <elementGuidId>ddd1f221-c0f0-4b86-ab13-6683533a58ba</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>tab_container bg_tab_container</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
                                                Estimate Info
                
                    
                        
                            
                            
                                
                                
                                    Upload New Photo
                                    
                                        

                                            
                                            
                                            
                                        
                                        (Only GIF,JPEG,PNG image format with max size of 2MB)
                                        
                        Select image
                            Change
                            
                                            Remove
                                        
                                    
                                
                                
                                    
                                
                            
                        
                        
                            
                                
                                    Personal Information
                                
                                
                                    Contact Type
                                    
                                        
                                        
                                                                                            Customer
                                                                                            Contractor
                                                                                            Business Affiliate
                                                                                        
                                        

                                        
                                    
                                
                                
                                
                                    First name
                                    
                                        
                                        
                                        First name is required

                                    
                                
                                
                                    Middle Initial
                                    
                                        
                                        
                                        Middle name is invalid
                                    
                                
                                
                                    Last name
                                    
                                        
                                        
                                        Last name is required

                                    
                                
                                

                                
                                
                                    Email
                                    
                                        
                                        
                                         Email is invalid
                                        
                                    
                                
                                
                                
                                    
                                        
                                        
                                            
                                                Number Type
                                                
                                                    

                                                    
                                                                                                                    Home
                                                                                                                    Work
                                                                                                                    Cell
                                                                                                                    Fax
                                                                                                                    Other
                                                                                                            

                                                    
                                                
                                            
                                            
                                                Phone Number
                                                
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                            
                                                Extension
                                                
                                                    
                                                    
                                                
                                            

                                        
                                    
                                        X
                                        
                                            
                                                Number Type
                                                
                                                    

                                                    
                                                                                                                    Home
                                                                                                                    Work
                                                                                                                    Cell
                                                                                                                    Fax
                                                                                                                    Other
                                                                                                            

                                                    
                                                
                                            
                                            
                                                Phone Number
                                                
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                            
                                                Extension
                                                
                                                    
                                                    
                                                
                                            

                                        
                                    
                                

                                
                                    
                                          Add Phone 
                                    
                                
                            
                        
                        
                        
                            
                                
                                    Address
                                
                                
                                    Street Address
                                    
                                        
                                        
                                    
                                
                                
                                    Apt/Suite no.
                                    
                                        
                                        
                                    
                                
                                
                                
                                    City
                                    
                                        
                                        
                                    
                                
                                
                                    State
                                    
                                        
                                        
                                    
                                
                                
                                    Country
                                    
                                        
                                        

                                        
                                    
                                
                                
                                
                                    ZIP
                                    
                                        
                                        
                                    
                                
                            
                        
                        
                            Billing address is identical to the property address ? 
                        
                        
                            
                                
                                    Billing Address
                                
                                
                                    Street Address
                                    
                                        
                                        
                                        Address cannot be more than 250 characters long
                                    
                                
                                
                                    Apt/Suite no.
                                    
                                        
                                        
                                        Apartment Suite No. cannot be more than 12 characters long
                                    
                                
                                
                                
                                    City
                                    
                                        
                                        
                                        City cannot be more than 30 characters long
                                    
                                
                                
                                    State
                                    
                                        
                                        
                                        State cannot be more than 30 characters long
                                    
                                
                                
                                    Country
                                    
                                        

                                        
                                        Country cannot be more than 30 characters long
                                    
                                
                                
                                
                                    ZIP
                                    
                                        
                                        
                                        Zip code cannot be more than 30 characters long
                                    
                                
                            
                        
                        
                            
                                
                                    Estimate Details
                                
                                
                                    Lock box #
                                    
                                        
                                        
                                    
                                
                                
                                    Estimate Revenue
                                    
                                        
                                        
                                    
                                
                                
                                    Office
                                    
                                        
                                        
                                                                                            Head
                                                                                            Test office
                                                                                            Philadelphia-1#
                                                                                        
                                        

                                        
                                    
                                
                                
                                    Estimate description
                                    
                                        
                                        
                                    
                                
                            
                        

                        
                            
    var app = angular.module('popApp');
    app.controller('eavController', function($scope,$http,$q,$rootScope) {
        
        $scope.additional_field={};
        /*$scope.new_fields=[];*/
        $scope.new_field_checkbox_count=0;
        $scope.fields_type=['Text','Text Area','Radio Button','Check Box','Drop Down'];
        $scope.addNewField={};

        $scope.$on('get-custom-fields', function(event, form_name,additional_form) {
            // console.log(form_name);
            $scope.getAdditionalFields(form_name,additional_form);
        });
        $scope.$on('get-custom-fields-values', function(event, id,model_name,form_name) {
            $scope.additional_form_modal_name = model_name;
            $scope.getAdditionalFieldsValues(id,form_name);            
        });
        $scope.$on('save-custom-fields-value', function(event, id,entity_type,form_name,callback_fn) {
            
            if ($scope.additional_field) {
                $scope.additional_field.entity_id = id;
                $scope.additional_field.entity_type = entity_type;
                $scope.additional_field.form_name = form_name;   
                             
                $http.post('/eav/store-addition-fields', {'additional_field':$scope.additional_field}).then(function (res) {
                    if(typeof callback_fn !== 'undefined'){
                        callback_fn();
                    }
                });
            }
        });

        $scope.getAdditionalFields = function (form_name,additional_form) {
            $http.get('/eav/get-custom_fields?form_name='+form_name).then(function (res) {
                $scope.new_fields=[];
                $scope.new_fields = res.data;
                if(typeof additional_form !== 'undefined'){
                    $rootScope.step_form2 = true;
                //    console.log(&quot;in condition&quot;);
                }
                $scope.additional_form_name = form_name;                
            });
        };

        $scope.getAdditionalFieldsValues = function (id,form_name) {
            
            $http.get('/eav/custom_field_value?form_type_modal='+$scope.additional_form_modal_name+'&amp;id='+id+'&amp;form_name='+form_name).then(function (res) {
                
                if(res.data.custom_fields){
                    $scope.customFieldsArrays = res.data.custom_fields;
                    $scope.additional_field = {};
                    $scope.additional_field.update_id = id;
                    $scope.additional_field.form_name = form_name;
                    $scope.additional_field.entity_type = '&quot;' + form_name + 's&quot;';
                    $scope.additional_field.entity_id = id;
                    $scope.additional_field.additional_text = {};
                    $scope.additional_field.additional_text = {};
                    $scope.additional_field.additional_text.id = [];
                    $scope.additional_field.additional_textarea = {};
                    $scope.additional_field.additional_textarea.id = [];
                    $scope.additional_field.additional_radio = {};
                    $scope.additional_field.additional_radio.id = [];
                    $scope.additional_field.selectbox = {};
                    $scope.additional_field.selectbox.id = [];
                    $scope.additional_field.additional_checkbox = {};
                    $scope.additional_field.additional_checkbox.id = [];
                    var i;
                    angular.forEach($scope.customFieldsArrays,function(customFieldsArray){
                        if(customFieldsArray.additional_field != null) {

                            if (customFieldsArray.additional_field.field_type == 'Text') {
                                $scope.additional_field.additional_text[customFieldsArray.key] = customFieldsArray.value;
                                var key = customFieldsArray.key;
                                var obj = {};
                                obj[key] = customFieldsArray.id;
                                $scope.additional_field.additional_text.id.push(obj);
                                $scope.additional_field.additional_text.form_id = customFieldsArray.additional_field.id;
                            }
                            if (customFieldsArray.additional_field.field_type == 'Text Area') {
                                $scope.additional_field.additional_textarea[customFieldsArray.key] = customFieldsArray.value;
                                var key = customFieldsArray.key;
                                var obj = {};
                                obj[key] = customFieldsArray.id;
                                $scope.additional_field.additional_textarea.id.push(obj);
                                $scope.additional_field.additional_textarea.form_id = customFieldsArray.form_id;
                                $scope.additional_field.additional_textarea.entity_type = customFieldsArray.entity_type;
                                $scope.additional_field.additional_textarea.entity_id = customFieldsArray.entity_id;
                            }
                            if (customFieldsArray.additional_field.field_type == 'Radio Button') {
                                $scope.additional_field.additional_radio[customFieldsArray.key] = customFieldsArray.value;
                                $scope.additional_field.additional_radio.id.push(customFieldsArray.id);
                            }
                            if (customFieldsArray.additional_field.field_type == 'Check Box') {
                                if ($scope.additional_field.additional_checkbox &amp;&amp; !$scope.additional_field.additional_checkbox[customFieldsArray.key]) {
                                    $scope.additional_field.additional_checkbox[customFieldsArray.key] = {};
                                    i = 1;
                                    $scope.new_field_checkbox_count = i;
                                }
                                asyncGreet('#chkbox' + customFieldsArray.key + i + '', customFieldsArray.key, i).then(function () {
                                });
                                i++;
                                $scope.additional_field.additional_checkbox.id.push(customFieldsArray.id);
                            }
                            if (customFieldsArray.additional_field.field_type == 'Drop Down') {
                                $scope.additional_field.selectbox[customFieldsArray.key] = customFieldsArray.value;
                                $scope.additional_field.selectbox.id.push(customFieldsArray.id);
                            }
                        }
                    });
                }


                function asyncGreet(id,key,index) {
                    var deferred = $q.defer();
                    $(document).ready(function() {
                        $(id).prop('checked', true);
                        $scope.additional_field.additional_checkbox[key][index]=true;
                    });
                    return deferred.promise;
                }
            });
        };



        $scope.addMoreFields = function(){
            $scope.addition_submitted = false;
            $('#myNewFieldModal').modal('show');
        }

        $scope.create_new_fields = function(){
            /*if($scope.add_new_field_form.$invalid){
                $scope.addition_submitted = true;
                return false;
            }*/
            $scope.addition_submitted = false;
            $scope.validation_error = [];
            //validation for field type and label name and option
            if($scope.addNewField.field_type == null)
            {
                $scope.addition_submitted = true;
                $scope.validation_error['field_type'] = true;

            }
            else {
                switch ($scope.addNewField.field_type)
                {
                    case 'Radio Button':
                        if(!angular.isDefined($scope.addNewField.radio_button_option))
                        {
                            $scope.addition_submitted = true;
                            $scope.validation_error['radio_button'] = true;
                        }
                        break;
                    case 'Check Box':
                        if(!angular.isDefined($scope.addNewField.checkBox_option))
                        {
                            $scope.addition_submitted = true;
                            $scope.validation_error['check_box'] = true;
                        }
                        break;
                    case 'Drop Down':
                        if(!angular.isDefined($scope.addNewField.dropBox_option))
                        {
                            $scope.addition_submitted = true;
                            $scope.validation_error['drop_down'] = true;
                        }                        
                        break;
                    default:
                        break;

                }
            }
            if(!angular.isDefined($scope.addNewField.label_name) || $scope.addNewField.label_name.trim() === '')
            {
                $scope.addition_submitted = true;
                $scope.validation_error['label_name'] = true;

            }
            if($scope.addition_submitted)
            {
                return false;
            }
            $http.post('/eav/add-new-field/'+$scope.additional_form_name,{'new_fields':$scope.addNewField}).then(function (res) {
                $scope.addNewField={};
                $scope.new_fields.push(res.data[0]);
                $('#myNewFieldModal').modal('hide');
            });

        };


        $scope.getNumber = function(num) {
            return new Array(parseInt(num));
        }

        $scope.countCheck = function (checkfields) {
            if(checkfields){
                $scope.new_field_checkbox_count++;
            }else{
                $scope.new_field_checkbox_count--;
            }
        };

        $scope.removeField =  function(index,id,field_id)
        {
            swal({title: &quot;Are you sure?&quot;,
                    text: &quot;You want to delete this additional field&quot;,
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
                        $http.post('/eav/remove-additional-field',{'id':id,'field_id':field_id}).then( function(res)
                        {
                            if(res.data.status === 'success')
                            {
                                $scope.new_fields.splice(index,1);
                                swal(&quot;Deleted successfully&quot;, &quot;&quot;, &quot;success&quot;);
                            }
                        });
                    }else{
                        swal(&quot;Cancelled&quot;, &quot;&quot;, &quot;success&quot;);
                    }
                });
        }

        $scope.changed =  function(id)
        {
            $http.get('/eav/show-in-list/'+id).then( function(res)
            {
            });
        }

        $scope.closeModal =  function(e)
        {
            $('#myNewFieldModal').modal('toggle');
            e.preventDefault();
            e.stopPropagation();
        }
    });




    
        
            
            
                
                    ×
                    Add New Fields
                
                
                
                    
                        
                            
                                
                                    
                                        Field Type 
                                        
                                            Choose Type
                                            TextText AreaRadio ButtonCheck BoxDrop Down
                                        
                                        This Field is required
                                        
                                    
                                    
                                         required
                                        
                                         Is Required
                                    

                                
                                
                                    
                                        Label Name
                                        
                                        This Field is required                                        
                                        
                                    
                                    
                                    
                                    

                                
                                

                                    
                                        
                                        Show in Listing
                                    
                                    
                                        
                                        Show in Advance Search
                                    

                                


                            
                        
                    
                
                
                
                
                     Close
                     Add
                
                
            
        
    
    

        


        
        
                    
                
                     Add More Fields 
                
            
                
    






                        
                        
                             Save &amp; Next
                            
                        
                    
                
                
                
                
                
                


                
                Additional Info
                
                    

                        
                            Business Category
                            
                                
                                    SalesNew CategoryPlumberTest Business CategoryTest category searchElectricianCarpentryGardening

                                

                                
                                
                                    Create Business Category
                                
                                
                                    
                                        
                                            
                                                ×
                                                Add New Business Category
                                            
                                            
                                                
                                                    Contact Type
                                                    
                                                        
                                                    
                                                
                                            
                                            
                                                Close
                                                Save changes
                                            
                                        
                                    
                                
                            


                        
                        
                            Company Name
                            
                                
                                
                            
                        
                        
                            Job Title
                            
                                
                                
                            
                        

                        
                            Insurance Company Name
                            
                                
                                
                            
                        
                        
                            Claim #
                            
                                
                                
                            
                        
                        
                            Policy #
                            
                                
                                
                            
                        
                        
                            
    var app = angular.module('popApp');
    app.controller('eavController', function($scope,$http,$q,$rootScope) {
        
        $scope.additional_field={};
        /*$scope.new_fields=[];*/
        $scope.new_field_checkbox_count=0;
        $scope.fields_type=['Text','Text Area','Radio Button','Check Box','Drop Down'];
        $scope.addNewField={};

        $scope.$on('get-custom-fields', function(event, form_name,additional_form) {
            // console.log(form_name);
            $scope.getAdditionalFields(form_name,additional_form);
        });
        $scope.$on('get-custom-fields-values', function(event, id,model_name,form_name) {
            $scope.additional_form_modal_name = model_name;
            $scope.getAdditionalFieldsValues(id,form_name);            
        });
        $scope.$on('save-custom-fields-value', function(event, id,entity_type,form_name,callback_fn) {
            
            if ($scope.additional_field) {
                $scope.additional_field.entity_id = id;
                $scope.additional_field.entity_type = entity_type;
                $scope.additional_field.form_name = form_name;   
                             
                $http.post('/eav/store-addition-fields', {'additional_field':$scope.additional_field}).then(function (res) {
                    if(typeof callback_fn !== 'undefined'){
                        callback_fn();
                    }
                });
            }
        });

        $scope.getAdditionalFields = function (form_name,additional_form) {
            $http.get('/eav/get-custom_fields?form_name='+form_name).then(function (res) {
                $scope.new_fields=[];
                $scope.new_fields = res.data;
                if(typeof additional_form !== 'undefined'){
                    $rootScope.step_form2 = true;
                //    console.log(&quot;in condition&quot;);
                }
                $scope.additional_form_name = form_name;                
            });
        };

        $scope.getAdditionalFieldsValues = function (id,form_name) {
            
            $http.get('/eav/custom_field_value?form_type_modal='+$scope.additional_form_modal_name+'&amp;id='+id+'&amp;form_name='+form_name).then(function (res) {
                
                if(res.data.custom_fields){
                    $scope.customFieldsArrays = res.data.custom_fields;
                    $scope.additional_field = {};
                    $scope.additional_field.update_id = id;
                    $scope.additional_field.form_name = form_name;
                    $scope.additional_field.entity_type = '&quot;' + form_name + 's&quot;';
                    $scope.additional_field.entity_id = id;
                    $scope.additional_field.additional_text = {};
                    $scope.additional_field.additional_text = {};
                    $scope.additional_field.additional_text.id = [];
                    $scope.additional_field.additional_textarea = {};
                    $scope.additional_field.additional_textarea.id = [];
                    $scope.additional_field.additional_radio = {};
                    $scope.additional_field.additional_radio.id = [];
                    $scope.additional_field.selectbox = {};
                    $scope.additional_field.selectbox.id = [];
                    $scope.additional_field.additional_checkbox = {};
                    $scope.additional_field.additional_checkbox.id = [];
                    var i;
                    angular.forEach($scope.customFieldsArrays,function(customFieldsArray){
                        if(customFieldsArray.additional_field != null) {

                            if (customFieldsArray.additional_field.field_type == 'Text') {
                                $scope.additional_field.additional_text[customFieldsArray.key] = customFieldsArray.value;
                                var key = customFieldsArray.key;
                                var obj = {};
                                obj[key] = customFieldsArray.id;
                                $scope.additional_field.additional_text.id.push(obj);
                                $scope.additional_field.additional_text.form_id = customFieldsArray.additional_field.id;
                            }
                            if (customFieldsArray.additional_field.field_type == 'Text Area') {
                                $scope.additional_field.additional_textarea[customFieldsArray.key] = customFieldsArray.value;
                                var key = customFieldsArray.key;
                                var obj = {};
                                obj[key] = customFieldsArray.id;
                                $scope.additional_field.additional_textarea.id.push(obj);
                                $scope.additional_field.additional_textarea.form_id = customFieldsArray.form_id;
                                $scope.additional_field.additional_textarea.entity_type = customFieldsArray.entity_type;
                                $scope.additional_field.additional_textarea.entity_id = customFieldsArray.entity_id;
                            }
                            if (customFieldsArray.additional_field.field_type == 'Radio Button') {
                                $scope.additional_field.additional_radio[customFieldsArray.key] = customFieldsArray.value;
                                $scope.additional_field.additional_radio.id.push(customFieldsArray.id);
                            }
                            if (customFieldsArray.additional_field.field_type == 'Check Box') {
                                if ($scope.additional_field.additional_checkbox &amp;&amp; !$scope.additional_field.additional_checkbox[customFieldsArray.key]) {
                                    $scope.additional_field.additional_checkbox[customFieldsArray.key] = {};
                                    i = 1;
                                    $scope.new_field_checkbox_count = i;
                                }
                                asyncGreet('#chkbox' + customFieldsArray.key + i + '', customFieldsArray.key, i).then(function () {
                                });
                                i++;
                                $scope.additional_field.additional_checkbox.id.push(customFieldsArray.id);
                            }
                            if (customFieldsArray.additional_field.field_type == 'Drop Down') {
                                $scope.additional_field.selectbox[customFieldsArray.key] = customFieldsArray.value;
                                $scope.additional_field.selectbox.id.push(customFieldsArray.id);
                            }
                        }
                    });
                }


                function asyncGreet(id,key,index) {
                    var deferred = $q.defer();
                    $(document).ready(function() {
                        $(id).prop('checked', true);
                        $scope.additional_field.additional_checkbox[key][index]=true;
                    });
                    return deferred.promise;
                }
            });
        };



        $scope.addMoreFields = function(){
            $scope.addition_submitted = false;
            $('#myNewFieldModal').modal('show');
        }

        $scope.create_new_fields = function(){
            /*if($scope.add_new_field_form.$invalid){
                $scope.addition_submitted = true;
                return false;
            }*/
            $scope.addition_submitted = false;
            $scope.validation_error = [];
            //validation for field type and label name and option
            if($scope.addNewField.field_type == null)
            {
                $scope.addition_submitted = true;
                $scope.validation_error['field_type'] = true;

            }
            else {
                switch ($scope.addNewField.field_type)
                {
                    case 'Radio Button':
                        if(!angular.isDefined($scope.addNewField.radio_button_option))
                        {
                            $scope.addition_submitted = true;
                            $scope.validation_error['radio_button'] = true;
                        }
                        break;
                    case 'Check Box':
                        if(!angular.isDefined($scope.addNewField.checkBox_option))
                        {
                            $scope.addition_submitted = true;
                            $scope.validation_error['check_box'] = true;
                        }
                        break;
                    case 'Drop Down':
                        if(!angular.isDefined($scope.addNewField.dropBox_option))
                        {
                            $scope.addition_submitted = true;
                            $scope.validation_error['drop_down'] = true;
                        }                        
                        break;
                    default:
                        break;

                }
            }
            if(!angular.isDefined($scope.addNewField.label_name) || $scope.addNewField.label_name.trim() === '')
            {
                $scope.addition_submitted = true;
                $scope.validation_error['label_name'] = true;

            }
            if($scope.addition_submitted)
            {
                return false;
            }
            $http.post('/eav/add-new-field/'+$scope.additional_form_name,{'new_fields':$scope.addNewField}).then(function (res) {
                $scope.addNewField={};
                $scope.new_fields.push(res.data[0]);
                $('#myNewFieldModal').modal('hide');
            });

        };


        $scope.getNumber = function(num) {
            return new Array(parseInt(num));
        }

        $scope.countCheck = function (checkfields) {
            if(checkfields){
                $scope.new_field_checkbox_count++;
            }else{
                $scope.new_field_checkbox_count--;
            }
        };

        $scope.removeField =  function(index,id,field_id)
        {
            swal({title: &quot;Are you sure?&quot;,
                    text: &quot;You want to delete this additional field&quot;,
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
                        $http.post('/eav/remove-additional-field',{'id':id,'field_id':field_id}).then( function(res)
                        {
                            if(res.data.status === 'success')
                            {
                                $scope.new_fields.splice(index,1);
                                swal(&quot;Deleted successfully&quot;, &quot;&quot;, &quot;success&quot;);
                            }
                        });
                    }else{
                        swal(&quot;Cancelled&quot;, &quot;&quot;, &quot;success&quot;);
                    }
                });
        }

        $scope.changed =  function(id)
        {
            $http.get('/eav/show-in-list/'+id).then( function(res)
            {
            });
        }

        $scope.closeModal =  function(e)
        {
            $('#myNewFieldModal').modal('toggle');
            e.preventDefault();
            e.stopPropagation();
        }
    });




    
        
            
            
                
                    ×
                    Add New Fields
                
                
                
                    
                        
                            
                                
                                    
                                        Field Type 
                                        
                                            Choose Type
                                            TextText AreaRadio ButtonCheck BoxDrop Down
                                        
                                        This Field is required
                                        
                                    
                                    
                                         required
                                        
                                         Is Required
                                    

                                
                                
                                    
                                        Label Name
                                        
                                        This Field is required                                        
                                        
                                    
                                    
                                    
                                    

                                
                                

                                    
                                        
                                        Show in Listing
                                    
                                    
                                        
                                        Show in Advance Search
                                    

                                


                            
                        
                    
                
                
                
                
                     Close
                     Add
                
                
            
        
    
    

        


        
        
                    
                
                     Add More Fields 
                
            
                
    






                        
                        
                              Submit
                            
                            
                                 Previous
                            
                        
                    
                


            </value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;app&quot;)/div[@class=&quot;content-wrapper&quot;]/section[@class=&quot;right-content&quot;]/div[@class=&quot;panel panel-primary&quot;]/div[@class=&quot;panel-body ng-scope&quot;]/div[@class=&quot;bg-form-wiz&quot;]/div[@class=&quot;tab_container bg_tab_container&quot;]</value>
   </webElementProperties>
</WebElementEntity>
