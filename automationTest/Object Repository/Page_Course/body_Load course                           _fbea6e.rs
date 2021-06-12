<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>body_Load course                           _fbea6e</name>
   <tag></tag>
   <elementGuidId>f4654300-0bc8-4d7f-9b90-6635c23d344d</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//body</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>body</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>body</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
    
    
      
        
          Load course
        
      
    

    
      
        
          
            Course List from Academy
            
              
              Add Course
            
          
          
            Show 102550100 entriesSearch:
              
                #TitlePriceDiscountShort DescriptionLast Update
              
              
                  1
                  Lập trình javascript từ cơ bản tới nâng cao
                  599
                  0
                  Javascirpt cơ bản cho mấy thằng mới biết code
                  02/06/2021 13:06:12
                
                  2
                  Khóa học lập trình web fullstack cho người mới bắt đầu
                  799
                  0
                  
                  31/05/2021 02:46:03
                
                  3
                  Lập trình Front-end với ReactJS
                  600
                  0
                  
                  31/05/2021 02:46:59
                
                  4
                  Lập tình android từ cơ bản tới nâng cao
                  599
                  0
                  
                  31/05/2021 02:48:13
                
                  5
                  Khóa học Unity cho người mới bắt đầu
                  600
                  0
                  
                  31/05/2021 02:49:53
                
                  15
                  new film
                  20000
                  0
                  
                  10/06/2021 14:32:45
                
                  25
                  new course
                  30000
                  0
                  
                  10/06/2021 14:33:27
                
                  35
                  Học lập trình Python cơ bản
                  500000
                  0
                  
                  12/06/2021 05:08:47
                
                  45
                  Học lập trình Python cơ bản
                  500000
                  0
                  
                  12/06/2021 05:08:48
                
                  55
                  Học lập trình Python cơ bản
                  500000
                  0
                  
                  12/06/2021 05:10:58
                
            Showing 1 to 10 of 15 entriesPrevious12Next
          
          
            Footer
          
        
      
    

    
    
    
    
    

    var table;
    table = $(&quot;#resultTable&quot;).DataTable();
    $('#btnLoadCourses').on('click', function () {
      $.ajax({
        url: 'http://localhost:3000/course/all',
        type: 'GET'
      }).done(function (data) {
        var html = ``;
        for (f of data) {
          const tr = `&lt;tr>
                  &lt;th scope=&quot;row&quot;>${f.id}&lt;/th>
                  &lt;td>${f.title}&lt;/td>
                  &lt;td>${f.price || 0}&lt;/td>
                  &lt;td>${f.discount || 0}&lt;/td>
                  &lt;td>${f.short_description || &quot;&quot;}&lt;/td>
                  &lt;td>${f.last_update}&lt;/td>
                &lt;/tr>`;
          html += tr;
        }

        table.destroy();
        $(&quot;#resultTable tbody&quot;).empty();
        $(&quot;#resultTable tbody&quot;).html(html);
        table = $(&quot;#resultTable&quot;).DataTable();
      })
    });





/html[1]/body[1]</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]/body[1]</value>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//body</value>
   </webElementXpaths>
</WebElementEntity>
