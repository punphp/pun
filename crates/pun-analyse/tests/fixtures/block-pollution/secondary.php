<?php
namespace BlockPollution;

function matchSubjects(): void {
    $subject = 'entry';
    { $subject = 1; }
    \PHPStan\dumpType($subject);
    match ($subject) { 'entry' => 'string', 1 => 'integer' };
    match ($subject) { 1 => 'integer', default => 'string' };
}
