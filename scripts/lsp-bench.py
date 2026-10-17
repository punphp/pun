#!/usr/bin/env python3
"""Drive a PHP language server over stdio and time it (context/reference/lsp.md).

usage: lsp-bench.py ROOT FQCN -- SERVER [ARGS...]

Starts SERVER in ROOT, sends initialize/initialized, waits for the end of PHPantom's indexing progress, then opens
an unsaved ROOT/__lsp_bench.php that uses FQCN and asks for completion on `$x->` and the definition of the class.
Prints one JSON line: ready_s (initialize to indexing done), completion_ms, definition_ms, completion count and
first labels, the definition location, the server's peak RSS (VmHWM) and its RSS at the end.
"""
import json, os, subprocess, sys, time

root, fqcn = os.path.abspath(sys.argv[1]), sys.argv[2]
cmd = sys.argv[sys.argv.index("--") + 1:]
p = subprocess.Popen(cmd, cwd=root, stdin=subprocess.PIPE, stdout=subprocess.PIPE)
nid = 0


def send(msg):
    body = json.dumps(msg).encode()
    p.stdin.write(b"Content-Length: %d\r\n\r\n" % len(body) + body)
    p.stdin.flush()


def recv():
    n = 0
    while (line := p.stdout.readline().strip()):
        if line.startswith(b"Content-Length:"):
            n = int(line.split(b":")[1])
    m = json.loads(p.stdout.read(n))
    if "method" in m and "id" in m:  # server -> client request: progress tokens, registrations
        send({"jsonrpc": "2.0", "id": m["id"], "result": None})
    return m


def request(method, params):
    global nid
    nid += 1
    send({"jsonrpc": "2.0", "id": nid, "method": method, "params": params})
    while True:
        m = recv()
        if "method" not in m and m.get("id") == nid:
            return m


def mem(field):
    for line in open(f"/proc/{p.pid}/status"):
        if line.startswith(field):
            return int(line.split()[1]) // 1024


uri = lambda rel: "file://" + os.path.join(root, rel)
t0 = time.time()
request("initialize", {"processId": os.getpid(), "rootUri": uri(""), "capabilities": {
    "window": {"workDoneProgress": True}, "textDocument": {"completion": {"completionItem": {"snippetSupport": True}}}}})
send({"jsonrpc": "2.0", "method": "initialized", "params": {}})
indexing = None
while True:
    m = recv()
    if m.get("method") == "window/workDoneProgress/create" and "index" in str(m["params"]["token"]):
        indexing = m["params"]["token"]
    if m.get("method") == "$/progress" and m["params"]["token"] == indexing and m["params"]["value"]["kind"] == "end":
        break
ready = time.time() - t0

short = fqcn.rsplit("\\", 1)[-1]
text = f"<?php\n\nuse {fqcn};\n\n$x = new {short}();\n$x->\n"
send({"jsonrpc": "2.0", "method": "textDocument/didOpen", "params": {"textDocument": {
    "uri": uri("__lsp_bench.php"), "languageId": "php", "version": 1, "text": text}}})
t = time.time()
c = request("textDocument/completion", {"textDocument": {"uri": uri("__lsp_bench.php")}, "position": {"line": 5, "character": 4}})
completion_ms = (time.time() - t) * 1000
items = c.get("result") or []
items = items.get("items", []) if isinstance(items, dict) else items
t = time.time()
d = request("textDocument/definition", {"textDocument": {"uri": uri("__lsp_bench.php")}, "position": {"line": 4, "character": 10}})
definition_ms = (time.time() - t) * 1000
loc = d.get("result")
loc = loc[0] if isinstance(loc, list) and loc else loc
out = {"ready_s": round(ready, 2), "completion_ms": round(completion_ms, 1), "definition_ms": round(definition_ms, 1),
       "completions": len(items), "first": [i["label"] for i in items[:5]],
       "definition": loc and (loc.get("uri", loc.get("targetUri", "")).replace(uri(""), ""),
                              (loc.get("range") or loc.get("targetRange"))["start"]["line"]),
       "peak_rss_mb": mem("VmHWM"), "rss_mb": mem("VmRSS")}
request("shutdown", None)
send({"jsonrpc": "2.0", "method": "exit"})
print(json.dumps(out), flush=True)
p.stdin.close()  # tower-lsp's serve loop ends at EOF, not at `exit`
p.wait(timeout=30)
