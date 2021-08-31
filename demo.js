import * as std from 'std';

print('hello')
print('args:',...args)

//write fs
let wf = std.open('demo.txt','w')
wf.puts('hello quickjs')
wf.close()

//read fs
let rf = std.open('demo.txt','r')
let r = rf.getline()
print(r)
rf.close()