# Finite State model 
Nodes: op and dp 
Node States: "not set" and "set" - i.e. 
1. for a given node (i.e. text input), the user has identified either the port, specific region, or broad region (without loss of generality, let's call this a place). 
2. As the user types into a node, in real-time there is a drop down list that is generated from a list of results the best fits a place from the voyages table and ports table in the database. 
3. The list of possible places for op can be derived from the Voyage struct: 
```
#[serde(rename = "MAJBUYPT")]
pub origin_port: Option<i32>,
#[serde(rename = "MAJSELPT")]
pub destination_port: Option<i32>, // Specific Regions 
#[serde(rename = "MAJBYIMP")]
pub embark_region: Option<i32>,
#[serde(rename = "MJSELIMP")]
pub disembark_region: Option<i32>, // Broad Regions
#[serde(rename = "MAJBYIMP1")]
pub embark_broad_region: Option<i32>,
```
4. Similarly, the list of possible places for dp can be derived from the corresponding mjslptimp mjselimp mjselimp1 values in the Voyage Struct.
5. Once a user clicks on a place from the list, the place is set as the value in the text input (node) and the state of the node is changed from "not set" to "set". 
6. Once either op or dp is "set", then the list of searchable places must be restricted: - In the case that op is "set", then the possible places to search from in dp are the entries in the voyages database where the place selected (i.e. either the value corresponding mjbyptimp majbyimp majbyimp1) is set. The user sets the value of the place from the list and dp is "set". - In the case that dp is "set", then the possible places to search from in op are the entries in the voyages database where the place selected (i.e. either the value corresponding mjbyptimp majbyimp majbyimp1) is set. The user sets the value of the place from the list and op is "set". 
7. Once both nodes, op and dp, are "set", we generate a table showing the details that is human readable of the voyages with details of where they came from and where they went and how many people embarked and disembarked.
8. There is a "Reset" button that clears the values and the table to allow the use to go through this workflow again and make a new query.