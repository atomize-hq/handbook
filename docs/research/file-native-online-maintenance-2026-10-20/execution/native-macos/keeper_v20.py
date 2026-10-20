"""Independent deadline, exact-owned tree termination, durable UNKNOWN fence.
The token FD is passed after flock acquisition; this process keeps the same
open-file-description alive until two numeric closure observations succeed.
"""
import array
import datetime as dt
import fcntl
import json
import os
from pathlib import Path
import select
import signal
import socket
import subprocess
import time
from native_probe import identity, usage, group
from monitor_v20 import OwnedTree, Registry, same_identity, valid_identity
from admission_v20 import ROOT, strict_json, bounded_receipt

FENCE = ROOT/'HOST-FENCE.json'
MAX_MESSAGE = 2048


def send(sock, row, fd=None):
    data = json.dumps(row, separators=(',', ':')).encode()
    if len(data) > MAX_MESSAGE:
        raise RuntimeError('control-message-cap')
    anc = [] if fd is None else [(socket.SOL_SOCKET, socket.SCM_RIGHTS, array.array('i', fd if isinstance(fd,list) else [fd]))]
    sock.sendmsg([data], anc)


def receive(sock, timeout):
    if not select.select([sock], [], [], timeout)[0]:
        return None, []
    raw, anc, flags, _ = sock.recvmsg(MAX_MESSAGE+1, socket.CMSG_SPACE(16))
    if len(raw) > MAX_MESSAGE or flags & (socket.MSG_TRUNC|socket.MSG_CTRUNC):
        raise RuntimeError('control-message-truncated')
    if not raw:
        raise EOFError('control-channel-closed')
    fds = []
    for level, kind, data in anc:
        if level == socket.SOL_SOCKET and kind == socket.SCM_RIGHTS:
            a = array.array('i')
            a.frombytes(data[:len(data)//a.itemsize*a.itemsize])
            fds.extend(a)
    return strict_json(raw), fds


def exact_signal(expected, sig):
    before = identity(expected['pid'])
    if not same_identity(expected, before):
        return {'pid':expected['pid'], 'signal':int(sig), 'sent':False,
                'reason':'identity-not-proven', 'observed':before}
    try:
        os.kill(expected['pid'], sig)
        return {'pid':expected['pid'], 'start':expected['start'], 'signal':int(sig), 'sent':True, 'errno':0}
    except OSError as error:
        return {'pid':expected['pid'], 'signal':int(sig), 'sent':False, 'errno':error.errno}


def numeric_observation(anchor, known):
    pgids = sorted({anchor['pgid']}|{r['pgid'] for r in known.values()})
    groups = [group(pgid) for pgid in pgids]
    remaining = []
    errors = []
    for expected in known.values():
        row = identity(expected['pid'])
        if same_identity(expected, row):
            remaining.append({'identity':row, 'usage':usage(row['pid'])})
        elif row['returned_bytes'] == 0 and row['errno'] in (0, 3):
            pass
        elif valid_identity(row) and row.get('start') != expected['start']:
            # The old identity is gone; never signal or adopt the replacement.
            pass
        else:
            errors.append(row)
    empty = all(x['valid'] and x['members'] == [] for x in groups) and not remaining and not errors
    return {'monotonic':time.monotonic(), 'groups':groups, 'remaining':remaining,
            'unknown_errno_observations':errors, 'empty':empty}


def closure(anchor, known, seconds=2):
    rows = []
    total_observations = 0
    until = time.monotonic()+seconds
    while time.monotonic() < until:
        rows.append(numeric_observation(anchor, known))
        total_observations += 1
        if len(rows)>8:
            rows=rows[:2]+rows[-6:]
        if len(rows) >= 2 and all(x['empty'] for x in rows[-2:]):
            return {'proven':True, 'required_observations':2, 'observations':rows,'total_numeric_observations':total_observations}
        time.sleep(.02)
    return {'proven':False, 'required_observations':2, 'observations':rows,'total_numeric_observations':total_observations}


def terminate_tree(anchor, known):
    """Repeatedly STOP every observed owned member before any final KILL.
    Newly forked descendants are included until two identical frozen sets are
    observed. Uncertainty or a process-count cap cannot count as closure.
    """
    tree = OwnedTree(anchor)
    tree.known.update(known)
    steps = []
    stable = 0
    stop_attempted = set()
    previous = None
    until = time.monotonic()+1.5
    try:
        while time.monotonic() < until:
            try:
                result = tree.observe()
            except RuntimeError as error:
                steps.append({'query_error':repr(error)})
                # Stop still-proven owned identities and retry bounded native
                # enumeration; a transient exiting PID never authorizes release.
                for expected in tree.known.values():
                    if same_identity(expected,identity(expected['pid'])):
                        steps.append(exact_signal(expected,signal.SIGSTOP))
                time.sleep(.01)
                continue
            active = result['identities']
            frozen = True
            signature = sorted((x['pid'], tuple(x['start'])) for x in active)
            for row in active:
                if row.get('state') not in (4, 5):
                    key=(row['pid'],tuple(row['start']))
                    if key not in stop_attempted:
                        step = exact_signal(row, signal.SIGSTOP)
                        steps.append(step)
                        stop_attempted.add(key)
                    frozen = False
            stable = stable+1 if frozen and signature == previous else 0
            previous = signature
            if stable >= 2:
                break
            time.sleep(.01)
        if stable < 2:
            return {'status':'UNKNOWN_FREEZE', 'steps':steps, 'known':list(tree.known.values())}
        # The trusted anchor launches once. After the complete observed tree is
        # frozen, no member can fork. Kill descendants before the anchor; the
        # parent/launchd reaps them before the separate numeric closure proof.
        for row in sorted(active, key=lambda x: x['pid'] == anchor['pid']):
            steps.append(exact_signal(row, signal.SIGKILL))
        return {'status':'FROZEN_TREE_SIGNALLED', 'steps':steps, 'known':list(tree.known.values())}
    except BaseException as error:
        return {'status':'UNKNOWN_TERMINATION', 'error':repr(error), 'steps':steps,
                'known':list(tree.known.values())}


def durable_fence(nonce, parent, keeper, anchor, known, reason, socket_path):
    row = {'kind':'native-host-ownership-HOLD','nonce':nonce,'parent':parent,'keeper':keeper,
           'anchor':anchor,'known':list(known.values()),'reason':reason,
           'recovery_socket':str(socket_path),'recovery_channel':'bounded-regular-JSON-file','token_retained_by_keeper':True,
           'no_PID_only_resume':True,'utc':dt.datetime.now(dt.timezone.utc).isoformat()}
    if FENCE.exists():
        current = strict_json(FENCE.read_bytes())
        if current.get('nonce') != nonce:
            raise RuntimeError('existing-other-owner-fence')
    temporary = ROOT/('HOST-FENCE.'+nonce+'.tmp')
    with temporary.open('w') as out:
        json.dump(row, out, separators=(',', ':'))
        out.flush()
        os.fsync(out.fileno())
    os.replace(temporary, FENCE)
    directory = os.open(ROOT, os.O_RDONLY)
    try:
        os.fsync(directory)
    finally:
        os.close(directory)
    return row


def run(sock, nonce, parent, local_deadline, fixed_deadline, outroot, identity_budget=4096):
    keeper = identity(os.getpid())
    token_fd = None
    anchor = None
    anchor_process = None
    known = {}
    last_heartbeat = time.monotonic()
    events = []
    reason = None
    record = {'kind':'independent-native-keeper','nonce':nonce,'identity':keeper,
              'fixed_cutoff_epoch':fixed_deadline,'local_deadline_monotonic':local_deadline,
              'token_received':False,'state':'SETUP','identity_budget':identity_budget}
    recovery_path = ROOT/('keeper-'+nonce+'.release.json')
    registry = Registry(nonce)
    try:
        send(sock, {'ready':nonce,'keeper':keeper,'recovery_socket':str(recovery_path)})
        while True:
            now = time.monotonic()
            observed = identity(parent['pid'])
            parent_live = same_identity(parent, observed) and observed.get('state') != 5
            if not parent_live:
                reason = 'supervisor-death-or-identity-unknown'
                break
            if time.time() >= fixed_deadline:
                reason = 'fixed-cutoff'
                break
            if now >= local_deadline:
                reason = 'local-cell-deadline'
                break
            if now-last_heartbeat > .04:
                reason = 'RSS-heartbeat-gap-over-40ms'
                break
            if anchor_process is not None:
                anchor_process.poll()
            row, fds = receive(sock, .005)
            if row is None:
                continue
            if row.get('nonce') != nonce:
                raise RuntimeError('keeper-nonce-mismatch')
            if 'launch' in row:
                if token_fd is None or anchor_process is not None or len(fds) != 3:
                    raise RuntimeError('anchor-launch-ownership-shape')
                details = row['launch']
                child_control,output,error = fds
                argv = [details['python'],'-B',details['guard'],'--anchor',str(child_control),nonce,
                        details['cwd'],details['input'],str(token_fd),json.dumps(keeper,separators=(',',':')),*details['command']]
                anchor_process = subprocess.Popen(argv,pass_fds=(child_control,token_fd),start_new_session=True,
                                                   stdout=output,stderr=error,env=dict(os.environ,**details['env_overrides']))
                for fd in fds:
                    os.close(fd)
                fds = []
                anchor = identity(anchor_process.pid)
                if not valid_identity(anchor) or anchor['ppid'] != os.getpid() or anchor['pgid'] != anchor['pid']:
                    raise RuntimeError('keeper-created-anchor-identity')
                known[anchor['pid']] = anchor
                send(sock,{'launched':nonce,'anchor':anchor})
            if 'token' in row:
                if len(fds) != 1 or token_fd is not None:
                    raise RuntimeError('token-fd-transfer-shape')
                token_fd = fds[0]
                record['token_received'] = True
                # Same open-file-description retains the already-held flock.
                fcntl.flock(token_fd, fcntl.LOCK_EX|fcntl.LOCK_NB)
                send(sock, {'token_ack':nonce})
            elif fds and 'launch' not in row:
                raise RuntimeError('unexpected-control-fd')
            if 'anchor' in row:
                anchor = row['anchor']
                if not same_identity(anchor, identity(anchor['pid'])):
                    raise RuntimeError('anchor-not-owned')
                known[anchor['pid']] = anchor
                send(sock, {'anchor_ack':nonce})
            if 'known' in row:
                for expected in row['known']:
                    known[expected['pid']] = expected
                if len(known) > identity_budget:
                    raise RuntimeError('keeper-known-process-cap')
            if 'sample_completed' in row:
                # The monotonic timestamp is produced by a successful sampler.
                stamp = row['sample_completed']
                if stamp > now+.01 or stamp < last_heartbeat:
                    raise RuntimeError('invalid-RSS-heartbeat')
                last_heartbeat = stamp
            if row.get('failure'):
                reason = row['failure']
                break
            if row.get('stop'):
                if anchor:
                    proof = closure(anchor, known)
                    events.append({'closure':proof})
                    if not proof['proven']:
                        reason = 'UNKNOWN-on-normal-stop'
                        break
                record['state'] = 'CLOSED'
                send(sock, {'closed':nonce})
                return
            if row.get('hold'):
                reason = 'parent-UNKNOWN-closure'
                break
    except BaseException as error:
        reason = 'keeper-error:'+repr(error)
    finally:
        if reason:
            record['reason'] = reason
            if anchor:
                events.append({'termination':terminate_tree(anchor, known)})
                for row in events[-1]['termination']['known']:
                    known[row['pid']] = row
                if anchor_process is not None:
                    try:
                        anchor_process.wait(timeout=.3)
                    except subprocess.TimeoutExpired:
                        pass
                # Alert the facade; the keeper owns/reaps its anchor.
                step = exact_signal(parent, signal.SIGUSR1)
                events.append({'notify_parent':step})
                if anchor_process is not None:
                    try:
                        anchor_process.wait(timeout=.3)
                    except subprocess.TimeoutExpired:
                        pass
                proof = closure(anchor, known)
                events.append({'closure':proof})
                force_hold = reason == 'parent-UNKNOWN-closure' or (FENCE.exists() and strict_json(FENCE.read_bytes()).get('nonce') == nonce)
            else:
                proof = {'proven':True,'no_workload_created':True}
                force_hold = False
                events.append({'notify_parent':exact_signal(parent, signal.SIGUSR1)})
            if anchor and (not proof['proven'] or force_hold):
                record['state'] = 'HOLD'
                record['fence'] = durable_fence(nonce,parent,keeper,anchor,known,reason,recovery_path)
                # A keeper HOLD has no new workload admission. It retains the
                # token and publishes its own positive RSS plus a still-live
                # original supervisor until explicit reviewed recovery.
                while True:
                    helpers = []
                    for expected in (keeper,parent):
                        observed = identity(expected['pid'])
                        if same_identity(expected, observed) and observed.get('state') != 5:
                            memory = usage(expected['pid'])
                            if memory['return'] == 0 and memory['rss_bytes'] is not None:
                                helpers.append({'identity':observed,'usage':memory})
                    try:
                        registry.publish(helpers, 'HOLD-token')
                    except BaseException as error:
                        record['hold_accounting_error'] = repr(error)
                    record['events'] = events[-8:]
                    bounded_receipt(outroot/'keeper.json',record)
                    time.sleep(.02)
                    if anchor_process is not None:
                        anchor_process.poll()
                    if recovery_path.exists():
                        if recovery_path.is_symlink() or recovery_path.stat().st_size > 4096:
                            record['release_request_error'] = 'request-path-or-size'
                            continue
                        request = strict_json(recovery_path.read_bytes())
                        terminate_request = request == {'nonce':nonce,'reviewed_terminate_owned_then_release':True}
                        release_request = request == {'nonce':nonce,'reviewed_release_after_positive_closure':True}
                        if terminate_request or release_request:
                            if terminate_request:
                                recovery_termination = terminate_tree(anchor,known)
                                events.append({'explicit_owned_recovery_termination':recovery_termination})
                                known.update({x['pid']:x for x in recovery_termination['known']})
                                if anchor_process is not None:
                                    try:
                                        anchor_process.wait(timeout=.3)
                                    except subprocess.TimeoutExpired:
                                        pass
                            checked = closure(anchor, known)
                            events.append({'explicit_recovery_closure':checked})
                            if checked['proven'] and strict_json(FENCE.read_bytes()).get('nonce') == nonce:
                                FENCE.unlink()
                                record['state'] = 'CLOSED_AFTER_EXPLICIT_REVIEW'
                                break
            else:
                record['state'] = 'CLOSED_AFTER_CENSOR'
        record['events'] = events
        bounded_receipt(outroot/'keeper.json',record)
        registry.update(remove=True)
        if token_fd is not None:
            os.close(token_fd)
        recovery_path.unlink(missing_ok=True)
        sock.close()
