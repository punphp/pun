<?php
// Independent worker fixture: copy complete named-object trees, removing request-local handles.
function readBytes(int $length): ?string {
    $value = '';
    while (strlen($value) < $length) {
        $part = fread(STDIN, $length - strlen($value));
        if ($part === false || $part === '') { return null; }
        $value .= $part;
    }
    return $value;
}
function take(string $data, int &$offset, int $length): string {
    $value = substr($data, $offset, $length);
    if (strlen($value) !== $length) { throw new RuntimeException('Truncated fixture request'); }
    $offset += $length;
    return $value;
}
function countField(string $data, int &$offset, int $width = 4): int {
    return unpack($width === 4 ? 'N' : 'n', take($data, $offset, $width))[1];
}
function bytesField(string $data, int &$offset): string {
    $length = countField($data, $offset);
    return pack('N', $length).take($data, $offset, $length);
}
function completeUnion(string $data, int &$offset, int $minor): string {
    take($data, $offset, 4); // The request-local type handle is absent from complete responses.
    $flags = take($data, $offset, 2);
    $count = countField($data, $offset);
    $value = $flags.pack('N', $count);
    for ($i = 0; $i < $count; $i++) { $value .= completeAtomic($data, $offset, $minor); }
    return $value;
}
function optionalMembers(string $data, int &$offset, int $minor, bool $unions): string {
    $present = take($data, $offset, 1);
    $value = $present;
    if ($present === "\1") {
        $count = countField($data, $offset);
        $value .= pack('N', $count);
        for ($i = 0; $i < $count; $i++) {
            $value .= $unions ? completeUnion($data, $offset, $minor) : completeAtomic($data, $offset, $minor);
        }
    }
    return $value;
}
function completeAtomic(string $data, int &$offset, int $minor): string {
    $tag = take($data, $offset, 2);
    if ($tag === "\1\7") {
        $literal = take($data, $offset, 1);
        return $tag.$literal.($literal === "\2" ? bytesField($data, $offset) : '').take($data, $offset, 2);
    }
    if ($tag !== "\4\2") { throw new RuntimeException('Expected a named-object snapshot'); }
    $value = $tag.bytesField($data, $offset).optionalMembers($data, $offset, $minor, true);
    $variances = take($data, $offset, 1);
    $value .= $variances;
    if ($variances === "\1") {
        $count = countField($data, $offset);
        $value .= pack('N', $count).take($data, $offset, $count);
    }
    $value .= take($data, $offset, 2).optionalMembers($data, $offset, $minor, false).take($data, $offset, 1);
    if ($minor >= 2) {
        $present = take($data, $offset, 1);
        $value .= $present;
        if ($present === "\1") { $value .= completeUnion($data, $offset, $minor); }
        $value .= take($data, $offset, 1); // Exact-class override, independent of the printed ID.
    }
    return $value;
}
function named(string $name, int $minor, ?string $excluded = null, ?array $intersections = null, bool $exact = false, ?array $parameters = null): string {
    $value = "\4\2".pack('N', strlen($name)).$name;
    $value .= $parameters === null ? "\0" : "\1".pack('N', count($parameters)).implode('', $parameters);
    $value .= "\0\0\0";
    $value .= $intersections === null ? "\0" : "\1".pack('N', count($intersections)).implode('', $intersections);
    $value .= "\0";
    if ($minor >= 2) {
        $value .= $excluded === null ? "\0" : "\1".$excluded;
        $value .= $exact ? "\1" : "\0";
    }
    return $value;
}
function unionOf(string ...$atomics): string { return "\0\0".pack('N', count($atomics)).implode('', $atomics); }

while (($header = readBytes(32)) !== null && ord($header[8]) !== 5) {
    $payload = readBytes(unpack('N', substr($header, 28, 4))[1]);
    $kind = unpack('n', substr($payload, 8, 2))[1];
    $minor = unpack('n', substr($payload, 6, 2))[1];
    if ($kind === 1) {
        $response = hex2bin($argv[1]);
    } else {
        $offset = 21;
        $providers = countField($payload, $offset, 2);
        take($payload, $offset, 2 * $providers);
        bytesField($payload, $offset); // Function name.
        take($payload, $offset, 8); // Invocation span.
        $arguments = countField($payload, $offset, 2);
        if ($arguments > 0) {
            if (take($payload, $offset, 1) === "\1") { bytesField($payload, $offset); }
            take($payload, $offset, 10); // Unpacking, placeholder, and argument span.
            bytesField($payload, $offset); // Source expression.
            if (take($payload, $offset, 1) !== "\1") { throw new RuntimeException('Missing argument type'); }
            bytesField($payload, $offset); // Human-readable ID is not used to reconstruct the type.
            $complete = completeUnion($payload, $offset, $minor);
        } else {
            $ac = named('A', $minor, intersections: [named('C', $minor)]);
            $bc = named('B', $minor, intersections: [named('C', $minor)]);
            $excluded = match ($argv[2]) {
                'nested' => unionOf(named('C', $minor, unionOf($ac))),
                'union' => unionOf($ac, $bc),
                'exact' => unionOf($ac),
                'deep' => unionOf(named('A', $minor)),
                default => null,
            };
            if ($argv[2] === 'deep') {
                for ($i = 0; $i < 90; $i++) { $excluded = unionOf(named('C', $minor, $excluded)); }
            }
            if ($argv[2] === 'many') { $excluded = "\0\0".pack('N', 65537); }
            if ($argv[2] === 'empty') { $excluded = "\0\0".pack('N', 0); }
            $complete = $argv[2] === 'exact'
                ? unionOf(named('WireBox', $minor, $excluded, exact: true, parameters: [unionOf("\1\7\0\0\3")]))
                : unionOf(named('C', $minor, $excluded));
        }
        $response = substr($payload, 0, 8).pack('nn', 0x8002, 0)."\1\24".$complete;
    }
    $header[8] = chr(2);
    fwrite(STDOUT, substr($header, 0, 28).pack('N', strlen($response)).$response);
    fflush(STDOUT);
}
