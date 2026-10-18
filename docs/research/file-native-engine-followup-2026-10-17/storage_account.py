"""Read-only file allocation split. Does not attribute filesystem directory/MFT bytes."""
import ctypes,os,pathlib
def category(parts):
    if parts[0]=='originals':return 'authoritative-payload'
    if parts[0]=='objects':return 'cow-metadata'
    if parts[0]=='divergence':return 'preserved-unadmitted-bytes'
    packet=(parts[0]=='packets' or 'retained-packet' in parts)
    generation=parts[0]=='generations' or 'generation-stage' in parts
    view=parts[0]=='view'
    if packet or generation or view:
        prefix='packet' if packet else 'readable-generation' if generation else 'live-view'
        if 'docs' in parts:return prefix+'-payload'
        if parts[-1]=='context.md':return prefix+'-assembly-output'
        if parts[-1]=='NAVIGATION.md':return prefix+'-navigation'
        if parts[-1] in ['provenance.json','MANIFEST.json']:return prefix+'-provenance-and-manifest'
        return prefix+'-staging-or-other'
    if parts[0]=='attempts':return 'candidate-state-and-request-metadata'
    if parts[0] in ['publications','receipts','fences','retired-attempts','retired-publications']:return 'publication-outcome-lifecycle-metadata'
    return 'store-control-and-other'
def detailed_inventory(root,selected=None,pending=None):
    root=pathlib.Path(root);groups={};liveness={};seen=set();aliases=0;directories=0
    selected=selected or set();pending=pending or set()
    if os.name=='nt':
        k=ctypes.WinDLL('kernel32',use_last_error=True)
        class Standard(ctypes.Structure):
            _fields_=[('allocation',ctypes.c_int64),('eof',ctypes.c_int64),('links',ctypes.c_uint32),('pending',ctypes.c_ubyte),('directory',ctypes.c_ubyte)]
        k.CreateFileW.argtypes=[ctypes.c_wchar_p,ctypes.c_uint32,ctypes.c_uint32,ctypes.c_void_p,ctypes.c_uint32,ctypes.c_uint32,ctypes.c_void_p];k.CreateFileW.restype=ctypes.c_void_p
        k.GetFileInformationByHandleEx.argtypes=[ctypes.c_void_p,ctypes.c_int,ctypes.c_void_p,ctypes.c_uint32];k.CloseHandle.argtypes=[ctypes.c_void_p]
    for directory,dirs,files in os.walk(root):
        directories+=len(dirs)
        for name in files:
            p=pathlib.Path(directory)/name;st=p.stat();key=category(p.relative_to(root).parts)
            if os.name=='nt':
                handle=k.CreateFileW(str(p),0,7,None,3,0,None)
                if handle in (None,ctypes.c_void_p(-1).value):raise ctypes.WinError(ctypes.get_last_error())
                info=Standard()
                try:
                    if not k.GetFileInformationByHandleEx(handle,1,ctypes.byref(info),ctypes.sizeof(info)):raise ctypes.WinError(ctypes.get_last_error())
                    allocated=info.allocation
                finally:k.CloseHandle(handle)
            else:
                identity=(st.st_dev,st.st_ino)
                if identity in seen:allocated=0;aliases+=1
                else:allocated=st.st_blocks*512;seen.add(identity)
            group=groups.setdefault(key,dict(files=0,logical_bytes=0,allocated_file_bytes=0));group['files']+=1;group['logical_bytes']+=st.st_size;group['allocated_file_bytes']+=allocated
            if key in ['authoritative-payload','cow-metadata']:
                rel=p.relative_to(root).as_posix();status='selected-retained' if rel in selected else 'eligible-sealed-candidate' if rel in pending else 'not-referenced-by-retained-or-eligible-sealed-roots'
                bucket=liveness.setdefault(key+'/'+status,dict(files=0,logical_bytes=0,allocated_file_bytes=0));bucket['files']+=1;bucket['logical_bytes']+=st.st_size;bucket['allocated_file_bytes']+=allocated
    return dict(directories=directories,groups=groups,immutable_liveness=liveness,hardlink_aliases_not_recounted=aliases,namespace_bytes='not attributed',total={k:sum(v[k] for v in groups.values()) for k in ['files','logical_bytes','allocated_file_bytes']})
