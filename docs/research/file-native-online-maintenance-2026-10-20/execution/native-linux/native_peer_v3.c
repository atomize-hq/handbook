/* Harmless, bounded interface peer SOURCE ONLY. Not an engine or an oracle.
 * No build/run is authorized by this file. Real controls need parent admission.
 * Known generated control INPUT only; this deliberately is not a JSON parser.
 */
#define _POSIX_C_SOURCE 200809L
#include <errno.h>
#include <fcntl.h>
#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <unistd.h>

static int owned(const char *p) {
    const char *prefix = "/home/spenser/file-native-online-linux-20261020/fixtures/";
    return p && strncmp(p,prefix,strlen(prefix))==0 && !strstr(p,"..") && !strchr(p,'\\');
}
static int marker(const char *p, const char *bytes) {
    if (!owned(p)) return 120;
    int fd=open(p,O_WRONLY|O_CREAT|O_EXCL|O_NOFOLLOW,0600);
    if (fd<0) return 121;
    size_t n=strlen(bytes);
    int ok=write(fd,bytes,n)==(ssize_t)n;
    if (close(fd)) ok=0;
    return ok?0:122;
}
int main(void) {
    if (getenv("EXP_PEER_EARLY_JSON")) {
        close(0);
        const char *ready=getenv("EXP_PEER_EARLY_READY");
        if (marker(ready,"early-stdin-closed")) return 114;
        puts("{\"status\":\"ok\",\"result\":{\"peer_only\":true}}");
        fflush(stdout);
        struct timespec hold={0,250000000}; nanosleep(&hold,NULL);
        return 0;
    }
    if (getenv("EXP_PEER_PARTIAL_JSON")) {
        char first; if(read(0,&first,1)!=1) return 113;
        close(0);
        puts("{\"status\":\"ok\",\"result\":{\"peer_only\":true}}");
        fflush(stdout);
        struct timespec hold={0,250000000}; nanosleep(&hold,NULL);
        return 0;
    }
    char input[8193]; size_t used=0;
    while (used<sizeof(input)-1) {
        ssize_t n=read(0,input+used,sizeof(input)-1-used);
        if(n<0) { if(errno==EINTR) continue; return 123; }
        if(n==0) break;
        used+=(size_t)n;
    }
    if(used==sizeof(input)-1) return 124;
    input[used]=0;
    const char *seen=getenv("EXP_STDIN_MARKER");
    if(seen && marker(seen,"stdin-delivered")) return 125;
    if(strstr(input,"\"peer\":\"code2\"")) {
        puts("{\"status\":\"error\",\"error\":\"conflict:captured-base\"}"); return 2;
    }
    if(strstr(input,"\"peer\":\"exit86\"")) return 86;
    if(strstr(input,"\"peer\":\"output\"")) {
        for(int i=0;i<32768;i++) if(fputs("overflow-control-only\n",stdout)<0) return 119;
        return 0;
    }
    unsigned char *memory=NULL;
    if(strstr(input,"\"peer\":\"rss\"")) {
        size_t n=300u*1024u*1024u;
        memory=malloc(n); if(!memory) return 118;
        for(size_t i=0;i<n;i+=4096) ((volatile unsigned char *)memory)[i]=1;
    }
    const char *ready=getenv("EXP_READY"), *release=getenv("EXP_RELEASE");
    if(ready || release) {
        if(!owned(ready)||!owned(release)) return 117;
        if(marker(ready,"ready")) return 116;
        struct timespec pause={0,1000000};
        int done=0;
        for(int i=0;i<10000;i++) {
            if(access(release,F_OK)==0) {done=1;break;}
            nanosleep(&pause,NULL);
        }
        if(!done) return 115;
    }
    free(memory);
    puts("{\"status\":\"ok\",\"result\":{\"peer_only\":true}}");
    return 0;
}
