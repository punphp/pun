<?php declare(strict_types=1);

namespace ParamsBranchContracts;

function constructorSelection(): void
{
    new \DatePeriod(new \DateTimeImmutable(), new \DateInterval('P1D'), 2);
    new class(new \DateTimeImmutable(), new \DateInterval('P1D'), 2) extends \DatePeriod {};
}

class UserQueue
{
    public function setExtractFlags(int $flags): void {}
}

function combinedMethods(\SplPriorityQueue|UserQueue $receiver): void
{
    $receiver->setExtractFlags(JSON_HEX_TAG);
    $receiver->setExtractFlags();
    $receiver->setExtractFlags(unknown: 1);
}
