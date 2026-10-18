"""Synthetic controller transport checks, not real-engine benchmark evidence."""
import argparse,base64,json,os,pathlib,platform,subprocess,sys
from experiment import Driver,cmd
import baseline_harness as b
p=argparse.ArgumentParser();p.add_argument('--base',type=pathlib.Path,required=True);a=p.parse_args()
a.base.mkdir(parents=True,exist_ok=False)
source=r'''use std::{io::{Read,Write},time::Duration};
fn main(){
    let mut input=String::new();std::io::stdin().read_to_string(&mut input).unwrap();
    if input.contains("partial-json") {
        std::io::stdout().write_all(b"{\"status\":").unwrap();std::io::stdout().flush().unwrap();
        std::fs::write(std::env::var("CAPTURE_READY").unwrap(),b"ready").unwrap();
        std::thread::sleep(Duration::from_secs(5));
    } else if input.contains("invalid-utf8") {
        std::io::stdout().write_all(&[0xff,0xc3]).unwrap();std::io::stdout().flush().unwrap();
    } else if !input.contains("empty-success") {
        println!("{}",r#"{"status":"ok","result":null}"#);
    }
}
'''
src=a.base/'synthetic-probe.rs';src.write_bytes(source.encode())
exe=a.base/('synthetic-probe.exe' if os.name=='nt' else 'synthetic-probe')
compile_command=['rustc','--edition=2021',str(src),'-o',str(exe)]
build=subprocess.run(compile_command,capture_output=True);(a.base/'build.stdout').write_bytes(build.stdout);(a.base/'build.stderr').write_bytes(build.stderr);assert build.returncode==0,build.stderr
d=Driver(a.base,exe,'capture-check','manifest');rows=[]
valid=d.call(cmd(a.base,'synthetic',scenario='valid-json'));assert valid==dict(status='ok',result=None)
rows.append(dict(scenario='valid-json',result='normal response preserved'))
for scenario,expected in [('partial-json',b'{"status":'),('invalid-utf8',b'\xff\xc3'),('empty-success',b'')]:
    command=cmd(a.base,'synthetic',scenario=scenario)
    try:
        if scenario=='partial-json':
            ready=a.base/'READY';pending=d.start(command,{'CAPTURE_READY':str(ready)})
            b.wait_path(ready,pending[0]);d.finish(pending,timeout=.2)
        else:d.call(command)
    except TimeoutError:assert scenario=='partial-json'
    except RuntimeError:assert scenario!='partial-json'
    else:raise AssertionError('malformed response was accepted')
    event=d.events[-1];raw=base64.b64decode(event['stdout_base64'],validate=True)
    assert raw==expected and event['stdout_bytes']==len(expected) and event['stdout_sha256']==b.sha(expected)
    assert event['output'] is None and event['stdout_parse_error']
    assert event['censored']==(scenario=='partial-json')
    rows.append(dict(scenario=scenario,result='lossless event retained before refusal',censored=event['censored'],stdout_bytes=len(raw),stdout_sha256=b.sha(raw)))
b.write(a.base/'capture-observations.json',dict(qualification='synthetic transport probe; no real engine correctness or latency claim',helper_sha256=b.sha(pathlib.Path(__file__).read_bytes()),probe_source_sha256=b.sha(src.read_bytes()),probe_binary_sha256=b.sha(exe.read_bytes()),compile_command=compile_command,platform=platform.platform(),python=sys.version,session=d.session,checks=rows))
print(json.dumps(rows))
