use std::fmt;

#[derive(Clone)]
pub struct Time {
    interval: String,
}

impl Time {
    pub fn new<T: Into<String>>(interval: T) -> Time
    {
        Time { interval: interval.into() }
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        let s = format!("[System[TimeCreated[timediff(@SystemTime) &lt;= '{}'", self.interval);
        write!(f, "{}", s)
        //write!(f, "*[System[TimeCreated[@SystemTime] &gt;= '{}']]", self.time)
    }
}
// ">*[System[TimeCreated[timediff(@SystemTime) &lt;= 3600000]]]</
// https://docs.microsoft.com/en-us/previous-versions/bb671200(v=vs.90)
/*
"<QueryList>" & _
"  <Query Id=""0"" Path=""Application"">" & _
"    <Select Path=""Application"">" & _
"        *[System[(Level &lt;= 3) and" & _
"        TimeCreated[timediff(@SystemTime) &lt;= 86400000]]]" & _
"    </Select>" & _
"    <Suppress Path=""Application"">" & _
"        *[System[(Level = 2)]]" & _
"    </Suppress>" & _
"    <Select Path=""System"">" & _
"        *[System[(Level=1  or Level=2 or Level=3) and" & _
"        TimeCreated[timediff(@SystemTime) &lt;= 86400000]]]" & _
"    </Select>" & _
"  </Query>" & _
"</QueryList>"
*/
/*
<QueryList>
  <Query Id="0" Path="Security">
    <Select Path="Security"> 
        *[EventData[Data[@Name='SubjectUserName'] and (Data='test')]] 
        and
        *[System[TimeCreated[@SystemTime] &gt;= '2015-01-24T00:00:000Z']]
        and
        *[System[TimeCreated[@SystemTime] &lt;= '2015-01-26T00:00:000Z']]
    </Select>
  </Query>
</QueryList>
*/