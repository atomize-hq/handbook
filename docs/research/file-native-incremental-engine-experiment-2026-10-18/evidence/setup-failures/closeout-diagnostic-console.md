# Closing diagnostic console encoding

The first closing GitNexus wrapper saved its JSON receipt with the actual CLI
exit0, then its attempt to print decoded diagnostics failed because the Windows
CP1252 console cannot encode a thin-space character. This was a bookkeeping
display failure, not an engine invocation or a successful indexed review.
The read-only diagnostic was repeated with an ASCII-only summary. Its receipt
also retains exact stdout/stderr as base64, length and SHA-256; decoded display
text can contain replacement characters. Coverage remains unknown because the
index is stale/degraded. No engine result, censor, deadline or source changed.
