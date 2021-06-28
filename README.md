amelius
=============



amelius is a simple rust app that allows me to use timewarrior and taskwarrior in such a way that I can have 90min deep work sessions 

during which: 
- task and timewarrior are triggered through task start <n> (where n is number of task to complete)
- all notifications are turned off 
- every second the time remaining until 90 minutes is displayed 

after which:
- i receive a matrix notification notifying me that the time work session has been completed
- notifications are turned back on 
- task stop <n>



## TODO
- [ ] trigger task start
- [ ] turn off notifications
- [ ] loop every second and display information (hint: use \r to print over the last line in stdout)
- [ ] setup matrix server
- [ ] send matrix notifications
- [ ] turn notifications back on
- [ ] stop the task
