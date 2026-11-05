<?php

namespace DateFormatArrayFill;

use function PHPStan\dumpType;

/**
 * @param int<-5, -1> $negative
 * @param int<0, 3> $range
 * @param positive-int $positive
 * @param non-empty-string $nonEmpty
 * @param 'Y'|'D' $either
 * @param T $template
 * @param U $negativeTemplate
 * @template T of int<1, 5>
 * @template U of int<-5, -1>
 */
function arrays(int $n, int $negative, int $range, int $positive, string $s, string $nonEmpty, string $either, int $template, int $negativeTemplate, mixed $m): void
{
    dumpType(array_fill(0, 3, 'x'));
    dumpType(array_fill(5, 2, 1));
    dumpType(array_fill(-3, 3, true));
    dumpType(array_fill(PHP_INT_MAX, 1, 'a'));
    dumpType(array_fill(PHP_INT_MAX, 2, 'a'));
    dumpType(array_fill(0, 101, 'x'));
    dumpType(array_fill(0, $n, 'x'));
    dumpType(array_fill(3, $positive, 'x'));
    dumpType(array_fill($n, $range, 'x'));
    dumpType(array_fill(0, $template, 'x'));
    dumpType(array_fill(0, $negative, 'x'));
    dumpType(array_fill(0, -1, 'x'));
    dumpType(array_fill(0, $s, 'x'));
    dumpType(array_fill(0, $negativeTemplate, 'x'));
    dumpType(array_fill(0, $m, 'x'));
    dumpType(array_fill(0, 2));
}

function dates(\DateTime $d, \DateTimeImmutable $i, \DateTimeInterface $t, string $s, string $nonEmpty, string $either): void
{
    dumpType(date_format($d, 'd'));
    dumpType(date_format($d, 'j'));
    dumpType(date_format($d, 'N'));
    dumpType(date_format($d, 'w'));
    dumpType(date_format($d, 'm'));
    dumpType(date_format($d, 'n'));
    dumpType(date_format($d, 't'));
    dumpType(date_format($d, 'L'));
    dumpType(date_format($d, 'g'));
    dumpType(date_format($d, 'G'));
    dumpType(date_format($d, 'h'));
    dumpType(date_format($d, 'H'));
    dumpType(date_format($d, 'I'));
    dumpType(date_format($d, 'u'));
    dumpType(date_format($d, 'v'));
    dumpType(date_format($d, 'dDjlNSwzWFmMntLoXxYyaABgGhHisuveIOPpTZcrU'));
    dumpType(date_format($d, 'Ymd'));
    dumpType(date_format($d, 'zLGIZUoxXyBghHisuvWdmjnNwt'));
    dumpType(date_format($d, '\Y\-\m'));
    dumpType(date_format($d, 'Q'));
    dumpType(date_format($d, '\\'));
    dumpType(date_format($d, ''));
    dumpType(date_format($d, $s));
    dumpType(date_format($d, $nonEmpty));
    dumpType(date_format($d, $either));
    dumpType(date_format($d));
    dumpType($d->format('Y'));
    dumpType($i->format('D'));
    dumpType($t->format('H:i'));
    dumpType($i->format($s));
    dumpType($d->format());
}

class NotADate
{
    public function format(string $f): string
    {
        return $f;
    }
}

function other(NotADate $n): void
{
    dumpType($n->format('Y'));
}
