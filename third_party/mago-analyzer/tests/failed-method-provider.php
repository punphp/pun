<?php
// A real framed worker: callable signatures establish otherwise absent methods;
// the return provider supplies 73 independently of their native/magic declarations.
function bytes(string $value): string { return pack('N', strlen($value)).$value; }
function message(int $kind, string $body): string { return 'MANA'.pack('nnnn', 2, 1, $kind, 0).$body; }
function readBytes(int $length): string {
    $result = '';
    while (strlen($result) < $length) {
        $part = fread(STDIN, $length - strlen($result));
        if ($part === '' || $part === false) { exit(0); }
        $result .= $part;
    }
    return $result;
}
while (true) {
    $header = readBytes(32);
    if (ord($header[8]) === 5) { exit(0); }
    $length = unpack('N', substr($header, 28, 4))[1];
    $request = readBytes($length);
    $kind = unpack('n', substr($request, 8, 2))[1];
    if ($kind === 1) {
        $body = pack('N', 1).bytes('failed-methods').bytes('Failed methods').bytes('1.0.0')."\0";
        $body .= pack('N', 1).bytes('methods').bytes('Methods').bytes('Signature and return provider')."\1\0";
        $body .= pack('NN', 0, 0); // aliases and function providers
        $body .= pack('NnCN', 1, 0, 1, 2); // one callable-signature method provider, two targets
        $body .= bytes('*').bytes('provided').bytes('ProviderTarget').bytes('magic');
        $body .= str_repeat(pack('N', 0), 12); // remaining provider and hook lists
        $response = message(0x8001, $body);
    } elseif ($kind === 12) {
        $response = message(0x800C, "\1\0\1".pack('N', 0)); // handled, unnamed, named args, no parameters
    } elseif ($kind === 2) {
        $response = message(0x8002, "\1\x13".pack('J', 73)); // handled literal integer
    } else {
        throw new RuntimeException('Unexpected analyzer request '.$kind);
    }
    $reply = substr_replace($header, "\2\0", 8, 2);
    $reply = substr_replace($reply, pack('N', strlen($response)), 28, 4);
    fwrite(STDOUT, $reply.$response);
    fflush(STDOUT);
}
