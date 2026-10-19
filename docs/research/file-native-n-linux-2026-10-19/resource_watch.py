"""Additional owned-root coexistence samples; absent earlier peak data stays absent."""
import json,time,os,resource,shutil,sys
from pathlib import Path
BASE=Path(__file__).resolve().parent
sys.path.insert(0,str(BASE/'source/docs/research/file-native-navigation-2026-10-19'))
from census import census
started=time.time();peak_bytes=peak_files=0;minimum_free=None;samples=0
with (BASE/'coexistence-samples.jsonl').open('w') as stream:
    while time.time()<1792453485:
        row=dict(epoch=time.time(),owned=census(BASE),free_bytes=shutil.disk_usage(BASE).free,watcher_peak_rss=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss*1024)
        stream.write(json.dumps(row)+'\n');stream.flush();samples+=1
        peak_bytes=max(peak_bytes,row['owned']['logical_bytes']);peak_files=max(peak_files,row['owned']['files']);minimum_free=min(minimum_free or row['free_bytes'],row['free_bytes'])
        if (BASE/'host-after-all.json').exists() or (BASE/'matrix-failed.json').exists():break
        time.sleep(.5)
(BASE/'coexistence.json').write_text(json.dumps(dict(start_epoch=started,end_epoch=time.time(),samples=samples,observed_peak_owned_bytes=peak_bytes,observed_peak_owned_files=peak_files,minimum_free_bytes=minimum_free,earlier_unsampled_peak='missing; earlier stage-end census retained',watcher_peak_rss=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss*1024),indent=2))
