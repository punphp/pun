<?php

namespace SignatureArrowNever;

$h = fn (): never => throw new \Exception();
$i = fn (int $a): ?int => $a;
$j = fn (int $a) => $a;
